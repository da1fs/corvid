//! GitHub Releases auto-updater (the middle tier).
//!
//! The aliases live in three tiers:
//!   1. LIGHT  - the drawer used to hit the GitHub Releases API directly and
//!      show a human-clickable "Download" link. Historical, now replaced.
//!   2. MIDDLE - THIS MODULE: the host checks
//!      `https://api.github.com/repos/da1fs/corvid/releases/latest`
//!      (startup + every 5 hours + on drawer demand), compares the tag with
//!      the running version and, when a newer Windows installer asset
//!      exists, downloads it into `<data>/updates/` automatically with
//!      progress on the event bus. A one-click install quits the app, runs
//!      the NSIS installer silently and relaunches Corvid.
//!   3. DEEP   - tauri-plugin-updater with release signing keys: still an
//!      option later if this middle tier ever needs signed manifests.
//!
//! Doctrine ("never break"): every failure collapses into a `failed` phase
//! string that the drawer prints; the running app is never touched until the
//! user (or an agent) explicitly asks for the install.
//!
//! Update POLICY (prefs.update_mode, the user's standing choice):
//!   "ask"  (default) - downloads run automatically, but the shell pops the
//!     center card on "ready" and the user decides per release (the
//!     "Don't show this message again + Update now" combo stores "auto";
//!     "...+ No thanks" stores "off"). Dismissed tags never re-ask until a
//!     newer release arrives.
//!   "auto" - download, verify, install and relaunch with NO prompts (the
//!     standing opt-in the card created counts as the explicit ask).
//!   "off"  - NOTHING happens on its own: silent checks keep the drawer
//!     honest, every download/install comes from the drawer's buttons.
//!     "Never automatically" includes the network fetches: pure compliance.
//!
//! Endpoints (management tier, no API key needed, shell same-origin):
//!   GET  /api/update            - status snapshot (see status_json)
//!   POST /api/update/check      - force a GitHub check now (backgrounded)
//!   POST /api/update/download   - forces the download when a newer asset exists
//!   POST /api/update/install    - install the downloaded installer + restart

use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

const RELEASE_LATEST_URL: &str = "https://api.github.com/repos/da1fs/corvid/releases/latest";
const UPDATER_UA: &str = "zapdude-updater";
const CHECK_EVERY: Duration = Duration::from_secs(5 * 60 * 60);
const FIRST_CHECK_DELAY: Duration = Duration::from_secs(180);

/// Phases the status can hold. Serialized into short strings for the API.
#[derive(Debug, Clone)]
enum Phase {
    /// Nothing checked yet (or nothing newer found without details).
    Idle,
    /// A check just started (so the drawer can show activity).
    Checking,
    /// The wrapper batch is running; the app exits within a second.
    Installing { tag: String },
    /// Checked and the running build IS the newest release.
    Fresh { latest: String },
    /// A newer release exists. `asset` carries the Windows installer asset
    /// when the release ships one (name, browser_download_url, size).
    Newer {
        tag: String,
        notes_url: String,
        asset: Option<(String, String, u64)>,
    },
    /// Downloading the installer; received bytes / optional total.
    Downloading {
        tag: String,
        received: u64,
        total: Option<u64>,
    },
    /// Installer downloaded and size/EXE-header verified.
    Ready { tag: String, path: String },
    /// Something failed at `stage`; the message is meant for humans.
    Failed { stage: &'static str, error: String },
}

/// The human-facing phase names the drawer and the update card consume.
fn phase_name(phase: &Phase) -> &'static str {
    match phase {
        Phase::Idle => "idle",
        Phase::Checking => "checking",
        Phase::Installing { .. } => "installing",
        Phase::Fresh { .. } => "fresh",
        Phase::Newer { .. } => "newer",
        Phase::Downloading { .. } => "downloading",
        Phase::Ready { .. } => "ready",
        Phase::Failed { .. } => "failed",
    }
}

struct UpdateState {
    phase: Mutex<Option<Phase>>,
    last_check: Mutex<Option<Instant>>,
}

static UPDATE: OnceLock<UpdateState> = OnceLock::new();
static CHECK_IN_FLIGHT: AtomicBool = AtomicBool::new(false);
static DOWNLOAD_IN_FLIGHT: AtomicBool = AtomicBool::new(false);
static INSTALL_IN_FLIGHT: AtomicBool = AtomicBool::new(false);

