//! Loopback HTTP server for the Corvid shell (axum).
//!
//! This server IS the shell's own surface: it serves the rail, the
//! welcome page, the drawer and the card pages, plus the management
//! endpoints those pages consume. Since v2.0.0 it is NOT a public
//! automation API anymore: no message/chat-list/call surface, no API
//! keys, no SSE, no webhook. The loopback binding plus the same-origin
//! guard (Sec-Fetch-Site / Origin checks) are the whole door. The red
//! lines live in docs/RED-LINES.md.
//!
//! Endpoints (management tier, same-origin only):
//!   GET  /api/healthz                - availability probe
//!   GET  /api/accounts               - configured accounts (rail sync)
//!   POST /api/accounts               - add an account (welcome / picker)
//!   DELETE /api/accounts/{id}        - remove an account (drawer)
//!   POST /api/accounts/{id}/open     - show an account in the shell
//!   POST /api/accounts/{id}/hide     - hide an account webview
//!   POST /api/accounts/{id}/rename   - rename an account (drawer)
//!   GET|POST /api/prefs              - shell preferences (theme and co.)
//!   GET|POST /api/browsers           - external browser for chat links
//!   GET  /api/traffic                - netguard host ledger (security lens)
//!   GET|POST /api/shield             - permission shield state/switches
//!   GET  /api/update (+ variants)    - updater status and actions
//!   POST /api/ui/*                   - shell choreography (rail + pages)

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, OnceLock};

use axum::extract::{DefaultBodyLimit, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tauri::Manager;

use super::link::{page_for, PageError};
use crate::config::{self, AccountConfig, SettingsState};

/// Historical 1 MB cap. Management payloads are small JSON documents;
/// the bound stays as a hard stop against abusive local actors.
const MAX_BODY_BYTES: usize = 1_048_576;

/// The shell UI is served BY this server (same origin as its endpoints),
/// so the rail/welcome/drawer pages never suffer CORS or path issues.
const RAIL_HTML: &str = include_str!("../../ui/rail.html");
const WELCOME_HTML: &str = include_str!("../../ui/welcome.html");
const DRAWER_HTML: &str = include_str!("../../ui/drawer.html");
const PICKER_STRIP_HTML: &str = include_str!("../../ui/picker_strip.html");
const UPDATE_NUDGE_HTML: &str = include_str!("../../ui/update_nudge.html");
const BOOT_CURTAIN_HTML: &str = include_str!("../../ui/boot_curtain.html");

/// Holder guarded by a Mutex so the server can be stopped and restarted
/// without losing the slot.
static SERVER: OnceLock<Mutex<Option<ServerHolder>>> = OnceLock::new();

struct ServerHolder {
    shutdown: Arc<std::sync::atomic::AtomicBool>,
}

fn server_slot() -> &'static Mutex<Option<ServerHolder>> {
    SERVER.get_or_init(|| Mutex::new(None))
}

fn server_running() -> bool {
    server_slot()
        .lock()
        .ok()
        .and_then(|guard| {
            guard
                .as_ref()
                .map(|holder| !holder.shutdown.load(std::sync::atomic::Ordering::Relaxed))
        })
        .unwrap_or(false)
}

#[derive(Clone)]
pub struct ApiState {
    pub settings: SettingsState,
}

fn settings() -> Option<SettingsState> {
    crate::SHARED.get().map(|shared| shared.settings.clone())
}

