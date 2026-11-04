# Corvid

**Corvid** is a desktop chat hub for **WhatsApp, Messenger, Instagram, X, TikTok and Twitch** (multi account, multi service) in a single window, for **Windows and macOS** (a GNU/Linux port is planned), written in **Rust** (Tauri 2 + system webviews).

Corvid started life as **Zapdude**; the rename is identity-only: the
codebase, the installed sessions and the AGPL-3.0 license carry over.

**Original project by [da1fs](https://github.com/da1fs)** (this account
was formerly ldpf96), written with AI assistance: the **OpenCode** agent
harness driving the **GLM-5.3 Flash** model (by Z.ai), with every change
reviewed and verified on real hardware by the author before it ships.
The origin story and the full credits live in [`NOTICE.md`](NOTICE.md).

The starting point was the design and behavior of
[ZapZap](https://github.com/rafatosta/zapzap) by Rafael Tosta: the early
versions deliberately tried to feel like ZapZap, and the project then
grew into something substantially different (six embedded chat
services, a permission shield, a network lens and a self-updater). No
ZapZap code ships here (different language, different stack); the
credit is given loudly because that is the honest thing to do.

License: **AGPL-3.0**.

![Windows 10/11](https://img.shields.io/badge/Windows-10%2F11-0078d7) ![macOS 13+](https://img.shields.io/badge/macOS-13%2B-999999) ![Rust](https://img.shields.io/badge/Rust-stable-dea584) ![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131) ![License](https://img.shields.io/badge/License-AGPL--3.0-blue)

## Status, read this first

Corvid is a young project maintained by one person in spare time. It is
being built in the open, which means three things at any moment:

- Things can break. The embedded pages are WhatsApp, Meta, X and TikTok,
  and they change whenever they decide to; when they do, features that
  depend on those pages can stop working until the next fix lands.
- Windows is the primary platform (per-user NSIS installer). macOS
  builds ride the release workflow, and some recent tags shipped without
  `.dmg` files; the Releases page always shows what each tag actually
  carries.
- When something breaks, an [Issue](https://github.com/da1fs/corvid/issues)
  with what you saw is enough to trigger a fix. Releases land here in
  public, and each release note states plainly what changed.

## Features

- **Multi account at its core** - a left rail with a logo, one avatar
  per account and a "+". Adding an account asks for nothing: it opens
  its chat web right away. Each account gets an **isolated webview
  profile**, so sessions never mix.
- **Six embedded chat services** - WhatsApp, Messenger, Instagram, X,
  TikTok (experimental: its own anti-automation may degrade the page
  state the shell reads) and Twitch (the logged-in site, since whispers
  have no standalone page) each anchor on their real chat surface.
- **Instant account switching** - inactive accounts stay parked at full
  size outside the window, so switching is only a position move: the
  chat does not re-render (measured: full paint in under 350 ms).
- **Branded native notifications** - messages, incoming calls and
  missed calls arrive as same-size Windows toasts with the contact's
  round avatar and name, whether the window is visible or hidden;
  clicking a toast opens the account that produced it.
- **Permission shield** - when a chat page asks for the microphone,
  camera, screen or clipboard, the answer comes from Corvid's own
  switches (Security section), not silently from the browser. Off
  means refused inside the app.
- **Network lens** - the netguard records every host each account's
  page contacts, answers known third-party tracker requests locally
  with an empty 403 (they never leave the machine), and the drawer
  renders the live ledger per account with BLOCKED badges.
- **Embeddable blocker** - a bundled, unmodified copy of uBlock Origin
  Lite mounts inside each chat profile; attribution lives in
  [`NOTICE.md`](NOTICE.md).
- **Chat links leave cleanly** - a link clicked inside a chat opens in
  the user's own browser (with its own sessions); the choice of browser
  is a preference listed from the system registry.
- **Theme, tray and badges** - full light/dark theme across the shell
  and the embedded pages, tray with per-account entries, start with
  Windows, start minimized, mobile-style unread count bubbles.

## What Corvid does NOT do

Corvid has **no automation surface**. The v2.0.0 release removed the
whole local automation API (REST + SSE + webhook for reading chats,
sending messages and starting calls) on purpose: those endpoints drove
the embedded pages from outside, and that is the one thing the chat
services' terms name outright. What Corvid ships today is browser-class
work only:

- The app loads each service's own real web page inside the user's
  logged-in session. Every click, message and call is a human action on
  the page. The user is the operator.
- The app's own page hooks serve the shell: notification capture
  (they re-render as native toasts), the unread count sampling (the
  document title the page already publishes), the page state probe that
  drives the rail dots, and one recovery gesture for a stuck call pill
  after a call popup closes the hard way. Nothing reads conversations,
  nothing sends messages, nothing starts calls.

## Terms of service reality

The six embedded services prohibit unofficial automation in their
terms, and their platforms enforce it on accounts: WhatsApp states that
unofficial clients and automation lead to account suspensions (their
help center is explicit), Meta prohibits automated collection of its
products' data while logged in, X prohibits access outside its
published interfaces and forbids scraping without written consent,
TikTok prohibits automated scripts interacting with its services, and
Twitch bans access through automated means. Corvid's v2.0.0 cut exists
exactly because of that letter: the automation endpoints are gone, not
hidden.

What remains is the wrapper territory a whole public ecosystem shares
(web-clients like ZapZap, Ferdium, Rambox and the like, none of which
has been sued), but a wrapper still sits at the edge of those terms,
because the terms are one-sided and platforms change them when they
want. If a platform decides to restrict or terminate an account, that
happens between the platform and the account holder, NOT through this
project: an Issue here cannot restore a suspended account anywhere.

Corvid is not an official product of any of these services and is not
affiliated with any of them.

## Installation

### Windows 10 / 11

Download the current installer from the
[Releases](https://github.com/da1fs/corvid/releases) page and run it. It
installs for the current user (no admin needed). Uninstalling removes
the data folder `%APPDATA%\app.zapzap.desktop`, which holds the
configuration and every account's session: copy that folder out first
if you want to keep the sessions.

Or build it yourself:

```powershell
cd zapdude-rs
.\tools\tauri.exe build --bundles nsis   # installer lands in target\release\bundle\nsis
```

### macOS

Download the `.dmg` from Releases (Apple Silicon builds are
`aarch64.dmg`, Intel Macs are `x64.dmg`; minimum **macOS 13 Ventura**).
The app is unsigned (no paid Apple signing), so the first open of each
downloaded build asks for one confirmation: either run
`xattr -dr com.apple.quarantine /Applications/Zapdude.app` in Terminal,
or use the "Open Anyway" button in System Settings, Privacy & Security.
It happens again on each new downloaded build, not once forever.

Note: some recent tags shipped Windows-only; if the newest tag has no
`.dmg`, use the last tag that does and watch for the next release.

### GNU/Linux

No packages yet (deb/AppImage are planned). The code base builds like
any Tauri project: stable Rust toolchain plus the tauri CLI.

## Configuration

`%APPDATA%\app.zapzap.desktop\config.json` (created on first run):

```json
{
  "port": 29977,
  "prefs": {
    "theme": "dark",
    "autostart": false,
    "startMinimized": false,
    "unreadBadges": true,
    "externalBrowser": "",
    "updateMode": "ask"
  },
  "accounts": [
    { "id": "work", "label": "Work", "enabled": true, "service": "whatsapp" }
  ]
}
```

- `port` is the shell's own loopback port (127.0.0.1 only, no network
  exposure). Configs written by v1.x keep working as they are: the old
  `api.port` value carries over and the removed `api.*` keys (enabled,
  apiKey, webhook) are ignored.
- The app has no network-facing interface, no API keys and no webhook
  receiver anymore; there is nothing to enable or authenticate.
- The permission shield and update policy also persist here
  (`shieldCamera`, `shieldMic`, `shieldScreen`, `shieldClipboard`,
  `updateMode`: `ask` | `auto` | `off`, plus `updateDismissedTag`).

## How it works

A single Tauri window hosts several webviews: the **rail** (76 px), the
full screen **options** panel (created on demand), a welcome backdrop
and one chat webview per account (isolated webview profiles under
`%APPDATA%\app.zapzap.desktop\webview\<id>`). There are also on-demand
children: the "+" strip, the boot curtain, the update card.

The whole shell UI (rail, options, welcome) is served by the loopback
shell server itself (same origin), so UI `fetch` calls never fight
CORS and no Tauri IPC is used for the UI. Cross-site and cross-origin
requests to the server are refused by middleware.

The shell talks to the embedded pages through a slim, resident page
link script with a hash-beacon transport: notification capture, unread
sampling and page state only, embedded as the accounts' webviews boot.
Every failure degrades into a missing badge or dot instead of breaking
the app.

```
zapdude-rs/
+-- src/window.rs      # multi webview shell + tray + lifecycle + shield
+-- src/api/           # loopback shell server + slim page link
+-- src/netguard.rs    # per-webview tracker list + traffic ledger
+-- src/notifier.rs    # branded native toasts (contact avatar + identity)
+-- src/updater.rs     # GitHub Releases check/download/install + policy
+-- src/browsers.rs    # StartMenuInternet enumeration for link handling
+-- scripts/           # page link + per-page capture hooks, client hints
+-- ui/                # rail, options (drawer), welcome, strip, curtain
+-- windows/           # NSIS hooks (cleanup on uninstall)
+-- docs/              # architecture, voice notes, red lines, session notes
```

## Development

```powershell
cargo run                               # debug
cargo test                              # unit tests
.\tools\tauri.exe build --bundles nsis  # release + installer
```

Requires the stable Rust toolchain; on Windows the WebView2 Runtime
(bundled with Windows 11), on macOS a WKWebView (multi-webview uses
private APIs behind Tauri's `unstable` feature).

Dev-time diagnostics exist for QA: a dev build can add
`--features devtools` plus `ZAPDUDE_DEVTOOLS=1` to run a maintainers'
harness (shell state probes, synthetic toast payloads so pipelines get
verified without touching any service). The flag pair is the only
door: release builds and the release workflow compile it out, so no
release binary ever carries it. The rules live in
[docs/RED-LINES.md](docs/RED-LINES.md).

## Known limitations

- There is no automation of any kind by design (see "What Corvid does
  NOT do" above): the pages answer to the user, not to scripts.
- The `Sec-CH-UA` HTTP headers cannot be rewritten from Tauri; the
  Chrome identity is applied in JavaScript, which is what WhatsApp
  Web's feature gates read.
- Background accounts load on first use: their live notifications start
  once they are opened in the session.
- TikTok is experimental (its own anti-automation may degrade the page
  state the shell reads; the surfaces say so instead of inventing
  data). Twitch anchors on the logged-in site because whispers have no
  standalone chat page.
- The macOS build uses private multi-webview APIs, so App Store
  distribution is out; builds ship unsigned and each downloaded build
  triggers one Gatekeeper confirmation.

## License

**AGPL-3.0** - see [`LICENSE`](LICENSE) and [`NOTICE.md`](NOTICE.md).
Copyright belongs to [da1fs](https://github.com/da1fs). The design and
behavior reference was ZapZap by Rafael Tosta, who is credited with the
original concept. Corvid is not an official product of WhatsApp, Meta
or any other service: it is not built by or affiliated with any of
them, and the project keeps its own voice: started on ZapZap's ideas,
living somewhere of its own.

AI notice: developed with **OpenCode** + the **GLM-5.3 Flash** model
(by Z.ai), with the author reviewing and shipping every patch.