fn state() -> &'static UpdateState {
    UPDATE.get_or_init(|| UpdateState {
        phase: Mutex::new(None),
        last_check: Mutex::new(None),
    })
}

fn with_phase<T>(reader: impl FnOnce(&Option<Phase>) -> T) -> T {
    let guard = state()
        .phase
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    reader(&guard)
}

fn set_phase(next: Phase) {
    {
        let mut guard = state()
            .phase
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *guard = Some(next.clone());
    }
    // The shell reacts: GET /api/update polls (drawer + card) read the
    // phase, and the center update card appears on "ready" (ask policy,
    // once per tag) and closes when the phase moves on.
    crate::window::nudge_poll();
}

/// Current app version straight from the build (Cargo.toml == tauri.conf).
pub fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// "v1.4.0" -> Some(vec![1, 4, 0]). Tolerant to a leading v / V and to any
/// trailing suffix (beta tags simply compare on their numeric prefix, the
/// honest fallback portrait used since 1.2.0).
fn parse_version(text: &str) -> Option<Vec<u32>> {
    let cleaned = text.trim().trim_start_matches(['v', 'V']);
    let head = cleaned
        .split(|c: char| !(c.is_ascii_digit() || c == '.'))
        .next()
        .unwrap_or("");
    let parts: Vec<u32> = head
        .split('.')
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().ok())
        .collect::<Option<Vec<u32>>>()?;
    (!parts.is_empty()).then_some(parts)
}

/// Is `latest` strictly newer than `current`? Missing segments count as 0
/// (semver: "1.4" == "1.4.0"), so tag style drift never forces a pointless
/// reinstall of identical bits.
fn version_newer(latest: &[u32], current: &[u32]) -> bool {
    let width = latest.len().max(current.len());
    for index in 0..width {
        let lhs = latest.get(index).copied().unwrap_or(0);
        let rhs = current.get(index).copied().unwrap_or(0);
        if lhs != rhs {
            return lhs > rhs;
        }
    }
    false
}

/// A parsed "latest release" payload from GitHub.
#[derive(Debug, Clone)]
struct LatestInfo {
    /// Numeric tag ("1.4.0", no v prefix).
    tag: String,
    /// Absolute HTTPS link the drawer hands to humans.
    notes_url: String,
    /// (asset name, browser_download_url, asset bytes) for the Windows NSIS
    /// setup when the release ships one.
    asset: Option<(String, String, u64)>,
}

/// Pull the latest release from GitHub (blocking; run inside a thread).
fn fetch_latest() -> Result<LatestInfo, String> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(25)))
        .build()
        .into();
    let response = agent
        .get(RELEASE_LATEST_URL)
        .header("User-Agent", UPDATER_UA)
        .header("Accept", "application/vnd.github+json")
        .call()
        .map_err(|err| format!("request failed: {err}"))?;
    if response.status().as_u16() == 403 {
        // GitHub anonymous rate limit (60/h per IP): honest error, retryable.
        return Err("GitHub rate limit (403) - try again later".to_string());
    }
    if !response.status().is_success() {
        return Err(format!(
            "GitHub returned HTTP {}",
            response.status().as_u16()
        ));
    }
    let text = response
        .into_body()
        .read_to_string()
        .map_err(|err| format!("unreadable response: {err}"))?;
    let value: Value =
        serde_json::from_str(&text).map_err(|err| format!("bad JSON payload: {err}"))?;
    let raw_tag = value
        .get("tag_name")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .trim_start_matches(['v', 'V'])
        .to_string();
    let notes_url = value.get("html_url").and_then(Value::as_str).unwrap_or("");
    if !tag_shape_valid(&raw_tag) {
        return Err(format!("unexpected tag shape: {raw_tag:?}"));
    }
    if notes_url.is_empty() || !notes_url.starts_with("https://github.com/") {
        return Err("release payload carries no usable html_url".to_string());
    }

    // Windows NSIS asset: exact version match wins, the generic
    // v-prefixed spelling second. BOTH product spellings are accepted on
    // purpose (Zapdude_* assets today, Corvid_* from the rename cut) so an
    // installed 1.4.x client can hop onto the Corvid feed without losing
    // the updater.
    let mut asset: Option<(String, String, u64)> = None;
    let mut asset_priority: u8 = u8::MAX;
    if let Some(assets) = value.get("assets").and_then(Value::as_array) {
        for entry in assets {
            let name = entry.get("name").and_then(Value::as_str).unwrap_or("");
            let url = entry
                .get("browser_download_url")
                .and_then(Value::as_str)
                .unwrap_or("");
            if name.is_empty() || !url.starts_with("https://github.com/") {
                continue;
            }
            if let Some(priority) = installer_asset_priority(name, &raw_tag) {
                if priority < asset_priority {
                    asset_priority = priority;
                    asset = Some((
                        name.to_string(),
                        url.to_string(),
                        entry.get("size").and_then(Value::as_u64).unwrap_or(0),
                    ));
                }
            }
        }
    }
    Ok(LatestInfo {
        tag: raw_tag,
        notes_url: notes_url.to_string(),
        asset,
    })
}