/// Start the loopback server (idempotent). ALWAYS binds 127.0.0.1: since
/// v2.0.0 this surface is the shell's own UI and management tier only,
/// and a network-facing binding buys nothing (the network exposure
/// option died with the automation API).
pub fn start_loopback_server() {
    {
        let guard = match server_slot().lock() {
            Ok(guard) => guard,
            Err(_) => return,
        };
        if let Some(holder) = guard.as_ref() {
            if !holder.shutdown.load(std::sync::atomic::Ordering::Relaxed) {
                return; // already running
            }
        }
    }
    let Some(state_settings) = settings() else {
        return;
    };
    let state = ApiState {
        settings: state_settings,
    };
    let port = state.settings.port();
    let bind_addr = format!("127.0.0.1:{port}");
    let bind_log = format!("http://{bind_addr}");

    let shutdown = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let shutdown_handle = shutdown.clone();
    let thread = std::thread::Builder::new()
        .name("zapdude-shell-server".into())
        .spawn(move || {
            let bind_addr = bind_addr.clone();
            let runtime = match tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .thread_name("zapdude-shell-io")
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(err) => {
                    eprintln!("zapdude: shell server runtime could not start: {}", err);
                    return;
                }
            };
            let app = router(state);
            let serve = async {
                // Bind with retries: if a previous listener is still
                // shutting down, the port may be held for a moment
                // (os error 10048). Without retries the shell UI would
                // stay dead.
                let mut listener = None;
                for attempt in 0..40u32 {
                    match tokio::net::TcpListener::bind(&bind_addr).await {
                        Ok(bound) => {
                            listener = Some(bound);
                            break;
                        }
                        Err(err) => {
                            if attempt == 0 || attempt % 8 == 0 {
                                eprintln!(
                                    "zapdude: shell bind retry #{} on {}: {}",
                                    attempt + 1,
                                    bind_addr,
                                    err
                                );
                            }
                            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                        }
                    }
                }
                match listener {
                    Some(listener) => {
                        if let Err(err) = axum::serve(
                            listener,
                            app.into_make_service_with_connect_info::<SocketAddr>(),
                        )
                        .await
                        {
                            eprintln!("zapdude: shell server stopped: {}", err);
                        }
                    }
                    None => {
                        eprintln!("zapdude: shell bind failed on {} (gave up)", bind_addr);
                    }
                }
            };
            let watcher = {
                let shutdown_flag = shutdown_handle.clone();
                async move {
                    loop {
                        if shutdown_flag.load(std::sync::atomic::Ordering::Relaxed) {
                            break;
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                    }
                }
            };
            let _exit = runtime.block_on(async {
                tokio::select! {
                    _ = serve => 0u8,
                    _ = watcher => 1u8,
                }
            });
            runtime.shutdown_timeout(std::time::Duration::from_secs(2));
        });
    if thread.is_ok() {
        eprintln!("zapdude: shell server on {bind_log} [loopback only]");
        if let Ok(mut guard) = server_slot().lock() {
            *guard = Some(ServerHolder { shutdown });
        }
    }
}

