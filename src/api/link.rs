//! The slim page link: per-account resident transport between the shell
//! and the embedded chat pages.
//!
//! Remaining duties after the local automation API removal (v2.0.0):
//!   - deferred install and re-install of the page scripts (the page link
//!     helper, the Notification capture, the unread watcher and friends),
//!   - the beacon transport (`#__zapdude=<payload>`, page-to-host events),
//!   - unread badge store ingest (server::unread_store_set) for the rail,
//!   - Notification payloads routed DIRECTLY to the toast pipeline (no
//!     event ring, no SSE, no webhook: that surface is gone),
//!   - the page state probe behind the rail dots and the boot curtain,
//!   - the stuck-call-pill recovery gesture (human-initiated calls only).
//!
//! Everything else is gone with v2.0.0 by project red lines (no message
//! surface, no chat reads, no call starting, no key auth): see
//! docs/RED-LINES.md. Every page interaction stays defensive on purpose:
//! chat-web markup changes degrade the helper, never the shell.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use serde_json::{json, Value};
use tauri::webview::Webview;
use tauri::Wry;
use tokio::sync::oneshot;

/// Chamfer between beacon polls (event transport only).
const POLL_INTERVAL_MS: u64 = 30;

/// Install the deferred page scripts once every N beacon ticks (~3 s at
/// the 30 ms poll interval). The check itself is a no-op once installed.
const DEFER_INSTALL_EVERY_TICKS: u64 = 100;

/// Maximum time a page evaluation may wait for the page to answer.
const EVAL_TIMEOUT_MS: u64 = 15_000;

static PAGE_LINKS: OnceLock<Mutex<HashMap<String, Arc<PageLink>>>> = OnceLock::new();

/// Minimal page-side error shape for the slim link (the old automation
/// ApiError carried the REST contract; this one just needs a stable
/// `code` so state derivation can tell "still loading" from "broken").
#[derive(Debug, Clone)]
pub struct PageError {
    pub code: String,
    pub status: u16,
    pub message: String,
}

impl PageError {
    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            code: "page_unavailable".to_string(),
            status: 503,
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            code: "internal_error".to_string(),
            status: 500,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            code: "bad_request".to_string(),
            status: 400,
            message: message.into(),
        }
    }
}

/// Resolve the link for a given account id (or the default account).
pub fn page_for(account: Option<&str>) -> Option<Arc<PageLink>> {
    let registry = PAGE_LINKS.get()?;
    let map = registry.lock().ok()?;
    match account {
        Some(id) => map.get(id).cloned(),
        None => {
            if let Some(default_page) = map.get("default") {
                return Some(default_page.clone());
            }
            let mut ids: Vec<&String> = map.keys().collect();
            ids.sort();
            ids.first().and_then(|first| map.get(*first)).cloned()
        }
    }
}

/// Create the link for an account webview and start its polling loop.
/// Idempotent: the FIRST install for an account wins (the poller thread
/// is only spawned once per account).
pub fn install(account_id: &str, window: Webview<Wry>) -> Arc<PageLink> {
    let registry = PAGE_LINKS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(existing) = registry
        .lock()
        .ok()
        .and_then(|map| map.get(account_id).cloned())
    {
        return existing;
    }
    let service = service_for_account(account_id);
    let link = Arc::new(PageLink {
        account_id: account_id.to_string(),
        service,
        window,
        cancelled: std::sync::atomic::AtomicBool::new(false),
    });
    if let Ok(mut map) = registry.lock() {
        map.insert(account_id.to_string(), link.clone());
    }
    let poller = link.clone();
    let _ = std::thread::Builder::new()
        .name(format!("zapdude-beacon-{account_id}"))
        .spawn(move || {
            // The deferred scripts (page link helper, Notification
            // capture, unread watcher, call helpers) are (re-)installed
            // from here: the page may load/reload several times (QR
            // refresh, reconnects), and a light idempotent check every
            // DEFER_INSTALL_EVERY_TICKS keeps them present without
            // touching the page-start injection (which must stay small;
            // see `startup_scripts`). Each script is guarded by its OWN
            // flag and try/catch so one failure can never silence the
            // other (that is exactly how the banner hider got lost).
            let install_expressions: Vec<String> = crate::window::deferred_scripts(Some(
                &poller.service,
            ))
                .into_iter()
                .map(|(flag, script)| {
                    format!(
                        "if (window === window.top && !window.{flag}) {{ try {{ {script} }} catch (_e) {{}} }}",
                        flag = flag,
                        script = script
                    )
                })
                .collect();
            let mut ticks: u64 = 0;
            loop {
                std::thread::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS));
                // The webview vanished (closed/recreated): stop polling a
                // dead target and release the thread.
                if poller.is_cancelled() {
                    break;
                }
                ticks = ticks.wrapping_add(1);
                if ticks % DEFER_INSTALL_EVERY_TICKS == 0 {
                    for expression in &install_expressions {
                        let _ = poller.window.eval(expression);
                    }
                }
                if let Err(_failure) = poller.poll_beacons() {
                    // Temporary window/state errors are ignored; the loop
                    // keeps retrying, matching the "never break" behavior.
                }
            }
        });
    link
}