/// Does `name` look like our Windows installer, and how much do we trust
/// it? Some(0): the exact `Zapdude_<tag>_x64-setup.exe` /
/// `Corvid_<tag>_x64-setup.exe` CI archive; Some(1): the generic product
/// v-prefixed spelling; None: not ours. The Corvid spellings exist for the
/// planned rename cut (docs/corvid-rename-checklist.md).
fn installer_asset_priority(name: &str, tag: &str) -> Option<u8> {
    for product in ["Zapdude", "Corvid"] {
        if name == format!("{product}_{tag}_x64-setup.exe") {
            return Some(0);
        }
    }
    let generic_v = (name.starts_with("Zapdude_v") || name.starts_with("Corvid_v"))
        && name.ends_with("x64-setup.exe");
    generic_v.then_some(1)
}

fn tag_shape_valid(tag: &str) -> bool {
    let parts: Vec<&str> = tag.split('.').collect();
    parts.len() >= 2
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
}

/// Run one GitHub check; when a NEWER version exists, lands in `newer` and
/// auto-starts the download (the auto behavior the user asked for). Safe to
/// call from any thread; concurrent checks collapse into one.
pub fn request_check(source: &'static str) -> bool {
    if CHECK_IN_FLIGHT
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return false;
    }
    eprintln!("zapdude: updater check ({source})");
    let spawn_result = std::thread::Builder::new()
        .name("zapdude-update-check".to_string())
        .spawn(run_check_until_download);
    if let Err(err) = spawn_result {
        CHECK_IN_FLIGHT.store(false, Ordering::SeqCst);
        eprintln!("zapdude: updater check thread could not start: {err}");
        return false;
    }
    true
}

fn run_check_until_download() {
    set_phase(Phase::Checking);
    let outcome = fetch_latest();
    match outcome {
        Ok(info) => {
            let current = parse_version(current_version()).unwrap_or_else(|| vec![0]);
            let latest = parse_version(&info.tag).unwrap_or_else(|| vec![0]);
            {
                let mut last = state()
                    .last_check
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                *last = Some(Instant::now());
            }
            if !version_newer(&latest, &current) {
                set_phase(Phase::Fresh { latest: info.tag });
                CHECK_IN_FLIGHT.store(false, Ordering::SeqCst);
                return;
            }
            set_phase(Phase::Newer {
                tag: info.tag.clone(),
                notes_url: info.notes_url.clone(),
                asset: info.asset.clone(),
            });
            CHECK_IN_FLIGHT.store(false, Ordering::SeqCst);
            // Auto-download right away: the user asked for the "download the
            // newest one" flow to be automatic end to end. The "off" policy
            // opts out of EVERYTHING automatic (that promise includes the
            // network fetches): only the drawer's manual buttons may download.
            if info.asset.is_some() && update_mode() != "off" {
                start_download();
            }
        }
        Err(error) => {
            set_phase(Phase::Failed {
                stage: "check",
                error,
            });
            CHECK_IN_FLIGHT.store(false, Ordering::SeqCst);
        }
    }
}

