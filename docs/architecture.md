# Architecture - Corvid

Living document. Describes the components and design decisions of the
project (original code by da1fs; the starting design reference was the
ZapZap concept by Rafael Tosta, see NOTICE.md). The internal identifiers
(binary name `zapdude`, the `app.zapzap.desktop` app id, the Windows
Run key value) keep the old spelling on purpose until the rename cut;
see docs/corvid-rename-checklist.md.

## Overview

```
+----------------------------------------------------------------------+
|  zapdude.exe (Tauri 2 + WebView2)                                    |
|                                                                      |
|  +----------------------------------------------------------------+  |
|  | main window (single window shell)                              |  |
|  |                                                                |  |
|  |  [ rail 76px ]   [ welcome / account webviews / drawer ]       |  |
|  |                                                                |  |
|  |  rail.html      : logo (toggles drawer), avatars, "+"          |  |
|  |  account-<id>   : one chat web per account, isolated profile  |  |
|  |  drawer         : full screen options panel, created on demand |  |
|  |  (on-demand children: picker strip, boot curtain, update card) |  |
|  |  welcome        : branded backdrop / empty-state destination   |  |
|  +----------------------------------------------------------------+  |
|                                                                      |
|  [ tray ] Show <account>, Hide all windows, Quit Corvid              |
|                                                                      |
|  [ loopback shell server ] rail + drawer + welcome + pages UI      |
+----------------------------------------------------------------------+
```

Call pop-outs open as separate windows. They share the account's WebView2
environment, so the session and cookies are the same one the page uses.

## Components

