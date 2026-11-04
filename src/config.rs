//! Persistent settings for Corvid.
//!
//! Holds the shell loopback port, the multi-account registry and the UI
//! prefs. Everything lives in one JSON file inside the OS
//! application-data directory. Labels are kept separately from ids.

use std::path::{Path, PathBuf};
use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const DEFAULT_API_PORT: u16 = 29977;
pub const MIN_API_PORT: u16 = 1024;
pub const MAX_API_PORT: u16 = 65_535;

/// One chat account: stable id, human label, enabled flag, service.
///
/// `id` is the persisted identity (must survive label edits); `label` is
/// freely editable and shown in the tray and API outputs. `service` selects
/// the embedded web interface each account's webview loads (WhatsApp Web,
/// Facebook chat / Messenger, Instagram DMs). Old configs (without the
/// field) load as WhatsApp - the field can never break existing installs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountConfig {
    pub id: String,
    pub label: String,
    pub enabled: bool,
    pub service: String,
}

/// Known chat silos. A new embedding joins this list (add the const, the
/// URL and its page bridge - nothing else needs to change); anything else
/// normalizes to WhatsApp so a typo or legacy file never breaks loading.
pub const SERVICE_WHATSAPP: &str = "whatsapp";
pub const SERVICE_FACEBOOK: &str = "facebook";
pub const SERVICE_INSTAGRAM: &str = "instagram";
pub const SERVICE_X: &str = "x";
pub const SERVICE_TIKTOK: &str = "tiktok";
pub const SERVICE_TWITCH: &str = "twitch";
pub const KNOWN_SERVICES: [&str; 6] = [
    SERVICE_WHATSAPP,
    SERVICE_FACEBOOK,
    SERVICE_INSTAGRAM,
    SERVICE_X,
    SERVICE_TIKTOK,
    SERVICE_TWITCH,
];

/// Normalize a service name defensively (whitelist; WhatsApp fallback).
pub fn sanitize_service(raw: &str) -> String {
    let cleaned = raw.trim().to_ascii_lowercase();
    if KNOWN_SERVICES.contains(&cleaned.as_str()) {
        cleaned
    } else {
        SERVICE_WHATSAPP.to_string()
    }
}

impl Default for AccountConfig {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            label: "default".to_string(),
            enabled: true,
            service: SERVICE_WHATSAPP.to_string(),
        }
    }
}

fn sanitize_account_id(raw: &str) -> String {
    let cleaned: String = raw
        .trim()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .take(48)
        .collect();
    cleaned
}

fn sanitize_accounts(raw: Vec<AccountConfig>) -> Vec<AccountConfig> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for mut account in raw {
        account.id = sanitize_account_id(&account.id);
        account.service = sanitize_service(&account.service);
        if account.id.is_empty() || !seen.insert(account.id.clone()) {
            continue; // drop empty or duplicate ids
        }
        if account.label.trim().is_empty() {
            account.label = account.id.clone();
        } else {
            account.label = account.label.trim().to_string();
        }
        result.push(account);
    }
    // An EMPTY list is legal (the user removed every account); the shell
    // auto-creates a fresh account on the next open. The [default] fallback
    // only applies to legacy configs being parsed at load time.
    result
}