/// Stop the server (shell endpoints unreachable; restartable).
pub fn shutdown_loopback_server() {
    if let Ok(mut guard) = server_slot().lock() {
        if let Some(holder) = guard.take() {
            holder
                .shutdown
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

fn router(state: ApiState) -> Router {
    Router::new()
        // --- Shell UI (same origin as the endpoints: fetch always works) ---
        .route("/", get(serve_rail))
        .route("/welcome", get(serve_welcome))
        .route("/drawer", get(serve_drawer))
        .route("/picker", get(serve_picker))
        .route("/update-nudge", get(serve_update_nudge))
        .route("/boot-curtain", get(serve_boot_curtain))
        .route("/api/ui/picker/show", post(ui_picker_show))
        .route("/api/ui/picker/hide", post(ui_picker_hide))
        .route("/api/ui/picker/toggle", post(ui_picker_toggle))
        .route("/api/ui/picker/touch", post(ui_picker_touch))
        .route("/api/ui/picker/soft-close", post(ui_picker_soft_close))
        .route("/api/ui/quit", post(ui_quit))
        .route("/api/ui/drawer", post(ui_toggle_drawer))
        .route("/api/ui/show", post(ui_show))
        .route("/api/ui/bootcurtain/hide", post(ui_bootcurtain_hide))
        .route("/api/ui/rail/refresh", post(ui_rail_refresh))
        // --- Management tier for the shell pages themselves ---
        .route("/api/healthz", get(healthz))
        .route("/api/prefs", get(get_prefs).post(post_prefs))
        .route("/api/accounts", get(list_accounts).post(add_account_http))
        .route("/api/accounts/{id}", delete(remove_account_http))
        .route("/api/accounts/{id}/open", post(open_account_http))
        .route("/api/accounts/{id}/hide", post(hide_account_http))
        .route("/api/accounts/{id}/rename", post(rename_account_http))
        .route(
            "/api/browsers",
            get(list_browsers).post(save_browser_choice),
        )
        .route("/api/traffic", get(traffic_snapshot))
        .route("/api/update", get(update_status))
        .route("/api/update/check", post(update_check_now))
        .route("/api/update/download", post(update_download_now))
        .route("/api/update/install", post(update_install_now))
        .route("/api/update/prefs", post(update_update_prefs))
        .route("/api/update/nudge/hide", post(update_nudge_hide))
        .route("/api/shield", get(shield_state).post(shield_update))
        .with_state(state)
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .layer(axum::middleware::from_fn(loopback_origin_guard))
}

/// LOCAL-SURFACE GUARD for every endpoint on this router.
///
/// A web page from the internet could always FIRE requests at
/// `http://127.0.0.1:29977` (per-destination: any site can send POSTs to
/// loopback ports from the user's browser: the so-called port-70 attack
/// plane). The rule:
///   - Browsers TAG every fetch/navigation with `Sec-Fetch-Site`;
///     requests marked "cross-site" (a different site) are refused.
///   - When an `Origin` header rides along it must be OUR OWN loopback
///     base (host + port): another local tool port is refused too.
/// Plain HTTP clients (curl, agents' local scripts, Node tools) send
/// NEITHER header: they keep working for shell-adjacent local tooling.
/// This matches the "management tier for the shell same-origin"
/// doctrine.
async fn loopback_origin_guard(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let headers = request.headers();
    let fetch_site = headers
        .get("sec-fetch-site")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    if fetch_site == "cross-site" {
        return (StatusCode::FORBIDDEN, "cross-site requests refused").into_response();
    }
    if let Some(origin) = headers.get("origin").and_then(|value| value.to_str().ok()) {
        let origin_parsed = url::Url::parse(origin);
        match origin_parsed {
            Ok(origin_url) => {
                let host = origin_url.host_str().unwrap_or("").to_ascii_lowercase();
                let port = origin_url.port_or_known_default();
                let self_port = crate::SHARED
                    .get()
                    .map(|shared| shared.settings.port())
                    .unwrap_or(crate::config::DEFAULT_API_PORT);
                let same_host = matches!(host.as_str(), "127.0.0.1" | "localhost" | "[::1]");
                let same_port = port.map(|port| port == self_port).unwrap_or(false);
                if !same_host || !same_port {
                    return (
                        StatusCode::FORBIDDEN,
                        format!("origin {origin} is not the Corvid shell"),
                    )
                        .into_response();
                }
            }
            Err(_) => {
                return (StatusCode::FORBIDDEN, "unparseable origin refused").into_response();
            }
        }
    }
    next.run(request).await
}

// -------------------------------------------------------------------------
// Shared helpers
// -------------------------------------------------------------------------

fn json_ok(extra: Value) -> (StatusCode, Json<Value>) {
    let mut base = json!({ "ok": true });
    if let (Value::Object(base_obj), Value::Object(extra_obj)) = (&mut base, &extra) {
        for (key, value) in extra_obj {
            base_obj.insert(key.clone(), value.clone());
        }
    }
    (StatusCode::OK, Json(base))
}

fn json_error(err: &PageError) -> (StatusCode, Json<Value>) {
    (
        StatusCode::from_u16(err.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        Json(json!({
            "ok": false,
            "error": err.code,
            "message": err.message,
        })),
    )
}

// -------------------------------------------------------------------------
// Shell UI handlers (same-origin pages and their choreography)
// -------------------------------------------------------------------------

async fn serve_rail() -> impl IntoResponse {
    axum::response::Html(RAIL_HTML)
}

async fn serve_welcome() -> impl IntoResponse {
    axum::response::Html(WELCOME_HTML)
}

async fn serve_drawer() -> impl IntoResponse {
    axum::response::Html(DRAWER_HTML)
}

/// The "+" hover-menu strip page (a child webview re-anchored by the host
/// beside the rail; see window.rs for the open/close choreography).
async fn serve_picker() -> impl IntoResponse {
    axum::response::Html(PICKER_STRIP_HTML)
}

/// Rail "+" interactions. /show opens (idempotent), /hide closes,
/// /toggle flips (the + click), /touch reports the cursor entering the
/// strip (cancels the rail's soft-close), /soft-close schedules the
/// close the rail asks for when the cursor leaves the +.
async fn ui_picker_show() -> (StatusCode, Json<Value>) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        crate::window::show_picker(app_handle);
        json_ok(json!({ "ok": true }))
    } else {
        json_error(&PageError::internal("app handle not available"))
    }
}

async fn ui_picker_hide() -> (StatusCode, Json<Value>) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        crate::window::hide_picker(app_handle);
        json_ok(json!({ "ok": true }))
    } else {
        json_error(&PageError::internal("app handle not available"))
    }
}

async fn ui_picker_toggle() -> (StatusCode, Json<Value>) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        crate::window::toggle_picker(app_handle);
        json_ok(json!({ "ok": true }))
    } else {
        json_error(&PageError::internal("app handle not available"))
    }
}

async fn ui_picker_touch() -> (StatusCode, Json<Value>) {
    crate::window::picker_touch();
    json_ok(json!({ "ok": true }))
}

async fn ui_picker_soft_close() -> (StatusCode, Json<Value>) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        crate::window::soft_close_picker(app_handle);
        json_ok(json!({ "ok": true }))
    } else {
        json_error(&PageError::internal("app handle not available"))
    }
}

/// Toggle the options drawer (brand-logo click in the rail).
async fn ui_toggle_drawer() -> (StatusCode, Json<Value>) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        if let Err(err) = crate::window::toggle_drawer(app_handle) {
            return json_error(&PageError::internal(err.to_string()));
        }
        json_ok(json!({ "ok": true }))
    } else {
        json_error(&PageError::internal("app handle not available"))
    }
}