/// Start the background download thread (no-op when nothing newer exists,
/// the asset is missing, or one is already running).
pub fn start_download() -> bool {
    if DOWNLOAD_IN_FLIGHT
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return false;
    }
    let plan = with_phase(|phase| match phase {
        Some(Phase::Newer { tag, asset, .. }) => match asset {
            Some((name, url, size)) => Some((tag.clone(), name.clone(), url.clone(), *size)),
            None => None,
        },
        _ => None,
    });
    let (tag, name, url, size) = match plan {
        Some(plan) => plan,
        None => {
            DOWNLOAD_IN_FLIGHT.store(false, Ordering::SeqCst);
            return false;
        }
    };
    let spawn_result = std::thread::Builder::new()
        .name("zapdude-update-dl".to_string())
        .spawn(move || perform_download(tag, name, url, size));
    if let Err(err) = spawn_result {
        DOWNLOAD_IN_FLIGHT.store(false, Ordering::SeqCst);
        eprintln!("zapdude: updater download thread could not start: {err}");
        return false;
    }
    true
}

fn updates_dir() -> PathBuf {
    crate::SHARED
        .get()
        .map(|shared| shared.settings.data_dir().join("updates"))
        .unwrap_or_else(std::env::temp_dir)
}

fn perform_download(tag: String, name: String, url: String, expected_size: u64) {
    let dir = updates_dir();
    if let Err(err) = std::fs::create_dir_all(&dir) {
        fail_download(&tag, format!("could not create {}: {err}", dir.display()));
        return;
    }
    let final_path = dir.join(&name);
    let partial_path = dir.join(format!("{name}.part"));
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(600)))
        .build()
        .into();
    let mut response = match agent
        .get(&url)
        .header("User-Agent", UPDATER_UA)
        .header("Accept", "application/octet-stream")
        .call()
    {
        Ok(response) => response,
        Err(err) => {
            fail_download(&tag, format!("download request failed: {err}"));
            return;
        }
    };
    if !response.status().is_success() {
        fail_download(
            &tag,
            format!("download hit HTTP {}", response.status().as_u16()),
        );
        return;
    }
    let total = response.body().content_length();
    let mut reader = response.body_mut().as_reader();
    let mut file = match std::fs::File::create(&partial_path) {
        Ok(file) => file,
        Err(err) => {
            fail_download(
                &tag,
                format!("could not write {}: {err}", partial_path.display()),
            );
            return;
        }
    };
    let mut buffer = [0u8; 64 * 1024];
    let mut received: u64 = 0;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => {
                if let Err(err) = std::io::Write::write_all(&mut file, &buffer[..count]) {
                    fail_download(&tag, format!("write failed: {err}"));
                    return;
                }
                received += count as u64;
                // Phase update: /api/update polling (drawer + card) reads
                // it live while the bytes land. The throttled bus emit is
                // gone with the automation API.
                let mut guard = state()
                    .phase
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                *guard = Some(Phase::Downloading {
                    tag: tag.clone(),
                    received,
                    total,
                });
                drop(guard);
            }
            Err(err) => {
                fail_download(&tag, format!("read failed: {err}"));
                return;
            }
        }
    }
    // Sanity from the manifest: size matches when GitHub told us, and the
    // file must look like an executable (MZ header) - never install a blob.
    if let Err(err) = file.sync_all() {
        let _ = std::fs::remove_file(&partial_path);
        fail_download(&tag, format!("could not flush the installer: {err}"));
        return;
    }
    drop(file);
    if expected_size > 0 && received != expected_size {
        let _ = std::fs::remove_file(&partial_path);
        fail_download(
            &tag,
            format!("downloaded {} bytes, expected {expected_size}", received),
        );
        return;
    }
    if received < 1_000_000 {
        let _ = std::fs::remove_file(&partial_path);
        fail_download(
            &tag,
            format!("download suspiciously small ({received} bytes)"),
        );
        return;
    }
    match std::fs::File::open(&partial_path) {
        Ok(mut probe) => {
            let mut magic = [0u8; 2];
            let magic_ok = probe.read_exact(&mut magic).is_ok() && magic == [b'M', b'Z'];
            if !magic_ok {
                let _ = std::fs::remove_file(&partial_path);
                fail_download(&tag, "download is not an executable".to_string());
                return;
            }
        }
        Err(err) => {
            fail_download(&tag, format!("readback failed: {err}"));
            return;
        }
    }
    if let Err(err) = std::fs::rename(&partial_path, &final_path) {
        let _ = std::fs::remove_file(&partial_path);
        fail_download(&tag, format!("could not finalize the installer: {err}"));
        return;
    }
    set_phase(Phase::Ready {
        tag: tag.clone(),
        path: final_path.display().to_string(),
    });
    schedule_auto_install();
    DOWNLOAD_IN_FLIGHT.store(false, Ordering::SeqCst);
}