| Piece | Role |
|---|---|
| `src/main.rs` | startup order: single instance, server, shell, tray, first account |
| `src/window.rs` | the webview shell: rail + parked accounts + drawer + tray + popouts + permission shield |
| `src/config.rs` | settings on disk: shell `port` + `prefs` + `accounts`, sanitized, BOM tolerant |
| `src/api/link.rs` | the slim page link (`window.__zapdudeLinkV1`): beacon poller, telemetry ingest, state probe, call-pill recovery |
| `src/api/server.rs` | axum 0.8 loopback server: shell UI router, management endpoints, loopback origin guard (no automation surface) |
| `src/netguard.rs` | per-webview curated tracker list (empty 403, no round trip) + the traffic ledger behind `/api/traffic` |
| `src/notifier.rs` | branded native Windows toasts (contact avatar, click-through) |
| `src/updater.rs` | GitHub Releases check, verified download, install script, ask/auto/off policy |
| `src/browsers.rs` | StartMenuInternet enumeration (chat links open in the user's own browser) |
| `scripts/client_hints_fix.js` | JS side Chrome identity (Sec-CH-UA is HTTP layer and not rewritable from the host) |
| `scripts/page_link.js` | resident per document: `window.__zapdudeLinkV1` (outbound envelopes, state probe, call-pill recovery) |
| `scripts/notification_hook.js` | Notification capture re-rendered by the shell as native toasts |
| `scripts/unread_watcher.js` | page title sampling for the rail badges |
| `scripts/hide_download_banners.js` | rule-based hider for the "download the app" promos with guardrails |
| `ui/rail.html` | left rail: logo toggles the drawer, avatars with state dots and unread badges, right click delete, "+" |
| `ui/drawer.html` | full screen options: preferences, security lens + permission shield, account rename, API settings, agent endpoints, About |
| `ui/welcome.html` | branded backdrop and the empty-state destination when no accounts remain |
| `ui/picker_strip.html` | the "+" service menu (six tiles) |
| `ui/boot_curtain.html` | service-branded loading cover between the click and the chat |
| `ui/update_nudge.html` | centered "new version is ready" card (ask policy) |
| `windows/installer-hooks.nsh` | NSIS hook that wipes `%APPDATA%\app.zapzap.desktop` on uninstall |

## Startup order (critical)

1. Single instance: Windows mutex `Local\app.zapzap.desktop.single`; a
   second instance exits immediately.
2. The loopback server starts BEFORE any webview exists (the UI is served
   by the same server, and a UI that boots before the server returns
   broken pages).
3. `create_shell`: rail + welcome webviews.
4. Accounts: the first account opens right away (or "Account 1" is
   created on a virgin install with no config). Every other account
   creates its webview lazily on first open.
5. Tray icon and menu.

## Shells and visibility model

- One Tauri window (`main`, feature `unstable`) hosts several child
  webviews. The ACTIVE content webview sits next to the rail at
  `(RAIL_WIDTH, 0)` at full size; every other content webview is parked
  outside the client area (x = -24000 logical) AT FULL SIZE.
- Why full-size parking: a webview born hidden (or tiny) never paints
  (WebView2 composition pipeline), and shrinking parked webviews would
  force WhatsApp to re-render on every switch. Position-only moves keep
  switching instant (measured under 350 ms full repaint, switches feel
  immediate).
- A geometry watchdog re-aligns every webview every 2 s (plus resize and
  scale events) so the four corners always hug the client area after any
  DPI or resize dance.
- The drawer is created closed and lazy. Each open DESTROYS and re-opens
  it so it always renders above the account webviews (z-order) with
  fresh state. Closing parks it with the same geometry watchdog path
  (instant).

## Page link flow (slim, telemetry-only)

1. The link: each account webview gets a deferred, idempotent
   `page_link.js` install (flag `window.__zapdudeLinkV1`) from the
   beacon poller, re-armed after every page load/reload.
2. Outbound events: the notification capture and the unread watcher
   push payloads (`pushEvent`) or fall back to
   `location.hash = #__zapdude=<payload>`; the host poller consumes
   and clears them.
3. Direct delivery: notification payloads call
   `notifier::deliver_page_notification` straight to the toast
   pipeline (no event ring since v2.0.0); unread counts land in the
   server's badge store the rail polls through `/api/accounts`.
4. Page state: the rail dots and boot curtain resolve through the
   guarded `stateInfo` probe (composer/QR/password presence only)
   evaluated via `eval_with_callback` with stale-while-revalidate.
5. Call safety: after a call popup dies, one recovery gesture clicks
   the leftover pill and reloads the page as the failsafe (session
   safe: audio dies, the pill clears). Serves human-initiated calls.
6. Never break: every page interaction degrades (missing badge, grey
   dot) instead of failing the shell; links are replaced when a
   webview is re-created and cancelled on close.

## Multi account model

- Each account runs its own WebView2 profile at
  `%APPDATA%\app.zapzap.desktop\webview\<account-id>` so sessions,
  cookies and storage never mix.
- Each account carries a `service` (config, `AccountConfig.service`):
  which REAL web chat the view embeds - `whatsapp` -> web.whatsapp.com,
  `facebook` -> messenger.com (chat-only surface; the Facebook login,
  including 2FA, happens inside the embedded view, mirroring how the
  WhatsApp QR works), `instagram` -> instagram.com/direct, `x` ->
  x.com/messages, `tiktok` -> tiktok.com/messages (experimental) and
  `twitch` -> the logged-in site (whispers have no standalone chat
  page). The page is always the integration target; the page link is
  service-agnostic and telemetry-only (see docs/RED-LINES.md), so a
  service's internals never appear in the shell's data plane.
- Deleting an account removes its webview AND wipes its profile folder
  (best effort); the shell falls back to the next account or the
  welcome screen (the welcome screen carries explicit add buttons per
  service, and the rail's "+" opens the mini service-picker window;
  nothing auto-creates empty accounts).
- Renames are metadata only: ids are stable forever.

## Tray and window behavior

- Closing the main window hides it (app stays alive in the tray).
- Tray menu rebuilds on account add/remove/rename so entries are always
  current; left click on the tray icon shows and focuses the main window.
- "Quit Zapdude" stops the loopback server first, then exits.

## Security posture

- The shell server listens on 127.0.0.1, ALWAYS. There is no key auth,
  no rate limiting and no webhook: since v2.0.0 the server is the
  shell's own UI plus management tier for its pages (docs/RED-LINES.md).
- Requests that carry a browser `Sec-Fetch-Site: cross-site` marker, or
  an `Origin` outside the shell's own loopback address, are refused by
  middleware; see the guard in `src/api/server.rs`.
- The account webviews refuse navigation outside their own chat site
  (links and popups go to the user's browser), the netguard answers the
  curated tracker-host requests locally with an empty 403, and the
  permission shield decides mic/camera/screen/clipboard access.
  Honest limits are documented in-module (netguard) and in the drawer's
  Security section: the chat services still see what their own pages do.
- The shell itself makes no third-party calls: no telemetry, no
  analytics, no global listeners. The account pages talk to their chat
  services, exactly as any browser would.
- The updater talks only to GitHub Releases (read + download) and runs
  under the policy the user chooses (ask/auto/off).

## Testing and build

- `cargo test`: unit tests for the page link, the config migration, the
  netguard, the notifier text helpers, the updater and the browser
  handling (the current count lives in the source modules).
  The webview shell is verified manually with real
  window captures (PrintWindow + PW_RENDERFULLCONTENT, see
  docs/SESSION-STATUS.md).
- Windows build: `tools\tauri.exe build --bundles nsis` or
  `cargo install tauri-cli --locked` and `cargo tauri build --bundles nsis`.
- Debug bisection: env var `ZAPDUKE_WEBVIEW_OPTS` toggles webview options
  (UA, scripts, data dir, args, permissions) to isolate the cause when
  WebView2 misbehaves.