/// Bring the first instance's window back: a second launch (desktop
/// shortcut, Start Menu, exe double click) hits single-instance handling
/// and pings this endpoint instead of dying silently (main.rs). The
/// loopback binding plus the origin guard restrict this to local
/// callers.
async fn ui_show() -> (StatusCode, Json<Value>) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        crate::window::show_main_window(app_handle);
        json_ok(json!({ "shown": true }))
    } else {
        json_error(&PageError::internal("app handle not available"))
    }
}

/// Quit the whole app from the shell UI.
async fn ui_quit() -> (StatusCode, Json<Value>) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        let handle = app_handle.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(200));
            shutdown_loopback_server();
            handle.exit(0);
        });
        json_ok(json!({ "quitting": true }))
    } else {
        json_error(&PageError::internal("app handle not available"))
    }
}

/// Tell the rail to refresh NOW (the + strip / welcome pages call this
/// right after /api/accounts succeeds so the new tile shows in a beat
/// instead of waiting for the next 2.5 s poll).
async fn ui_rail_refresh() -> (StatusCode, Json<Value>) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        if let Some(rail) = app_handle.get_webview("rail") {
            // Idempotent hook (older rail builds without it simply no-op).
            let _ = rail
                .eval("(window.__zapdudeRailRefresh ? window.__zapdudeRailRefresh() : null); true");
        }
    }
    (StatusCode::OK, Json(json!({ "ok": true })))
}

/// The center "new update ready" card page (window.rs creates/closes the
/// webview per updater:should_nudge; this endpoint serves its HTML).
async fn serve_update_nudge() -> impl IntoResponse {
    axum::response::Html(UPDATE_NUDGE_HTML)
}

/// The account-boot curtain page (covers the content area while a chat
/// page boots so its native loading states never flash in the shell).
async fn serve_boot_curtain() -> impl IntoResponse {
    axum::response::Html(BOOT_CURTAIN_HTML)
}

/// Curtain dismissal (the curtain page fires this once its account's
/// session state is honest: logged in / needs login / QR / error).
async fn ui_bootcurtain_hide(
    State(state): State<ApiState>,
    Json(body): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let account = body
        .get("account")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();
    let _ = state;
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        crate::window::close_boot_curtain(app_handle, &account);
    }
    (StatusCode::OK, Json(json!({ "ok": true })))
}

// -------------------------------------------------------------------------
// Session states (rail dots + boot curtain resolution)
// -------------------------------------------------------------------------

/// Session state derived from a page snapshot (three honest states for
/// WhatsApp, plus the login wait for the other embedded chat webs).
/// The login-form signal is now the page link helper's generic
/// password-field probe (the per-service DOM bridges are gone with the
/// automation API).
fn three_state(snapshot: &Value, service: &str) -> String {
    let composer = snapshot
        .get("composer_visible")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let chat_list = snapshot
        .get("chat_list")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let qrcode = snapshot
        .get("qrcode_visible")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if composer || chat_list {
        "logged_in".to_string()
    } else if service == crate::config::SERVICE_FACEBOOK
        || service == crate::config::SERVICE_INSTAGRAM
        || service == crate::config::SERVICE_X
        || service == crate::config::SERVICE_TIKTOK
        || service == crate::config::SERVICE_TWITCH
    {
        // Facebook, Instagram, X, TikTok and Twitch have no QR stage:
        // the login screen (email + password + 2FA) means "waiting for
        // the user to sign in".
        match snapshot.get("login_form_visible").and_then(Value::as_bool) {
            Some(true) => "needs_login".to_string(),
            _ => "detecting".to_string(),
        }
    } else if qrcode {
        "scan_qr".to_string()
    } else {
        "detecting".to_string()
    }
}

/// Cached session states (id → (state, fetched_at)) so the rail UI never
/// waits on a page snapshot: responses are instant, stale entries refresh
/// in the background.
static SESSION_STATE_CACHE: OnceLock<Mutex<HashMap<String, (String, std::time::Instant)>>> =
    OnceLock::new();

fn session_state_cache() -> &'static Mutex<HashMap<String, (String, std::time::Instant)>> {
    SESSION_STATE_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

const SESSION_STATE_TTL: std::time::Duration = std::time::Duration::from_millis(2500);
/// Hard cap for serving a STALE state: after this age the value is treated
/// as unknown (grey dot) even while a refresh is in flight.
const SESSION_STATE_MAX: std::time::Duration = std::time::Duration::from_millis(30_000);