/// App-shell preferences (theme, autostart, minimized start).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct AppPrefs {
    /// Shell UI theme: "dark" (default) or "light".
    pub theme: String,
    /// Launch Corvid when Windows starts (HKCU Run key).
    pub autostart: bool,
    /// Start hidden in the tray instead of showing the window.
    pub start_minimized: bool,
    /// Unread count bubbles on the rail account tiles (the mobile-style
    /// badge). Defaults to true; new fields must keep the same default
    /// story so a config written by an older build still parses.
    #[serde(default = "crate::config::default_unread_badges")]
    pub unread_badges: bool,
    /// Update behavior:
    ///   "ask"  - download new versions in the background and ask before
    ///            installing (the center popup card, once per release);
    ///   "auto" - download, verify, install and relaunch with no prompts;
    ///   "off"  - NEVER download or install on its own: silent checks keep
    ///            the drawer honest, every action stays manual (the promise
    ///            behind "don't show again + No thanks").
    #[serde(default = "default_update_mode")]
    pub update_mode: String,
    /// Release tag whose popup the user already dismissed ("ask" mode shows
    /// the card once per release; a newer tag clears this implicitly).
    #[serde(default)]
    pub update_dismissed_tag: String,
    /// Browser for chat links that leave Corvid: "" (system default) or a
    /// key from Software\Clients\StartMenuInternet (validated on use; an
    /// unknown key degrades to the system default, never to a failure).
    #[serde(default)]
    pub external_browser: String,
    /// The permission shield (Corvid as the gatekeeper): when OFF, the
    /// pages that request the device get REFUSED inside the webview (the
    /// spy question dies at the door). Defaults ON to preserve the call
    /// flows the users already rely on (WhatsApp/Messenger video calls).
    /// Screen share and clipboard read default ON for the same reason and
    /// can be flipped OFF in the Security section at any time.
    #[serde(default = "default_shield_on")]
    pub shield_camera: bool,
    #[serde(default = "default_shield_on")]
    pub shield_mic: bool,
    #[serde(default = "default_shield_on")]
    pub shield_screen: bool,
    #[serde(default = "default_shield_on")]
    pub shield_clipboard: bool,
}

impl Default for AppPrefs {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            autostart: false,
            start_minimized: false,
            unread_badges: true,
            update_mode: default_update_mode(),
            update_dismissed_tag: String::new(),
            external_browser: String::new(),
            shield_camera: default_shield_on(),
            shield_mic: default_shield_on(),
            shield_screen: default_shield_on(),
            shield_clipboard: default_shield_on(),
        }
    }
}

impl AppPrefs {
    pub fn sanitized(mut self) -> Self {
        self.theme = if self.theme.eq_ignore_ascii_case("light") {
            "light".to_string()
        } else {
            "dark".to_string()
        };
        self.update_mode = normalize_update_mode(&self.update_mode);
        self.update_dismissed_tag = sanitize_release_tag(&self.update_dismissed_tag);
        self.external_browser = sanitize_browser_key(&self.external_browser);
        // The shield has no middle ground: a bool either passes or it does
        // not; nothing to normalize beyond the serde layer.
        self
    }
}

/// serde default hook for the four shield devices (ON keeps every call
/// flow from the first run; the drawer is the one who turns devices off).
fn default_shield_on() -> bool {
    true
}

/// serde default hook for `AppPrefs::update_mode` (configs older than the
/// update card never carried a policy; "ask" matches their behavior).
fn default_update_mode() -> String {
    "ask".to_string()
}

/// Whitelist the update policy: "auto" and "off" pass through, anything
/// else (typos, legacy junk, hostile values) falls back to "ask".
fn normalize_update_mode(raw: &str) -> String {
    match raw.trim().to_ascii_lowercase().as_str() {
        "auto" => "auto".to_string(),
        "off" => "off".to_string(),
        _ => "ask".to_string(),
    }
}

/// Release tags on the prefs side are pure digit-dot shapes ("1.4.3"); the
/// filter is injection hygiene only, never interpretation.
fn sanitize_release_tag(raw: &str) -> String {
    raw.trim()
        .trim_start_matches(['v', 'V'])
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .take(16)
        .collect()
}

/// The browser choice stores a registry key name; the narrow charset
/// mirrors browsers.rs's `registry_key_name_ok` so a hand-edited config
/// file can never smuggle a subpath or an argument into the verb lookup.
fn sanitize_browser_key(raw: &str) -> String {
    let key = raw.trim();
    if key.chars().all(|c| {
        c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '(' | ')' | ' ' | '+' | '#')
    }) {
        key.chars().take(64).collect()
    } else {
        String::new()
    }
}

/// serde default hook for `AppPrefs::unread_badges` (keep ON for configs
/// that predate the field).
fn default_unread_badges() -> bool {
    true
}

/// Whole configuration file shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ConfigFile {
    /// Loopback port the shell server binds (127.0.0.1 only). Carried
    /// over from the legacy `api.port` key on load; the network-facing
    /// option and the automation section it lived in are gone (the
    /// v2.0.0 red lines removed the local automation API).
    pub port: u16,
    pub accounts: Vec<AccountConfig>,
    pub prefs: AppPrefs,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            port: DEFAULT_API_PORT,
            // VIRGIN installs start with NO accounts: the shell auto-opens
            // a fresh "Account 1" (account-1) with its QR. The legacy
            // "default" account name from other clients' configs is never
            // recreated.
            accounts: vec![],
            prefs: AppPrefs::default(),
        }
    }
}

