//! Network guard + transparency lens for the embedded chat webviews.
//!
//! The chats run REAL pages served by WhatsApp/Messenger/Instagram: those
//! pages legitimately need their own services, but they should not be able
//! to light beacons to third-party tracking clouds from this machine. Each
//! account webview gets a WebView2-level REQUEST LENS: every request is
//! seen, recorded (host + verdict, for the drawer's security panel) and
//! decided here: requests to the curated third-party tracker/ad list are
//! answered with an empty 403 BEFORE they leave the machine; everything
//! else passes untouched (first-party traffic: login, media, calls).
//!
//! Honest limits (the philosophy needs the boundaries stated, not implied):
//!   - The chat services themselves can still log what their own pages do;
//!     first-party telemetry is invisible to this gate by design (blocking
//!     THEIR domains would break login, QR, media, calls).
//!   - Any site can learn the IP that dials it; network-level anonymity
//!     belongs to an outer VPN/proxy, not to a webview filter.
//!   - The list is curated, not magical: evolvers win temporarily against
//!     static deny lists; the allow-list mode is the strict future step.

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

/// Third-party tracking/ad hosts blocked for account webviews. Curated
/// candidates for what chat surfaces NEVER need: analytics beacons, ad
/// tech, cross-site profiling. `meta`-network first-party hosts are
/// deliberately ABSENT here (login + QR + media + calls ride them); the
/// day one proves itself hostile it moves in with a working chat as proof.
pub const TRACKER_HOSTS: [&str; 19] = [
    "google-analytics.com",
    "googletagmanager.com",
    "googleadservices.com",
    "googlesyndication.com",
    "doubleclick.net",
    "scorecardresearch.com",
    "quantserve.com",
    "hotjar.com",
    "hotjar.io",
    "criteo.com",
    "criteo.net",
    "taboola.com",
    "outbrain.com",
    "amplitude.com",
    "mixpanel.com",
    "segment.io",
    "app-measurement.com",
    "crashlytics.com",
    "sentry-cdn.com",
];

/// Suffix-match the curated blocklist (bare host or subdomain; a
/// lookalike that merely ENDS with the name stays legal).
pub(crate) fn is_tracker_host(host: &str) -> bool {
    let host = host.trim().trim_end_matches('.').to_ascii_lowercase();
    TRACKER_HOSTS
        .iter()
        .any(|suffix| host == *suffix || host.ends_with(&format!(".{suffix}")))
}

/// The transparency lens memory: per webview label, per host: request
/// count, blocked count, last seen (unix seconds). Capped so a hostile
/// page cannot grow it unboundedly (the cap only sheds RARE new hosts;
/// chatter keeps counting into its existing bucket).
struct HostTally {
    requests: u64,
    blocked: u64,
    last_seen: u64,
}

#[derive(Default)]
struct TrafficLedger {
    hosts: HashMap<String, HostTally>,
    total: u64,
}

static TRAFFIC: OnceLock<Mutex<HashMap<String, TrafficLedger>>> = OnceLock::new();

const LEDGER_HOST_CAP: usize = 256;

fn traffic_ledger() -> &'static Mutex<HashMap<String, TrafficLedger>> {
    TRAFFIC.get_or_init(|| Mutex::new(HashMap::new()))
}

fn epoch_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or(0)
}

/// Record one request verdict (host + was it blocked?) into the ledger.
/// Cheap: string ops only, called from the WebView2 event handler.
fn record_host(label: &str, host: &str, blocked: bool) {
    let mut ledger_map = traffic_ledger()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let ledger = ledger_map.entry(label.to_string()).or_default();
    ledger.total += 1;
    if let Some(tally) = ledger.hosts.get_mut(host) {
        tally.requests += 1;
        if blocked {
            tally.blocked += 1;
        }
        tally.last_seen = epoch_now();
        return;
    }
    // New host over the cap: keep the memory bounded (honest visibility:
    // the drawer still shows the count of shed requests in the total).
    if ledger.hosts.len() >= LEDGER_HOST_CAP {
        return;
    }
    ledger.hosts.insert(
        host.to_string(),
        HostTally {
            requests: 1,
            blocked: u64::from(blocked),
            last_seen: epoch_now(),
        },
    );
}

/// The drawer-facing snapshot: per account, its hosts sorted by volume
/// with request/blocked counters. Used by the security panel to make the
/// promises OBSERVABLE (the user sees the page's own hosts and the
/// tracker rows that never left the machine).
pub fn snapshot() -> serde_json::Value {
    let ledger_map = traffic_ledger()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut accounts: Vec<serde_json::Value> = Vec::new();
    for (label, ledger) in ledger_map.iter() {
        let mut hosts: Vec<serde_json::Value> = ledger
            .hosts
            .iter()
            .map(|(host, tally)| {
                serde_json::json!({
                    "host": host,
                    "requests": tally.requests,
                    "blocked": tally.blocked,
                    "last_seen": tally.last_seen,
                })
            })
            .collect();
        hosts.sort_by(|a, b| {
            let a_requests = a["requests"].as_u64().unwrap_or(0);
            let b_requests = b["requests"].as_u64().unwrap_or(0);
            b_requests.cmp(&a_requests)
        });
        hosts.truncate(LEDGER_HOST_CAP.min(200));
        accounts.push(serde_json::json!({
            "account": label,
            "total_requests": ledger.total,
            "hosts": hosts,
        }));
    }
    // Stable order for the UI: by total volume desc.
    accounts.sort_by(|a, b| {
        let a_total = a["total_requests"].as_u64().unwrap_or(0);
        let b_total = b["total_requests"].as_u64().unwrap_or(0);
        b_total.cmp(&a_total)
    });
    serde_json::json!({ "ok": true, "accounts": accounts })
}