/// Returns (served state, needs background refresh). STALE states keep
/// serving their last value while the refresh runs - the rail dot used to
/// rotate green/grey because a fresh positive state flipped to "detecting"
/// the moment the 2.5s TTL expired (the rail polls every 2.5s, so the flap
/// was on EVERY poll). Stale-while-revalidate keeps the honest value shown
/// and updates it silently.
fn session_state_entry(account_id: &str) -> (Option<String>, bool) {
    match session_state_cache().lock() {
        Ok(map) => match map.get(account_id) {
            Some((state, fetched)) => {
                let age = fetched.elapsed();
                if age < SESSION_STATE_TTL {
                    (Some(state.clone()), false)
                } else if age < SESSION_STATE_MAX {
                    (Some(state.clone()), true)
                } else {
                    (None, true)
                }
            }
            None => (None, true),
        },
        Err(_) => (None, true),
    }
}

fn spawn_session_state_refresh(account_id: String) {
    tokio::spawn(async move {
        let service = crate::SHARED
            .get()
            .and_then(|shared| shared.settings.account(&account_id))
            .map(|account| account.service)
            .unwrap_or_else(|| crate::config::SERVICE_WHATSAPP.to_string());
        let state = match page_for(Some(&account_id)) {
            None => "detecting".to_string(),
            Some(link) => match tokio::time::timeout(
                std::time::Duration::from_millis(1200),
                link.status_snapshot(),
            )
            .await
            {
                // A slow page is NOT an error: keep re-detecting quietly.
                Err(_timeout) => "detecting".to_string(),
                Ok(Ok(snapshot)) => three_state(&snapshot, &service),
                // Link not installed yet = still loading (not broken).
                Ok(Err(err)) if err.code == "page_unavailable" => "detecting".to_string(),
                // Any other evaluation failure = a real problem (red dot).
                Ok(Err(_failure)) => "page_error".to_string(),
            },
        };
        if let Ok(mut cache) = session_state_cache().lock() {
            cache.insert(account_id, (state, std::time::Instant::now()));
        }
    });
}

// -------------------------------------------------------------------------
// Accounts + unread badges (rail data)
// -------------------------------------------------------------------------

/// Last unread badge count per account id (page-sampled). The page
/// watcher (`unread_watcher.js`) keeps this fresh through the beacon
/// transport; `/api/accounts` serves it without touching the page DOM.
/// Reading account payloads is a hot path, hence the plain Mutex +
/// OnceLock.
static UNREAD_COUNTS: OnceLock<Mutex<HashMap<String, u32>>> = OnceLock::new();

/// Store the freshest unread count of an account (page transport ingest).
pub fn unread_store_set(account_id: &str, count: u32) {
    let map = UNREAD_COUNTS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(mut guard) = map.lock() {
        guard.insert(account_id.to_string(), count);
    }
}

/// Drop an account's unread state (removed/closed accounts stop existing).
pub fn unread_store_clear(account_id: &str) {
    let map = UNREAD_COUNTS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(mut guard) = map.lock() {
        guard.remove(account_id);
    }
}

/// The account's last known unread count (0 when never sampled).
fn unread_for(account_id: &str) -> u32 {
    let map = UNREAD_COUNTS.get_or_init(|| Mutex::new(HashMap::new()));
    map.lock()
        .ok()
        .and_then(|guard| guard.get(account_id).copied())
        .unwrap_or(0)
}