/// The live update policy ("ask" | "auto" | "off"), defaulting to "ask"
/// while the shared settings are not booted yet (early THREADS race: the
/// updater never blocks on config).
pub fn update_mode() -> String {
    crate::SHARED
        .get()
        .map(|shared| shared.settings.current_prefs().update_mode)
        .unwrap_or_else(|| "ask".to_string())
}

/// "auto" policy: once the installer is verified on disk, install and
/// relaunch with NO prompts. Runs on its own thread (the download thread
/// stays clean), re-reads the preference before acting (the user may flip
/// the mode between "ready" and the 1.2 s grace beat), and failures land in
/// the failed phase where the drawer prints them verbatim.
fn schedule_auto_install() {
    if update_mode() != "auto" {
        return;
    }
    let spawn_result = std::thread::Builder::new()
        .name("zapdude-auto-install".to_string())
        .spawn(|| {
            // A short beat: the bus carries "ready" (drawer + journal get
            // one honest look) before the app starts its exit flow.
            std::thread::sleep(Duration::from_millis(1200));
            if update_mode() != "auto" {
                return;
            }
            if let Err(error) = request_install() {
                eprintln!("zapdude: auto-install did not run: {error}");
            }
        });
    if let Err(err) = spawn_result {
        eprintln!("zapdude: auto-install thread could not start: {err}");
    }
}

/// Pure policy behind the center "new update ready" card: only in "ask"
/// mode, only when a VERIFIED installer is ready, only once per release
/// tag (dismissing the card stops it until a NEWER tag arrives). Kept pure
/// so the preference semantics are testable without the global phase lock.
fn nudge_decision(mode: &str, phase: &str, dismissed: &str, tag: &str) -> bool {
    mode == "ask" && phase == "ready" && dismissed != tag
}

/// Should the shell show the center update card right now?
pub fn should_nudge() -> bool {
    let phase = with_phase(|phase| phase.clone());
    let (phase_name, tag) = match phase {
        Some(Phase::Ready { tag, .. }) => ("ready", tag),
        _ => (
            phase_name(phase.as_ref().unwrap_or(&Phase::Idle)),
            String::new(),
        ),
    };
    if phase_name != "ready" {
        return false;
    }
    let prefs = crate::SHARED
        .get()
        .map(|shared| shared.settings.current_prefs())
        .unwrap_or_default();
    nudge_decision(
        &prefs.update_mode,
        "ready",
        &prefs.update_dismissed_tag,
        &tag,
    )
}

/// A policy flip out of "off" ("ask"/"auto") may need to resume a NEWER
/// release that "off" left undownloaded. No-op on every other phase: a
/// ready installer and its auto-install timer are already on rails.
pub fn resume_after_policy_change() {
    let undownloaded = with_phase(|phase| matches!(phase, Some(Phase::Newer { .. })));
    if undownloaded && update_mode() != "off" {
        request_check("policy");
    }
}

fn fail_download(tag: &str, error: String) {
    DOWNLOAD_IN_FLIGHT.store(false, Ordering::SeqCst);
    set_phase(Phase::Failed {
        stage: "download",
        error: format!("v{tag}: {error}"),
    });
}

/// Force a download NOW (drawer button). Idempotent: a running download and
/// an already-ready installer both just report their state back.
pub fn request_download() -> bool {
    ensure_fresh_plan();
    start_download()
}

/// When the process restarted between check and download (rare), a stale
/// phase can survive; this normalizes an idle state into an honest
/// re-check request. Other phases pass through untouched.
fn ensure_fresh_plan() {
    let idle = with_phase(|phase| matches!(phase, Some(Phase::Idle) | None));
    if idle {
        request_check("manual");
    }
}