/// Replace the link of an account whose webview was RECREATED (close +
/// open flows). The old link is cancelled (its poller exits) and a fresh
/// one is installed on the new webview. Without this, the first-ever
/// link for an account would keep pointing at a destroyed webview and
/// every evaluation would run into eval timeouts (seen live).
pub fn replace(account_id: &str, window: Webview<Wry>) -> Arc<PageLink> {
    remove(account_id);
    install(account_id, window)
}

/// Remove the link of a closed/recreated account and cancel its poller.
pub fn remove(account_id: &str) {
    let registry = PAGE_LINKS.get();
    let Some(registry) = registry else {
        return;
    };
    if let Ok(mut map) = registry.lock() {
        if let Some(link) = map.remove(account_id) {
            link.cancel();
        }
    }
    // The account's page sources stopped feeding the badge store: drop
    // its last count so a removed account never leaves a stale bubble.
    super::server::unread_store_clear(account_id);
}

/// Resolve an account's service defensively at link-install time
/// (unknown id / closed registry / missing settings -> legacy WhatsApp).
fn service_for_account(account_id: &str) -> String {
    crate::SHARED
        .get()
        .and_then(|shared| shared.settings.account(account_id))
        .map(|account| account.service)
        .unwrap_or_else(|| crate::config::SERVICE_WHATSAPP.to_string())
}

fn percent_decode(raw: &str) -> String {
    let input = raw.as_bytes();
    let mut out = Vec::with_capacity(input.len());
    let mut index = 0;
    while index < input.len() {
        let byte = input[index];
        if byte == b'%' && index + 2 < input.len() {
            let hex_high = (input[index + 1] as char).to_digit(16);
            let hex_low = (input[index + 2] as char).to_digit(16);
            if let (Some(high), Some(low)) = (hex_high, hex_low) {
                out.push((high * 16 + low) as u8);
                index += 3;
                continue;
            }
        }
        out.push(byte);
        index += 1;
    }
    String::from_utf8(out).unwrap_or_default()
}

/// Normalize a page payload into an unread badge count. Sources:
/// `unread` (the shared title watcher: `{kind, count}`) and `title_badge`
/// (legacy Messenger bridge payloads: `{kind, badge}`). Anything else.
fn unread_count_from(payload: &Value) -> Option<u32> {
    let kind = payload.get("kind").and_then(Value::as_str)?;
    if kind != "unread" && kind != "title_badge" {
        return None;
    }
    let raw = payload
        .get("count")
        .and_then(Value::as_u64)
        .or_else(|| payload.get("badge").and_then(Value::as_u64))?;
    Some((raw.min(999)) as u32)
}

pub struct PageLink {
    pub account_id: String,
    /// Which chat web this account embeds ("whatsapp" legacy default);
    /// decided at link install time and attached to every notification
    /// payload so the shell never has to guess.
    pub service: String,
    window: Webview<Wry>,
    /// Set when the webview this link was installed on got destroyed
    /// (the account was closed or the webview recreated). The beacon
    /// poller exits instead of hammering a dead target forever.
    cancelled: std::sync::atomic::AtomicBool,
}