async fn healthz() -> impl IntoResponse {
    json_ok(json!({
        "service": "corvid-shell",
        "server_running": server_running(),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn list_accounts() -> (StatusCode, Json<Value>) {
    let Some(state_settings) = settings() else {
        return json_error(&PageError::internal("app state not initialized"));
    };
    let active = crate::window::active_content();
    let accounts = state_settings.current_accounts();
    // Background refresh for STALE entries - the response NEVER waits for a
    // page snapshot, and a stale honest value keeps serving while it runs
    // (stale-while-revalidate; the rail dot stopped flapping here).
    for account in &accounts {
        let (_cached, stale) = session_state_entry(&account.id);
        if stale {
            spawn_session_state_refresh(account.id.clone());
        }
    }
    let account_entries: Vec<Value> = accounts
        .iter()
        .map(|account| {
            let (cached, _) = session_state_entry(&account.id);
            json!({
                "id": account.id,
                "label": account.label,
                "enabled": account.enabled,
                "service": account.service,
                "active": active.as_deref()
                    == Some(crate::window::account_webview_label(&account.id).as_str()),
                "session_state": cached
                    .unwrap_or_else(|| "detecting".to_string()),
                "unread": unread_for(&account.id),
            })
        })
        .collect();
    // Shell button truth (the rail resyncs its optimistic pressed states:
    // the click answers immediately, THIS payload is the correction beat).
    let (drawer_open, picker_open) = crate::window::shell_open_states();
    json_ok(json!({
        "accounts": account_entries,
        "shell": { "drawer_open": drawer_open, "picker_open": picker_open },
    }))
}

/// Create an account the easy way: optional body, ids are generated
/// (`account-N`, label `Account N`) and the `service` selects which web
/// chat gets embedded (WhatsApp default - the legacy no-body callers keep
/// their exact behavior). The new account's chat web opens right away so
/// the login surface (QR / sign-in) is immediately on screen.
#[derive(Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct CreateAccountBody {
    service: Option<String>,
}

async fn add_account_http(
    State(state): State<ApiState>,
    body: Option<Json<CreateAccountBody>>,
) -> (StatusCode, Json<Value>) {
    let requested = body
        .as_ref()
        .and_then(|json| json.0.service.as_deref())
        .unwrap_or("");
    // Sanitize (whitelist + WhatsApp fallback): a bad or missing name can
    // never create a broken account.
    let service = crate::config::sanitize_service(requested);
    let mut accounts = state.settings.current_accounts();
    let mut n = accounts.len() + 1;
    let mut id = format!("account-{n}");
    while accounts.iter().any(|a| a.id == id) {
        n += 1;
        id = format!("account-{n}");
    }
    let label = format!("Account {n}");
    let account = AccountConfig {
        id: id.clone(),
        label: label.clone(),
        enabled: true,
        service,
    };
    accounts.push(account.clone());
    if let Err(err) = state.settings.update_accounts(accounts) {
        return json_error(&PageError::internal(err));
    }
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        let handle = app_handle.clone();
        std::thread::spawn(move || {
            if let Err(err) = crate::window::open_account_webview(&handle, &account) {
                eprintln!(
                    "zapdude: could not open new account '{}': {}",
                    account.id, err
                );
            }
            crate::window::refresh_tray(&handle);
        });
    }
    json_ok(json!({ "id": id, "label": label, "created": true }))
}

async fn remove_account_http(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    let Some(state_settings) = settings() else {
        return json_error(&PageError::internal("app state not initialized"));
    };
    let before = state_settings.current_accounts().len();
    let accounts: Vec<AccountConfig> = state_settings
        .current_accounts()
        .into_iter()
        .filter(|a| a.id != id)
        .collect();
    if accounts.len() == before {
        return json_error(&PageError::bad_request(format!("account '{id}' not found")));
    }
    if let Err(err) = state_settings.update_accounts(accounts) {
        return json_error(&PageError::internal(err));
    }
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        crate::window::close_account_webview(app_handle, &id);
        crate::window::refresh_tray(app_handle);
    }
    json_ok(json!({ "id": id }))
}

async fn open_account_http(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    let Some(state_settings) = settings() else {
        return json_error(&PageError::internal("app state not initialized"));
    };
    let Some(account) = state_settings.account(&id) else {
        return json_error(&PageError::bad_request(format!("account '{id}' not found")));
    };
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        let handle = app_handle.clone();
        std::thread::spawn(move || {
            if let Err(err) = crate::window::open_account_webview(&handle, &account) {
                eprintln!("zapdude: could not open account '{}': {}", account.id, err);
            }
        });
    }
    json_ok(json!({ "id": id, "opened": true }))
}

async fn hide_account_http(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        if let Err(err) = crate::window::hide_account_content(app_handle, &id) {
            return json_error(&PageError::internal(err.to_string()));
        }
        json_ok(json!({ "id": id, "hidden": true }))
    } else {
        json_error(&PageError::internal("app handle not available"))
    }
}

/// Rename an account (label only - ids are stable identities).
async fn rename_account_http(
    State(state): State<ApiState>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let raw = body.get("label").and_then(Value::as_str).unwrap_or("");
    let label: String = raw.trim().chars().take(40).collect();
    if label.is_empty() {
        return json_error(&PageError::bad_request("label is required"));
    }
    let accounts: Vec<AccountConfig> = state
        .settings
        .current_accounts()
        .into_iter()
        .map(|mut account| {
            if account.id == id {
                account.label = label.clone();
            }
            account
        })
        .collect();
    if !accounts.iter().any(|account| account.id == id) {
        return json_error(&PageError::bad_request(format!("account '{id}' not found")));
    }
    if let Err(err) = state.settings.update_accounts(accounts) {
        return json_error(&PageError::internal(err));
    }
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        crate::window::refresh_tray(app_handle);
    }
    json_ok(json!({ "id": id, "label": label }))
}

// -------------------------------------------------------------------------
// Preferences
// -------------------------------------------------------------------------

async fn get_prefs() -> (StatusCode, Json<Value>) {
    let Some(state_settings) = settings() else {
        return json_error(&PageError::internal("app state not initialized"));
    };
    let prefs = state_settings.current_prefs();
    json_ok(json!({
        "theme": prefs.theme,
        "autostart": prefs.autostart,
        "start_minimized": prefs.start_minimized,
        "unread_badges": prefs.unread_badges,
        "update_mode": prefs.update_mode,
        "external_browser": prefs.external_browser,
    }))
}