/// One-click install. Requires `ready` with the downloaded installer on
/// disk + this being Windows. Quits the app after the wrapper batch is
/// spawned so NSIS never races a locked exe. The atomic guard makes the
/// auto-install thread and a simultaneous manual click collapse into ONE
/// wrapper batch (two installers racing the same exe would be messy).
pub fn request_install() -> Result<(), String> {
    if INSTALL_IN_FLIGHT
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return Err("an install is already running".to_string());
    }
    let (path, tag) = with_phase(|phase| match phase {
        Some(Phase::Ready { tag, path, .. }) => Some((path.clone(), tag.clone())),
        _ => None,
    })
    .ok_or_else(|| {
        INSTALL_IN_FLIGHT.store(false, Ordering::SeqCst);
        "no downloaded installer yet".to_string()
    })?;
    if !cfg!(windows) {
        INSTALL_IN_FLIGHT.store(false, Ordering::SeqCst);
        return Err("self-install currently supports Windows only".to_string());
    }
    let installer = PathBuf::from(&path);
    if !installer.is_file() {
        INSTALL_IN_FLIGHT.store(false, Ordering::SeqCst);
        set_phase(Phase::Failed {
            stage: "install",
            error: "the downloaded installer vanished".to_string(),
        });
        return Err("the downloaded installer vanished".to_string());
    }
    // The app exits right after the wrapper spawn: the guard stays armed
    // for the remaining lifetime of THIS build (a fresh process resets it
    // back to false via the static initializer).
    run_install_sequence(&installer, tag)?;
    Ok(())
}

fn run_install_sequence(installer: &Path, tag: String) -> Result<(), String> {
    let exe =
        std::env::current_exe().map_err(|err| format!("cannot resolve the current exe: {err}"))?;
    let script_text = install_script_text(installer, &exe);
    let script_path = updates_dir().join("install-update.cmd");
    std::fs::write(&script_path, script_text)
        .map_err(|err| format!("could not write the installer script: {err}"))?;

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        std::process::Command::new("cmd")
            .arg("/C")
            .arg(&script_path)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|err| format!("could not spawn the installer: {err}"))?;
    }
    #[cfg(not(windows))]
    {
        std::process::Command::new("cmd")
            .arg("/C")
            .arg(&script_path)
            .spawn()
            .map_err(|err| format!("could not spawn the installer: {err}"))?;
    }
    set_phase(Phase::Installing { tag });
    // Graceful exit: the Tauri runtime shuts down (tray gone cleanly), and a
    // belt-and-suspenders hard exit covers a wedged webview 2 s later.
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(700));
        if let Some(app) = crate::APP_HANDLE.get() {
            app.exit(0);
        }
        std::thread::spawn(|| {
            std::thread::sleep(Duration::from_millis(1500));
            std::process::exit(0);
        });
    });
    Ok(())
}

/// The wrapper batch: wait for the app to drain, force-kill stragglers, run
/// the NSIS installer silently (/S, per-user), then relaunch the app.
fn install_script_text(installer: &Path, exe: &Path) -> String {
    let installer_text = installer.display().to_string();
    let exe_text = exe.display().to_string();
    format!(
        "@echo off\r\ncd /d \"{}\"\r\ntimeout /t 2 /nobreak >nul\r\n\
         taskkill /f /im zapdude.exe >nul 2>&1\r\n\
         if not exist \"{}\" exit /b 1\r\n\
         start \"zapdude-update\" /wait \"{}\" /S\r\n\
         if exist \"{}\" start \"\" \"{}\"\r\n",
        updates_dir().display(),
        installer_text,
        installer_text,
        exe_text,
        exe_text,
    )
}