fn api_port_ok(value: u16) -> bool {
    (MIN_API_PORT..=MAX_API_PORT).contains(&value)
}

/// Thread-safe accessor to the persisted configuration.
#[derive(Clone, Debug)]
pub struct SettingsState {
    data_dir: PathBuf,
    inner: std::sync::Arc<RwLock<ConfigFile>>,
}

impl SettingsState {
    /// Load the settings from the given data directory (legacy-free, the
    /// Rust app is the only owner of the file).
    pub fn load(data_dir: &Path) -> Self {
        let path = data_dir.join("config.json");
        // A concurrent writer (installer race, second instance starting
        // while the first persists) can hand us a half-written JSON: retry
        // briefly before falling back to defaults, so one unlucky read
        // never nukes the user's accounts.
        let mut file = None;
        for _ in 0..4 {
            let text = std::fs::read_to_string(&path).ok();
            match text {
                Some(text) => {
                    // Windows editors often emit UTF-8 with BOM; strip it.
                    let clean = text.trim_start_matches('\u{feff}');
                    match serde_json::from_str::<Value>(clean) {
                        Ok(mut value) => {
                            // v1.x configs carried the port inside "api";
                            // the section is gone now, the port carries
                            // over once so nobody loses their setup.
                            if value.get("port").is_none() {
                                if let Some(legacy_port) =
                                    value.get("api").and_then(|api| api.get("port")).cloned()
                                {
                                    value["port"] = legacy_port;
                                }
                            }
                            file = serde_json::from_value::<ConfigFile>(value).ok();
                            break;
                        }
                        Err(_) => std::thread::sleep(std::time::Duration::from_millis(60)),
                    }
                }
                None => break, // no file yet: genuine first run
            }
        }
        let file = file.unwrap_or_default();
        let clean = ConfigFile {
            port: if api_port_ok(file.port) {
                file.port
            } else {
                DEFAULT_API_PORT
            },
            accounts: sanitize_accounts(file.accounts),
            prefs: file.prefs.sanitized(),
        };
        let state = Self {
            data_dir: data_dir.to_path_buf(),
            inner: std::sync::Arc::new(RwLock::new(clean)),
        };
        // Make sure the file exists with all currently valid fields.
        let _ = state.persist_now();
        state
    }

    pub fn data_dir(&self) -> PathBuf {
        self.data_dir.clone()
    }

    pub fn port(&self) -> u16 {
        self.read(|file| file.port).unwrap_or(DEFAULT_API_PORT)
    }

    pub fn current_accounts(&self) -> Vec<AccountConfig> {
        self.read(|file| file.accounts.clone()).unwrap_or_default()
    }

    pub fn current_prefs(&self) -> AppPrefs {
        self.read(|file| file.prefs.clone())
            .unwrap_or_else(|_| AppPrefs::default())
    }

    /// Replace the app preferences (sanitized) and persist to disk.
    pub fn update_prefs(&self, prefs: AppPrefs) -> Result<(), String> {
        self.write(|file| {
            file.prefs = prefs.sanitized();
            self.persist_locked(file)
        })
    }

    pub fn account(&self, id: &str) -> Option<AccountConfig> {
        self.read(|file| {
            file.accounts
                .iter()
                .find(|account| account.id == id)
                .cloned()
        })
        .ok()
        .flatten()
    }

    /// First enabled account, or the first account when none is enabled.
    pub fn primary_account(&self) -> Option<AccountConfig> {
        let accounts = self.current_accounts();
        accounts
            .iter()
            .find(|account| account.enabled)
            .or_else(|| accounts.first())
            .cloned()
    }

    /// Replace the account registry (sanitized) and persist to disk.
    pub fn update_accounts(&self, accounts: Vec<AccountConfig>) -> Result<(), String> {
        self.write(|file| {
            file.accounts = sanitize_accounts(accounts);
            self.persist_locked(file)
        })
    }