/// `unread_badges` defaults ON: absent fields (older clients / scripts
/// calling /api/prefs blind) keep the badges instead of silently
/// switching them off.
fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
struct PrefsBody {
    theme: String,
    autostart: bool,
    start_minimized: bool,
    #[serde(default = "default_true")]
    unread_badges: bool,
    /// Callers older than the update card omit this: when absent the stored
    /// policy survives (a theme toggle must never reset the update mode).
    #[serde(default)]
    update_mode: Option<String>,
    /// Same preservation law for the external browser choice.
    #[serde(default)]
    external_browser: Option<String>,
}

async fn post_prefs(
    State(state): State<ApiState>,
    Json(body): Json<PrefsBody>,
) -> (StatusCode, Json<Value>) {
    let current = state.settings.current_prefs();
    let prefs = config::AppPrefs {
        theme: body.theme,
        autostart: body.autostart,
        start_minimized: body.start_minimized,
        unread_badges: body.unread_badges,
        update_mode: body
            .update_mode
            .unwrap_or_else(|| current.update_mode.clone()),
        update_dismissed_tag: current.update_dismissed_tag,
        external_browser: body
            .external_browser
            .unwrap_or_else(|| current.external_browser.clone()),
        // The shield rides the preservation law: legacy callers (which
        // never send it) keep their stored devices intact.
        shield_camera: current.shield_camera,
        shield_mic: current.shield_mic,
        shield_screen: current.shield_screen,
        shield_clipboard: current.shield_clipboard,
    }
    .sanitized();
    if let Err(err) = state.settings.update_prefs(prefs.clone()) {
        return json_error(&PageError::internal(err));
    }
    // Windows autostart registration (Run key). Errors degrade: the pref
    // persists, the registry just may not have changed.
    let autostart_result = crate::window::apply_autostart(prefs.autostart);
    if let Err(err) = autostart_result {
        eprintln!("zapdude: autostart registry update failed: {err}");
    }
    // Re-apply the theme everywhere: rail + welcome reload with the fresh
    // preference (the drawer applied it locally already), and the account
    // webviews get the new color scheme pinned on their WebView2 profiles.
    // This is what keeps the whole window UNIFORM - pages that only read
    // prefs at load would otherwise mix light/dark after a toggle.
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        crate::window::reload_shell_pages(app_handle);
        crate::window::apply_theme_everywhere(app_handle);
    }
    json_ok(json!({
        "theme": prefs.theme,
        "autostart": prefs.autostart,
        "start_minimized": prefs.start_minimized,
        "unread_badges": prefs.unread_badges,
        "update_mode": prefs.update_mode,
        "external_browser": prefs.external_browser,
    }))
}

// -------------------------------------------------------------------------
// Update endpoints (management tier for the drawer + the update card)
// -------------------------------------------------------------------------

async fn update_status() -> impl IntoResponse {
    // The snapshot never reaches the network: it reports the updater's own
    // phase only (a stale "idle" is honest until a check actually lands).
    (StatusCode::OK, Json(crate::updater::status_json()))
}

async fn update_check_now() -> impl IntoResponse {
    // Fire-and-return: the check runs on its own thread; callers poll
    // GET /api/update (there is no reason to block this handler on GitHub).
    let accepted = crate::updater::request_check("manual");
    (
        StatusCode::OK,
        Json(json!({ "ok": true, "accepted": accepted })),
    )
}

async fn update_download_now() -> impl IntoResponse {
    let accepted = crate::updater::request_download();
    (
        StatusCode::OK,
        Json(json!({ "ok": true, "accepted": accepted })),
    )
}

async fn update_install_now() -> impl IntoResponse {
    match crate::updater::request_install() {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({ "ok": true, "installing": true })),
        ),
        Err(message) => json_error(&PageError::bad_request(message)),
    }
}

/// Update policy toggle (center card buttons + the drawer selector).
/// Accepts an optional `mode` ("ask" | "auto" | "off") and an optional
/// `dismissed_tag` (the release whose card was just dismissed); both merge
/// into the stored prefs, so a caller may drive one or the other.
async fn update_update_prefs(Json(body): Json<Value>) -> (StatusCode, Json<Value>) {
    let Some(state_settings) = settings() else {
        return json_error(&PageError::internal("app state not initialized"));
    };
    let mut prefs = state_settings.current_prefs();
    if let Some(mode) = body.get("mode").and_then(Value::as_str) {
        prefs.update_mode = mode.to_string(); // sanitized inside update_prefs
    }
    if let Some(tag) = body.get("dismissed_tag").and_then(Value::as_str) {
        prefs.update_dismissed_tag = tag.to_string();
    }
    if let Err(err) = state_settings.update_prefs(prefs.clone()) {
        return json_error(&PageError::internal(err));
    }
    // A flip out of "off" may owe the user a resumed download (a NEWER
    // release it never fetched): a fresh check drives it. "off" NEVER
    // fetches or installs on its own - the promise stays whole.
    crate::updater::resume_after_policy_change();
    crate::window::nudge_poll();
    (
        StatusCode::OK,
        Json(json!({
            "ok": true,
            "mode": prefs.update_mode,
            "dismissed_tag": prefs.update_dismissed_tag,
        })),
    )
}