/// Status snapshot for GET /api/update (and bus events).
pub fn status_json() -> Value {
    let phase = with_phase(|phase| phase.clone());
    let last_check_epoch = state()
        .last_check
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .map(|when| epoch_now() - when.elapsed().as_secs());
    let current = current_version();
    let prefs = crate::SHARED
        .get()
        .map(|shared| shared.settings.current_prefs())
        .unwrap_or_default();
    let mut update = json!({
        "current": current,
        "phase": phase_name(phase.as_ref().unwrap_or(&Phase::Idle)),
        "last_check": last_check_epoch,
        "mode": prefs.update_mode,
        "dismissed_tag": prefs.update_dismissed_tag,
    });
    match phase {
        Some(Phase::Fresh { latest }) => {
            update["latest_tag"] = json!(latest);
            update["newer"] = json!(false);
        }
        Some(Phase::Newer {
            tag,
            notes_url,
            asset,
        }) => {
            update["latest_tag"] = json!(tag);
            update["notes_url"] = json!(notes_url);
            update["newer"] = json!(true);
            if let Some((name, _url, size)) = asset {
                update["asset_name"] = json!(name);
                update["asset_size"] = json!(size);
            }
        }
        Some(Phase::Downloading {
            tag,
            received,
            total,
        }) => {
            update["latest_tag"] = json!(tag);
            let mut progress = json!({ "received": received });
            if let Some(total) = total.filter(|total| *total > 0) {
                progress["total"] = json!(total);
                progress["percent"] =
                    json!(((received as f64 / total as f64) * 100.0).round() as u32);
            }
            update["progress"] = progress;
        }
        Some(Phase::Ready { tag, path, .. }) => {
            update["latest_tag"] = json!(tag);
            update["newer"] = json!(true);
            update["installer_path"] = json!(path);
        }
        Some(Phase::Installing { tag }) => {
            update["latest_tag"] = json!(tag);
        }
        Some(Phase::Failed { stage, error }) => {
            update["stage"] = json!(stage);
            update["error"] = json!(error);
        }
        _ => {}
    }
    json!({ "ok": true, "update": update })
}

fn epoch_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or(0)
}