    fn persist_now(&self) -> Result<(), String> {
        self.write(|file| self.persist_locked(file))
    }

    fn read<R>(&self, extract: impl Fn(&ConfigFile) -> R) -> Result<R, String>
    where
        R: Default,
    {
        let file = self
            .inner
            .read()
            .map_err(|_| "settings needed".to_string())?;
        Ok(extract(&file))
    }

    fn write<R>(
        &self,
        apply: impl FnOnce(&mut ConfigFile) -> Result<R, String>,
    ) -> Result<R, String> {
        let mut file = self
            .inner
            .write()
            .map_err(|_| "settings lock poisoned".to_string())?;
        apply(&mut file)
    }

    fn persist_locked(&self, file: &ConfigFile) -> Result<(), String> {
        if !self.data_dir.exists() {
            let _ = std::fs::create_dir_all(&self.data_dir);
        }
        let path = self.data_dir.join("config.json");
        serde_json::to_writer_pretty(
            std::io::BufWriter::new(std::fs::File::create(path).map_err(|err| err.to_string())?),
            file,
        )
        .map_err(|err| err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_default_port_and_no_accounts() {
        // Virgin installs: shell port at the default, ZERO accounts (the
        // shell auto-creates a fresh "Account 1" on the next open, never
        // a legacy "default").
        let config = ConfigFile::default();
        assert_eq!(config.port, DEFAULT_API_PORT);
        assert_eq!(config.accounts.len(), 0);
    }

    #[test]
    fn legacy_api_port_migrates_to_the_top_level_key() {
        // v1.x payloads nested the port under "api"; the port carries
        // over so a user-chosen port survives the automation removal.
        // (The exact move happens inside SettingsState::load; its core
        // step is mirrored here.)
        let mut value = serde_json::json!({
            "api": { "enabled": true, "port": 40501 },
            "accounts": [],
            "prefs": {}
        });
        if value.get("port").is_none() {
            if let Some(legacy_port) = value.get("api").and_then(|api| api.get("port")).cloned() {
                value["port"] = legacy_port;
            }
        }
        let config = serde_json::from_value::<ConfigFile>(value).expect("migrated config");
        assert_eq!(config.port, 40501);
    }

    #[test]
    fn sanitize_accounts_dedupes_and_repairs() {
        let raw = vec![
            AccountConfig {
                id: "personal!".into(),
                label: String::new(),
                enabled: true,
                service: " ".into(),
            },
            AccountConfig {
                id: "work".into(),
                label: "Work".into(),
                enabled: false,
                service: "whatsapp".into(),
            },
            AccountConfig {
                id: String::new(),
                label: "dropped".into(),
                enabled: true,
                service: "whatsapp".into(),
            },
        ];
        let clean = sanitize_accounts(raw);
        assert_eq!(
            clean
                .iter()
                .map(|account| account.id.clone())
                .collect::<Vec<_>>(),
            vec!["personal".to_string(), "work".to_string()]
        );
        assert_eq!(clean[0].label, "personal");
        assert_eq!(clean[1].label, "Work");
    }

    #[test]
    fn sanitize_accounts_allows_empty_registry() {
        // The user may remove every account; the shell auto-creates a fresh
        // one on the next open instead of resurrecting a zombie default.
        let clean = sanitize_accounts(vec![]);
        assert_eq!(clean.len(), 0);
    }

    #[test]
    fn service_names_are_whitelisted_with_whatsapp_fallback() {
        assert_eq!(sanitize_service("  FACEBOOK "), "facebook");
        assert_eq!(sanitize_service("whatsapp"), "whatsapp");
        // Unknown/legacy/empty names degrade to WhatsApp (never break).
        assert_eq!(sanitize_service(""), "whatsapp");
        assert_eq!(sanitize_service("skype"), "whatsapp");
    }

    #[test]
    fn sanitize_accounts_normalizes_service() {
        let clean = sanitize_accounts(vec![AccountConfig {
            id: "account-9".to_string(),
            label: "FB".to_string(),
            enabled: true,
            // A typoed future service name must not wedge the config.
            service: "facebookmessenger".to_string(),
        }]);
        assert_eq!(clean[0].service, "whatsapp");
    }
}