/// Hide the center update card now (the page itself fires this after a
/// policy/dismiss POST keeps the state; nudge_poll closes it too, this just
/// makes the dismissal independent from any phase churn).
async fn update_nudge_hide() -> (StatusCode, Json<Value>) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        crate::window::hide_update_nudge(app_handle);
    }
    (StatusCode::OK, Json(json!({ "ok": true })))
}

// -------------------------------------------------------------------------
// Security: the permission shield + the network lens
// -------------------------------------------------------------------------

/// The permission shield state the drawer renders (same-origin read).
async fn shield_state() -> (StatusCode, Json<Value>) {
    let Some(state_settings) = settings() else {
        return json_error(&PageError::internal("app state not initialized"));
    };
    let prefs = state_settings.current_prefs();
    (
        StatusCode::OK,
        Json(json!({
            "ok": true,
            "camera": prefs.shield_camera,
            "mic": prefs.shield_mic,
            "screen": prefs.shield_screen,
            "clipboard": prefs.shield_clipboard,
        })),
    )
}

/// The permission shield switches (camera / microphone / screen share /
/// clipboard read). Each is a boolean: ON the page gets the device when
/// it asks, OFF Corvid refuses at the door. Anything omitted keeps the
/// stored value (same preservation law as the update policy).
async fn shield_update(Json(body): Json<Value>) -> (StatusCode, Json<Value>) {
    let Some(state_settings) = settings() else {
        return json_error(&PageError::internal("app state not initialized"));
    };
    let mut prefs = state_settings.current_prefs();
    if let Some(v) = body.get("camera").and_then(Value::as_bool) {
        prefs.shield_camera = v;
    }
    if let Some(v) = body.get("mic").and_then(Value::as_bool) {
        prefs.shield_mic = v;
    }
    if let Some(v) = body.get("screen").and_then(Value::as_bool) {
        prefs.shield_screen = v;
    }
    if let Some(v) = body.get("clipboard").and_then(Value::as_bool) {
        prefs.shield_clipboard = v;
    }
    if let Err(err) = state_settings.update_prefs(prefs.clone()) {
        return json_error(&PageError::internal(err));
    }
    (
        StatusCode::OK,
        Json(json!({
            "ok": true,
            "camera": prefs.shield_camera,
            "mic": prefs.shield_mic,
            "screen": prefs.shield_screen,
            "clipboard": prefs.shield_clipboard,
        })),
    )
}

/// The per-account network lens: every host the chats touched, with
/// request/block counters (the security panel's proof data). Same-origin
/// management tier; it IS the observable promise.
async fn traffic_snapshot() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(crate::netguard::snapshot()))
}

// -------------------------------------------------------------------------
// External browser choice (Windows StartMenuInternet enumeration)
// -------------------------------------------------------------------------

async fn list_browsers() -> (StatusCode, Json<Value>) {
    let selected = settings()
        .map(|settings| settings.current_prefs().external_browser)
        .unwrap_or_default();
    let browsers: Vec<Value> = crate::browsers::list_browsers()
        .into_iter()
        .map(|entry| json!({ "key": entry.key, "name": entry.name }))
        .collect();
    (
        StatusCode::OK,
        Json(json!({ "ok": true, "selected": selected, "browsers": browsers })),
    )
}

/// Save the external browser choice. Empty = system default; anything else
/// must be a key the registry LIST still carries (a stale hand-written
/// value degrades to the default at click time anyway - this handler just
/// keeps the pref honest).
async fn save_browser_choice(Json(body): Json<Value>) -> (StatusCode, Json<Value>) {
    let key = body
        .get("key")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();
    if !key.is_empty() {
        let known = crate::browsers::list_browsers()
            .into_iter()
            .any(|entry| entry.key == key);
        if !known {
            return json_error(&PageError::bad_request("unknown browser key"));
        }
    }
    let Some(settings) = settings() else {
        return json_error(&PageError::internal("app state not initialized"));
    };
    let mut prefs = settings.current_prefs();
    prefs.external_browser = key.clone();
    if let Err(err) = settings.update_prefs(prefs.clone()) {
        return json_error(&PageError::internal(err));
    }
    (
        StatusCode::OK,
        Json(json!({ "ok": true, "selected": prefs.external_browser })),
    )
}