/// Boot wiring: the schedule thread (first check ~3 min in, then every 5 h
/// + auto-download). The startup needs no network at boot time at all.
pub fn spawn_background() {
    let spawn_result = std::thread::Builder::new()
        .name("zapdude-update-loop".to_string())
        .spawn(|| loop {
            std::thread::sleep(FIRST_CHECK_DELAY);
            request_check("auto");
            loop {
                std::thread::sleep(CHECK_EVERY);
                request_check("auto");
            }
        });
    if let Err(err) = spawn_result {
        eprintln!("zapdude: updater loop thread could not start: {err}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_versions_through_noise() {
        assert_eq!(parse_version("1.3.3"), Some(vec![1, 3, 3]));
        assert_eq!(parse_version("v1.4.0"), Some(vec![1, 4, 0]));
        assert_eq!(parse_version("V2.10.1"), Some(vec![2, 10, 1]));
        assert_eq!(parse_version("1.4"), Some(vec![1, 4]));
        assert_eq!(parse_version(""), None);
        assert_eq!(parse_version("beta"), None);
    }

    #[test]
    fn compares_semantic_versions() {
        let current = vec![1, 3, 3];
        assert!(version_newer(&[1, 3, 4], &current));
        assert!(version_newer(&[1, 4, 0], &current));
        assert!(version_newer(&[2, 0, 0], &current));
        assert!(!version_newer(&[1, 3], &current));
        assert!(!version_newer(&[1, 3, 3], &current));
        assert!(!version_newer(&[1, 3, 2], &current));
        // Missing segments act as zero: semver "1.4" == "1.4.0", so a
        // shorter tag never triggers a pointless reinstall of EQUAL bits.
        assert!(version_newer(&[1, 4], &[1, 3, 9]));
        assert!(!version_newer(&[1, 3, 3], &[1, 3, 3]));
        assert!(version_newer(&[1, 3, 3], &[1, 3]));
    }

    #[test]
    fn validates_tag_shapes() {
        assert!(tag_shape_valid("1.4.0"));
        assert!(tag_shape_valid("1.4"));
        assert!(tag_shape_valid("10.2.33"));
        assert!(!tag_shape_valid(""));
        assert!(!tag_shape_valid("1.4.0-beta"));
        assert!(!tag_shape_valid("x"));
    }

    #[test]
    fn installer_assets_match_both_product_spellings() {
        // Exact names, both eras: today Zapdude_*, from the rename cut
        // Corvid_*; the hop bridge needs both to be accepted.
        assert_eq!(
            installer_asset_priority("Zapdude_1.4.4_x64-setup.exe", "1.4.4"),
            Some(0)
        );
        assert_eq!(
            installer_asset_priority("Corvid_1.4.5_x64-setup.exe", "1.4.5"),
            Some(0)
        );
        // Generic v-prefixed spellings rank second.
        assert_eq!(
            installer_asset_priority("Zapdude_v1.4.4_x64-setup.exe", "1.4.4"),
            Some(1)
        );
        assert_eq!(
            installer_asset_priority("Corvid_v2.0.0_x64-setup.exe", "2.0.0"),
            Some(1)
        );
        // Wrong product, wrong tag or non-installers never match, and a
        // .dmg (macOS asset) is explicitly not an update source.
        assert_eq!(
            installer_asset_priority("Zapdude_1.4.4_x64-setup.exe", "1.4.3"),
            None
        );
        assert_eq!(
            installer_asset_priority("Zapdude_1.4.4_aarch64.dmg", "1.4.4"),
            None
        );
        assert_eq!(
            installer_asset_priority("other_1.4.4_x64-setup.exe", "1.4.4"),
            None
        );
        // Exact beats generic (picked by priority in fetch_latest).
        assert!(
            installer_asset_priority("Zapdude_1.4.4_x64-setup.exe", "1.4.4")
                < installer_asset_priority("Zapdude_v1.4.4_x64-setup.exe", "1.4.4")
        );
    }

    #[test]
    fn nudge_policy_is_strict() {
        // The card only exists in "ask" mode, only on "ready", and only
        // once per release tag.
        assert!(nudge_decision("ask", "ready", "", "1.4.3"));
        assert!(!nudge_decision("ask", "ready", "1.4.3", "1.4.3"));
        assert!(nudge_decision("ask", "ready", "1.4.2", "1.4.3"));
        // "auto" never asks (it just installs); "off" never shows anything.
        assert!(!nudge_decision("auto", "ready", "", "1.4.3"));
        assert!(!nudge_decision("off", "ready", "", "1.4.3"));
        // A "newer"/"downloading" phase is NOT the ask moment: the card
        // waits until the installer is verified on disk.
        assert!(!nudge_decision("ask", "newer", "", "1.4.3"));
        assert!(!nudge_decision("ask", "downloading", "", "1.4.3"));
        assert!(!nudge_decision("ask", "fresh", "", "1.4.3"));
    }

    #[test]
    fn install_script_waits_kills_installs_restarts() {
        let text = install_script_text(
            Path::new("C:\\updates piece\\Zapdude_1.4.0_x64-setup.exe"),
            Path::new("C:\\Users\\t\\AppData\\Local\\Zapdude\\zapdude.exe"),
        );
        assert!(text.contains("@echo off"));
        assert!(text.contains("taskkill /f /im zapdude.exe"));
        assert!(text.contains("/S"));
        assert!(text.contains("Zapdude_1.4.0_x64-setup.exe"));
        assert!(text.contains("start \"zapdude-update\" /wait"));
        // The relaunch happens AFTER the installer (start /wait).
        let wait_pos = text.find("start \"zapdude-update\"").unwrap();
        let relaunch_pos = text.find("start \"\" ").unwrap();
        assert!(relaunch_pos > wait_pos);
        assert!(text.ends_with("\r\n"));
    }

    #[test]
    fn phases_map_to_api_strings() {
        assert_eq!(phase_name(&Phase::Idle), "idle");
        assert_eq!(phase_name(&Phase::Checking), "checking");
        assert_eq!(
            phase_name(&Phase::Fresh {
                latest: "1.3.3".into()
            }),
            "fresh"
        );
        assert_eq!(
            phase_name(&Phase::Newer {
                tag: "1.4.0".into(),
                notes_url: String::new(),
                asset: None,
            }),
            "newer"
        );
        assert_eq!(
            phase_name(&Phase::Downloading {
                tag: "1.4.0".into(),
                received: 1,
                total: None,
            }),
            "downloading"
        );
        assert_eq!(
            phase_name(&Phase::Ready {
                tag: "1.4.0".into(),
                path: String::new(),
            }),
            "ready"
        );
        assert_eq!(
            phase_name(&Phase::Installing {
                tag: "1.4.0".into()
            }),
            "installing"
        );
        assert_eq!(
            phase_name(&Phase::Failed {
                stage: "check",
                error: String::new(),
            }),
            "failed"
        );
    }
}