#[cfg(windows)]
/// Labels already carrying the guard (per webview, once per session;
/// re-created webviews get a fresh entry automatically).
static INSTALL_LOG: Mutex<Option<std::collections::HashSet<String>>> = Mutex::new(None);

/// Install the lens on one WebView2 surface (account webviews). Once per
/// label; errors degrade silently (a runtime without the interface keeps
/// a working chat, honestly minus this one protection).
#[cfg(windows)]
pub fn install(label: &str, webview: &tauri::Webview<tauri::Wry>) {
    {
        let mut log = INSTALL_LOG
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        log.get_or_insert_with(Default::default);
        if let Some(set) = log.as_mut() {
            if !set.insert(label.to_string()) {
                return; // already armed this session
            }
        }
    }
    let label = label.to_string();
    let _ = webview.with_webview(move |platform| unsafe {
        // Runs on the main thread: WebView2 COM objects are STA-bound.
        let Ok(core) = platform.controller().CoreWebView2() else {
            return;
        };
        use windows_core::Interface as _;
        // ICoreWebView2_2 carries both the filter setter and our
        // Environment handle (needed to fabricate the 403 response).
        let Ok(core2) = core
            .cast::<webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_2>()
        else {
            return;
        };
        let Ok(env) = core2.Environment() else {
            return;
        };
        for pattern in ["https://*/*", "http://*/*", "ws://*/*", "wss://*/*"] {
            let wide = windows_core::HSTRING::from(pattern);
            // Duplicate filter adds fail; that is fine - the pattern is
            // already armed, which is the whole point.
            let _ = core.AddWebResourceRequestedFilter(
                &wide,
                webview2_com::Microsoft::Web::WebView2::Win32::COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL,
            );
        }
        let env_for_handler = env;
        let label_for_handler = label.clone();
        let handler = webview2_com::WebResourceRequestedEventHandler::create(Box::new(
            move |_core, args| {
                // The catch-all filters route EVERY request here: this is
                // the transparency lens AND the tracker gate in one place.
                if let Some(args) = args {
                    if let Some((host, blocked)) = extract_host(&args) {
                        record_host(&label_for_handler, &host, blocked);
                        if blocked {
                            if let Ok(response) = env_for_handler.CreateWebResourceResponse(
                                None,
                                403,
                                &windows_core::HSTRING::from("Corvid-blocked"),
                                &windows_core::HSTRING::from("Content-Type: text/plain\r\n"),
                            ) {
                                let _ = args.SetResponse(&response);
                            }
                        }
                    }
                }
                Ok(())
            },
        ));
        let mut token = 0i64;
        let _ = core.add_WebResourceRequested(&handler, &mut token);
        eprintln!("zapdude: netguard armed for {label}");
    });
}

#[cfg(not(windows))]
pub fn install(_label: &str, _webview: &tauri::Webview<tauri::Wry>) {}

#[cfg(windows)]
/// Pull (host, is the curated tracker?) from one WebResourceRequested
/// event. None = nothing logged (opaque/broken forms stay out of the
/// ledger rather than inventing a host).
unsafe fn extract_host(
    args: &webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2WebResourceRequestedEventArgs,
) -> Option<(String, bool)> {
    let request = args.Request().ok()?;
    let mut uri_ptr = windows_core::PWSTR::null();
    request.Uri(&mut uri_ptr).ok()?;
    let text = webview2_com::CoTaskMemPWSTR::from(uri_ptr).to_string();
    let parsed = url::Url::parse(&text).ok()?;
    let host = parsed.host_str()?.trim().to_ascii_lowercase();
    if host.is_empty() {
        return None;
    }
    let blocked = is_tracker_host(&host);
    Some((host, blocked))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracker_matching_is_suffix_true_lookalike_false() {
        assert!(is_tracker_host("doubleclick.net"));
        assert!(is_tracker_host("ads.DoubleClick.NET"));
        assert!(is_tracker_host("pagead2.googlesyndication.com"));
        // The chat surfaces are NOT trackers.
        assert!(!is_tracker_host("web.whatsapp.com"));
        // A lookalike that merely ENDS with a tracker name stays legal.
        assert!(!is_tracker_host("notdoubleclick.net"));
        assert!(!is_tracker_host("doubleclick.net.evil.example"));
        // Media CDNs stay allowed on purpose.
        assert!(TRACKER_HOSTS.iter().all(|host| !host.contains("whatsapp")
            && !host.contains("facebook")
            && !host.contains("messenger")
            && !host.contains("instagram")
            && !host.contains("meta")));
    }

    #[test]
    fn ledger_survives_the_cap_and_reports() {
        // The sill police: a hostile page cannot grow the memory term yet.
        let label = "test-label";
        for index in 0..600 {
            record_host(label, &format!("host-{index}.example"), index % 2 == 0);
        }
        let sol = snapshot();
        let accounts = sol["accounts"].as_array().unwrap();
        let mine = accounts
            .iter()
            .find(|account| account["account"] == label)
            .expect("test label must be snapshotted");
        // Hard cap + the shed-away new hosts (the remaining ones just do
        // not get rows; the total keeps counting requests, not entries).
        assert!(mine["hosts"].as_array().unwrap().len() <= 256);
    }
}