impl PageLink {
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn cancel(&self) {
        self.cancelled
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Poll the URL-hash beacon for page-to-host event envelopes.
    fn poll_beacons(&self) -> Result<(), String> {
        let url = self.window.url().map_err(|err| err.to_string())?;
        const BEACON: &str = "__zapdude=";
        let Some(fragment) = url.fragment() else {
            return Ok(());
        };
        if !fragment.starts_with(BEACON) {
            return Ok(());
        }
        let encoded = &fragment[BEACON.len()..];
        let decoded = percent_decode(encoded);
        if let Err(_clear) = self.window.eval("location.replace('#')") {
            // ignore; a later poll clears it
        }
        let Ok(envelope) = serde_json::from_str::<Value>(&decoded) else {
            return Ok(());
        };
        let id = envelope.get("id").and_then(Value::as_str).unwrap_or("");
        let payload = envelope.get("payload").cloned().unwrap_or(json!(null));
        // The badge store reads FIRST: the delivery below CONSUMES the
        // payload value, so the unread normalization must complete
        // before ownership moves.
        let unread_count = unread_count_from(&payload);
        if let Some(count) = unread_count {
            super::server::unread_store_set(&self.account_id, count);
        }
        if id.starts_with("evt_") {
            // Notifications travel straight to the toast pipeline. Every
            // other payload kind used to feed the API event ring, which
            // no longer exists: they are dropped here on purpose (the
            // red lines keep the page surface telemetry-only).
            let is_notification = payload
                .get("kind")
                .and_then(Value::as_str)
                .map(|kind| kind == "notification" || kind == "notification_closed")
                .unwrap_or(false);
            if is_notification {
                crate::notifier::deliver_page_notification(
                    &self.account_id.clone(),
                    &self.service.clone(),
                    payload,
                );
            }
        }
        Ok(())
    }

    /// Evaluate a page script and await its JSON-serialized result.
    /// Shared by every caller (page state probe, notification click,
    /// call recovery): WebView2's double-encoded string quirk and the
    /// missing-helper handling live here ONCE.
    pub async fn eval_in(window: Webview<Wry>, script: String) -> Result<Value, PageError> {
        // The callback contract is `Fn` (may fire once per evaluation),
        // so the one-shot sender is parked behind interior mutability.
        let (sender, receiver) = oneshot::channel::<String>();
        let slot = Arc::new(Mutex::new(Some(sender)));
        let slot_for_callback = Arc::clone(&slot);
        let deliver = move |result_json: String| {
            if let Ok(mut guard) = slot_for_callback.lock() {
                if let Some(one_shot) = guard.take() {
                    let _ = one_shot.send(result_json);
                }
            }
        };
        if let Err(eval_failure) = window.eval_with_callback(script, deliver) {
            return Err(PageError::unavailable(format!(
                "page evaluation failed: {}",
                eval_failure
            )));
        }
        let waited =
            tokio::time::timeout(std::time::Duration::from_millis(EVAL_TIMEOUT_MS), receiver).await;
        match waited {
            Err(_timeout) => Err(PageError::unavailable("page did not respond in time")),
            Ok(Err(_cancelled)) => Err(PageError::internal("page result channel closed")),
            Ok(Ok(result_json)) => {
                if result_json.is_empty() {
                    return Err(PageError::unavailable(
                        "page link not present on the page yet",
                    ));
                }
                let parsed = match serde_json::from_str::<Value>(&result_json) {
                    Ok(value) => value,
                    Err(_) => return Err(PageError::unavailable("page returned unreadable JSON")),
                };
                // Windows quirk: WebView2 ExecuteScript JSON-encodes the
                // value it returns. When the value type is a string (we
                // export `String(JSON.stringify(...))` results), the
                // payload arrives double-encoded and must be unwrapped
                // once more.
                let value = match &parsed {
                    Value::String(inner) if !inner.is_empty() => {
                        serde_json::from_str::<Value>(inner).unwrap_or(parsed)
                    }
                    _ => parsed,
                };
                if value.is_null() {
                    return Err(PageError::unavailable(
                        "page link not present on the page yet",
                    ));
                }
                Ok(value)
            }
        }
    }

    /// Evaluate via this link's own webview.
    pub async fn evaluate(&self, script: String) -> Result<Value, PageError> {
        Self::eval_in(self.window.clone(), script).await
    }

    /// Build the guarded page-link expression for one operation. The
    /// guard keeps every call safe against a missing helper: the answer
    /// is null instead of a page exception.
    fn bridge_expression(op: &str, args_json: &str) -> String {
        format!(
            "JSON.stringify(function(){{ try {{ if (!window.__zapdudeLinkV1 || \
             typeof window.__zapdudeLinkV1.{op} !== 'function') {{ return null; }} \
             return window.__zapdudeLinkV1.{op}({args}); }} catch (error) {{ \
             return String(error && error.message || error); }} }}())",
            op = op,
            args = args_json
        )
    }

    /// Read-only page state snapshot for the shell (rail dots, boot
    /// curtain). Deliberately shallow: presence checks only, no
    /// contacts, no conversations, no text.
    pub async fn status_snapshot(&self) -> Result<Value, PageError> {
        let info = Self::eval_in(
            self.window.clone(),
            Self::bridge_expression("stateInfo", ""),
        )
        .await?;
        Ok(json!({
            "account": self.account_id,
            "service": self.service,
            "url": self.window_url_for_status(),
            "title": info.get("title").cloned().unwrap_or(json!("")),
            "chat_list": info.get("chat_list").cloned().unwrap_or(json!(false)),
            "composer_visible": info.get("composer").cloned().unwrap_or(json!(false)),
            "qrcode_visible": info.get("qrcode").cloned().unwrap_or(json!(false)),
            // Login form (email + password) signal for the services that
            // have no QR stage - from the generic password-field probe.
            "login_form_visible": info
                .get("login_form")
                .cloned()
                .unwrap_or(json!(false)),
            // Page-reported location (location.href): reveals redirects
            // or WebView2 error pages that the nav URL alone hides.
            "page_url": info.get("url").cloned().unwrap_or(json!(null)),
        }))
    }

    fn window_url_for_status(&self) -> String {
        self.window
            .url()
            .map(|url| url.to_string())
            .unwrap_or_default()
    }
}

/// Best-effort reconciliation of an account's WhatsApp page after its
/// call popup window vanished without going through WhatsApp's own
/// end-call flow (X button / Alt+F4 bypass it: the call keeps "burning"
/// and the pill stays on screen with nothing reachable). Sequence: click
/// the leftover pill; wait a beat; if a pill still matches, reload the
/// page as the failsafe (session-safe reload, audio dies, pill clears).
/// Serves human-initiated calls only; no call starting lives here.
pub fn reconcile_after_popup_close(account_id: &str) {
    let account_id = account_id.to_string();
    let Some(link) = page_for(Some(account_id.as_str())) else {
        return;
    };
    let click_script = String::from(
        "JSON.stringify(function(){ try { if (!window.__zapdudeLinkV1 || \
         typeof window.__zapdudeLinkV1.callRecover !== 'function') { return null; } \
         return window.__zapdudeLinkV1.callRecover(false); } catch (error) { \
         return String(error && error.message || error); } }())",
    );
    let probe_script = String::from(
        "JSON.stringify(function(){ try { if (!window.__zapdudeLinkV1 || \
         typeof window.__zapdudeLinkV1.callRecover !== 'function') { return null; } \
         return window.__zapdudeLinkV1.callRecover(true); } catch (error) { \
         return String(error && error.message || error); } }())",
    );
    let reload_script = String::from("location.reload(); void 0");
    tauri::async_runtime::spawn(async move {
        let _ = link.evaluate(click_script).await;
        tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
        match link.evaluate(probe_script).await {
            Ok(probe) => {
                let stuck = probe
                    .get("in_call")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                if stuck {
                    eprintln!(
                        "zapdude: account '{}' still shows a stuck call pill; reloading the page",
                        account_id
                    );
                    let _ = link.evaluate(reload_script).await;
                }
            }
            Err(_probe_failure) => {}
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guarded_expression_targets_the_slim_link_only() {
        let script = PageLink::bridge_expression("stateInfo", "");
        assert!(script.contains("__zapdudeLinkV1"));
        assert!(script.contains("stateInfo"));
        // The old automation surface must be gone from new expressions.
        assert!(!script.contains("__zapdudeApiV1"));
    }

    #[test]
    fn percent_decode_handles_plain_and_encoded_input() {
        assert_eq!(percent_decode("plain"), "plain");
        assert_eq!(percent_decode("a%20b%21"), "a b!");
        assert_eq!(percent_decode("bad%zz"), "bad%zz");
    }

    #[test]
    fn unread_payload_normalization_is_honest() {
        assert_eq!(
            unread_count_from(&serde_json::json!({"kind": "unread", "count": 42})),
            Some(42)
        );
        assert_eq!(
            unread_count_from(&serde_json::json!({"kind": "unread", "count": 1500})),
            Some(999)
        );
        // Legacy Messenger field.
        assert_eq!(
            unread_count_from(&serde_json::json!({"kind": "title_badge", "badge": 3})),
            Some(3)
        );
        // Everything else is not a badge.
        assert_eq!(
            unread_count_from(&serde_json::json!({"kind": "notification"})),
            None
        );
    }
}
