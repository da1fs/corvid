# Changelog

Notable changes to this project are documented here. The format follows
Keep a Changelog; versioning is SemVer.

A note on names: the GitHub account was renamed to **da1fs** (formerly
ldpf96) in 2026-11. Entries written before that keep the old handle,
which GitHub redirects; new entries use the current names, and links
throughout now point at the canonical addresses.

## [2.0.0] - 2026-11-04

### Removed
- **The local automation API is gone, ON PURPOSE.** All of it: the
  REST/SSE/webhook endpoints (`/api/status`, `/api/message`,
  `/api/open-chat`, `/api/chats`, `/api/call`, `/api/call/end`,
  `/api/events`, `/api/events/stream`, `/api/webhook`, the key auth,
  the rate limiter, the event ring and the debug eval harness), the
  page-side act surface (composer ops, chat-list scraping, call
  starting), and the per-service automation bridges (WhatsApp DOM
  ops, Messenger, Instagram, X, TikTok, Twitch). The chat services'
  terms prohibit unofficial automation clients and their platforms
  enforce that on accounts; the project refused to keep building
  against that letter. What remains is hub-class: the class the
  public web-client ecosystem has always been in. The red lines live
  in docs/RED-LINES.md.
- **API settings and AI agent panels in the drawer.** The settings
  page knows nothing about keys, ports-in-the-UI or webhook receivers
  anymore.

### Changed
- **Identity pass: Zapdude becomes Corvid.** The display layer (window
  titles, UI labels, the boot curtain footnote, tray menu, installer
  string surfaces) now reads Corvid; internal identifiers (binary name,
  AUMID, config path, Run key, updater asset matching, repo slug) stay
  Zapdude until the planned rename cut so existing installs and the
  update flow keep working. The cut checklist lives in
  docs/corvid-rename-checklist.md.
- **Publish truth pass (docs and links, no behavior change except the
  feed URL).** The updater's release feed now uses the canonical
  address (`api.github.com/repos/da1fs/corvid/releases/latest`); the
  Windows installer matcher accepts BOTH product spellings
  (`Zapdude_*` and `Corvid_*`, exact match first) so the rename cut can
  publish Corvid assets while every installed 1.4.x keeps updating
  (tests included); `Cargo.toml` metadata and the release workflow body
  describe the project with current names; the workflow body and
  `tools/release-body.md` point at the live download site
  (da1fs.github.io/corvid-site) and state the real macOS minimum
  (13, Ventura).
- **Config shape: the `api` section became the `port` key.** The shell
  server's loopback port now lives at the top level (`"port": 29977`);
  a v1.x `api.port` value carries over on load and every other `api.*`
  key is ignored. The server binds 127.0.0.1 ALWAYS (the network
  exposure option died with the automation API), no API key exists, and
  the server is the shell's own UI + management surface only.- **Notification delivery went direct.** Notification payloads travel
  from the page link ingest straight to the toast pipeline (the event
  ring, SSE and webhook consumers are gone); `notification_closed`
  receipts (which only API consumers used) stop being surfaced.
- **Slim page link replaces the automation bridge.** The page helper
  is now `page_link.js` under `window.__zapdudeLinkV1`: telemetry
  (Notification capture, unread sampling, page state for the rail
  dots) plus the stuck-call-pill recovery gesture; a generic
  password-field probe replaces the per-service login-form bridges
  for the no-QR services. Every page interaction stays read-only
  telemetry or shell-class rendering.
- **README rewrite for the new shape.** The README documents the hub
  without the API, adds an explicit "What Corvid does NOT do" section
  and a terms-of-service honesty section (each service's own letter,
  in place), states that account restrictions live between the
  platform and the account holder, and keeps the truthful macOS status
  (Ventura 13 minimum; some tags shipped Windows-only). The options
  drawer and welcome copy got the same pass.

## [1.4.4] - 2026-11-03

### Added
- **Permission shield (Zapdude as THE gatekeeper of your devices).** A
  chat page that asks for the microphone, camera, screen or clipboard
  gets its answer from ZAPDUDE, not from the browser: four switches in
  the Security section (defaults ON to keep video calls alive); anything
  switched OFF is refused inside the app, so the "spy" path dies at the
  door. The states persist with the rest of the app preferences.
- **Twitch as the sixth embedded service.** No standalone DM page exists
  on Twitch (whispers live inside chat pages), so the account anchors on
  the logged-in site and an adaptive bridge reads the whisper trails
  (chat list, composer, send, title badge) with honest degraded reads.
  No "only the chat" relocation applies to Twitch by design: the whole
  logged-in site is the product there.

### Fixed
- **The "+" strip is back to its rock-solid choreography.** The z-order
  caching experiment (warm parked strip) was removed after live reports
  of ghosting and dead opens under rapid clicking: the strip is created
  fresh on every open (always on top, with a close/retry for the drain
  race) and destroyed on every hide. Reliability beats the 0.7 s open.
- **The ugly "127.0.0.1:29977 says" install dialog is dead.** The
  options drawer confirms the install INSIDE the app now: the button
  arms ("Tap again to install and restart") and completes on a second
  tap; the native dialog never appears again.
- **The boot curtain carries the service's own brand** (WhatsApp green,
  messenger blue, instagram gradient, X dark, tiktok tri-tone, twitch
  purple) with the account state in plain words, and the small "Powered
  by Zapdude" footnote; the ready reveal keeps the logo of the app you
  are opening, not a generic cover.
- The network lens now reports account names once (the duplicated
  "account-account-1" cosmetic was dropped).

## [1.4.3] - 2026-11-03

### Added
- **X (Twitter) + TikTok as embedded chat services** (4 + 5). `service:
  "x"` anchors x.com/messages (twitter.com stays an embedded alias for
  old sessions), `service: "tiktok"` anchors tiktok.com/messages and is
  flagged EXPERIMENTAL (TikTok's hostile anti-automation may degrade its
  bridge reads; the ops surface degrades honestly, never breaks).
  Full parity with the other services: adaptive per-service bridge
  (pageInfo/chatList/composerInfo/sendCurrent/titleBadge), the
  "only the chat" law at BOTH layers (host navigation swap for x.com +
  twitter.com and tiktok.com non-DM landings; scripts/meta_focus.js DOM
  guard covers the SPA variants), start-menu join points (picker strip
  5 tiles, welcome + rail + drawer chips with the X/TikTok identity),
  no-QR status law (login wall = needs_login).
- **Security panel with the network lens** (transparency, the user's
  phase 4). The netguard became a FULL LENS: every account request is
  seen (catch-all WebView2 filters), recorded per account+host in a
  capped ledger, then decided: tracker-host requests answer an empty
  403 locally, the rest pass untouched. GET /api/traffic serves the
  proof data; the drawer's new Security section renders it live (host
  rows, request counters, BLOCKED badges) plus a plain-language summary
  of what is already enforced (sandboxed pages, links out, remote-site
  refusals, tracker blocking).
- **uBlock Origin Lite embedded (the user's extension wish, phase 6
  spike SHIPPED working).** The runtime supports unpacked extensions
  since 2024 (WebView2 Runtime >= 120.0.2210.55 + AreBrowserExtensions-
  Enabled; wry 0.57.1 exposes with_browser_extensions_enabled +
  with_extensions_path, and tauri 2.12 mirrors them:
  browser_extensions_enabled()/extensions_path()). Zapdude now ships a
  GPI-clean copy of uBO Lite (v2026.1103.1743, GPL-3.0, source:
  github.com/uBlockOrigin/uBOL-home releases, chromium zip unpacked to
  extensions/ublock-origin-lite/ inside the repo and to the app-data
  extensions folder on install). Each ACCOUNT webview enables
  extensions IN ITS OWN data dir and mounts the folder; beliefs noted:
  full uBO (MV2) is dead on Chromium engines, and NoScript's
  interactive model does not fit embedded chat surfaces - uBOL is the
  viable fit. Live-verified: the loader mounts per account
  ("extensions dir ready" logs) with no bridge/behavior regressions.
- **UX batch (the standard feel fixes):**
  - The + strip is now ALWAYS warm: hide parks the webview (z-epoch
    law in window.rs: Z_EPOCH + PICKER_SEED; accounts/popups/card bump
    the epoch; the reveal is instant unless a newer child was created,
    in which case one recreation keeps it on top). Common case: no
    more ~0.7 s rebuild.
  - Meta "only the chat" now happens at HOST level: a logged-in
    facebook.com/instagram.com landing is refused and the webview is
    navigated to Messenger/DMs BEFORE the feed paints (X + TikTok ride
    the same law); meta_focus.js stays as the DOM-level safety net.
  - Brand-new accounts are BORN PARKED (full size, invisible) and swap
    into the slot on on_page_load(Finished) (3.5 s fallback timer;
    HELD registry + cleanup on close): the white/black boot flash is
    gone; the welcome covers the wait.
  - /api/ui/rail/refresh + rail.__zapdudeRailRefresh: the picker and
    welcome pages call it right after /api/accounts succeeds, so a new
    tile shows in a beat (not on the next 2.5 s poll).
- **Drawer breathing pass (the user's "apelmasado" report):** wider
  section rhythm (28px gaps, 720px column), softer card interior
  (20/22px, radius 16), rows with their own side padding and soft
  dividers, larger sliders/inputs/selects paddings, more air around
  the update buttons and footer, icon-lit section titles.
- **Local-surface guard on the whole HTTP API (port-70 plane closed).**
  Any web page on any site could always FIRE fetch/POSTs at Chromium's
  loopback (127.0.0.1:PORT); Zapdude's no-auth management endpoints would
  have received them (drive the UI, quit the app, flip prefs). The rule
  now (axum middleware on the router):
  - `Sec-Fetch-Site: cross-site` -> 403 (browsers always tag fetches).
  - An `Origin` header must be OUR loopback (127.0.0.1/localhost/[::1]
    AND the API port): other local tools firing from browser contexts
    on other ports are refused too.
  - Plain HTTP clients (curl, agents' scripts, Node tools) send neither
    header: they keep working verbatim (live-verified: cross-site and
    wrong-port origins 403; same-origin 200; keyless POST 200).
- **WebView2 network guard for chat accounts** (src/netguard.rs): each
  account webview gets a third-party tracker/ad blocklist at the
  WEBVIEW2 request layer (WebResourceRequested + 4 filters per host,
  curated list: google-analytics, googletagmanager, googleads/synd,
  doubleclick, scorecardresearch, quantserve, hotjar, criteo, taboola,
  outbrain, amplitude, mixpanel, segment.io, app-measurement,
  crashlytics, sentry-cdn...). Matched requests answer an EMPTY 403
  ("blocked") without a URL round trip; they never leave the machine.
  First-party chat traffic (login, QR, media, calls) is untouched by
  design. Honest limits documented in-module: the chat services can
  still see what their OWN pages do; the list is static (evolvers beat
  deny lists; the strict allow-list mode is the next step); the IP
  that dials the chat is always visible to the chat provider.

- **Chat links open OUTSIDE the app (the external browser, with your
  sessions).** A link clicked inside a chat never navigates the account
  webview anymore (that view carries no site sessions: opening links
  inside it demanded fresh logins, the "same login prompt" report).
  Behavior:
  - Embedded surfaces (whatsapp.com, messenger.com, facebook.com,
    instagram.com + blob/media schemes) keep navigating in-app; every
    other click (anchor + window.open) hands the URL to the user's own
    browser and refuses the in-app navigation/popup (`on_navigation`
    + the popup gate in window.rs; only http/https/mailto ever spawn).
  - Browser CHOICE (prefs.externalBrowser): the drawer's Preferences
    section gains "Chat links open with" listing every desktop browser
    Windows announces (`GET/POST /api/browsers`, resolved from
    Software\Clients\StartMenuInternet; a broken choice falls back to
    the system default at click time; registry keys get a strict
    charset so a weird entry cannot escape the subpath or lie into the
    command parser; `%1` in the verb command is the URL slot).
- **Instagram as a third embedded service.** `service: "instagram"`:
  account webview anchors on instagram.com/direct (DM surface), own
  adaptive page bridge under the shared `__zapdudeApiV1` contract
  (pageInfo/chatList/composer/send/titleBadge, multi-strategy
  selectors, honest degraded reads), status dot treats the login wall
  as "needs_login" (no QR stage), picker strip + welcome screen +
  rail/drawer chips carry the Instagram identity (icon + color).
- **"Only the chat" focus guard for the Meta services**
  (scripts/meta_focus.js, deferred, `__zapdudeFocusV1`): a login that
  detours through a logged-in facebook.com landing (feed) floats the
  page back to messenger.com, and any logged-in instagram.com page
  outside /direct returns to the DMs. DOM-proof of a logged-in chrome
  gates the relocation (never fight a mid-login user), a per-session
  shot cap (3) dead-ends the guard instead of looping, SPA history
  mutations re-arm it.

- **Update policy + centered update card (the "don't show again" flow).**
  The auto-updater gains a user-facing behavior axis:
  - `updateMode` pref: `ask` (default: download in the background, ask
    before installing), `auto` (download, verify, install and relaunch
    with no prompts), `off` (never download or install on its own; the
    drawer's buttons become the only path, which is the promise behind
    "Never install automatically").
  - A centered card (`/update-nudge` overlay, `380x204`, like the
    drawer/picker mechanics) appears the moment a verified installer is
    ready: "New version vX is ready", one checkbox ("Don't show this
    message again") and two buttons:
    - checkbox + **Update now** stores `auto` BEFORE the install POST (a
      fast exit can never outrun the promise), then installs.
    - checkbox + **No thanks** stores `off` + the release tag; the card
      came from the `ask` mode and never shows again. Plain No thanks
      dismisses only this release (a newer tag clears it).
    - Plain Update now installs just this release.
  - The card is true to live state: it self-closes if the phase stops
    being `ready` (someone else installed), the policy flips, or the
    tag changes; the shell also closes it on every updater phase change
    (`nudge_poll`) and per explicit POST (`/api/update/nudge/hide`).
  - `POST /api/update/prefs {mode?, dismissed_tag?}` drives both the
    card buttons and the drawer's new "Updates" selector (Preguntar /
    Automático / Nunca), plus a manual "Download and install vX" button
    on `newer` when the policy is `off` (the updater skips its
    auto-download there). `/api/update` and `/api/prefs` now carry the
    policy; legacy pref POSTs keep the stored mode (theme toggles never
    reset it).
- `nudge_policy_is_strict` test pins: ask+ready+undismissed only; auto
  and off never ask; `newer`/`downloading`/`fresh` are not card states.

## [1.4.2] - 2026-11-02

### Fixed
- **The right pane can no longer go permanently blank when a chat is
  clicked (the "chat disappears" report, struck structurally).** The
  promo hider's generic sweep STILL painted inline `display:none` and
  climbed ancestors (an 1.3.3 fix covered only the empty-state card),
  so a transition moment with a transient "promo-ish only" region
  could collapse a container the CONVERSATION mounts into; nothing
  ever un-hides inline styles, so the pane stayed born-invisible with
  the row still selected (live evidence: `#main` absent while intro
  nodes carried our inline hides). The whole file is RULE-BASED now
  (the 1.3.3 lesson, universal):
  - Nothing is painted, nothing is climbed; hiding is a
    `data-zapdude-promo` attribute + one stylesheet rule.
  - Stamps are corrected every sweep and REVERSED the moment the copy
    leaves, a functional element (composer, list, media controls,
    canvas...) appears inside, or the node stops being the smallest
    match. A transient over-catch can flicker; it CANNOT stay hidden
    forever (live-verified: stamped node with a mounted
    contenteditable lost its stamp within one beat).
  - Protections run in both directions (nothing stamped inside or
    holding `#pane-side`/`#main`/editables/canvas/dialogs), plus a new
    no-stamp list of WhatsApp shell roots.
  - The logged-out landing flow now stamps cards instead of painting
    them, same guarantees.
  - Verified live with REAL (trusted) mouse clicks: conversation opens
    complete (header + composer) from the intro, across THREE
    consecutive chat->chat switches, with zero stamps inside `#main`
    at every step.

## [1.4.1] - 2026-11-02

### Changed
- **Every shell button answers on the click frame now** (the user's
  law: la velocidad del cambio de cuenta se aplica a todo). Added:
  - The brand logo (options panel toggle) flips its green ring at the
    instant of the click; the panel follows a beat later, and the poll
    keeps the ring honest with server truth (drawer state now rides
    `/api/accounts` as `shell.drawer_open` / `shell.picker_open`).
  - The "+" tile fills instantly when pressed (its strip state
    synchronized the same way) and account tiles gained a pressed
    scale; welcome-screen buttons too.
  - Closing the options drawer (the X or Escape) fades it out at the
    click frame instead of waiting for the host round trip.
  - Picking WhatsApp/Messenger in the "+" strip collapses the card the
    frame the item is clicked (brought back if the account creation
    fails, so an optimistic dismissal never hides a real problem).
  - The settings "Save" button confirms the click instantly, with the
    "saved" toast following the round trip.

## [1.4.0] - 2026-11-02

### Changed
- Light-tier update notice (the drawer's direct GitHub check) replaced by
  the full middle tier below.

### Added
- **GitHub Releases auto-updater (middle tier).** The host checks the
  latest release on a quiet schedule (3 minutes after boot, then every
  5 hours), compares it against the running build, and when a newer
  Windows installer exists it downloads it by itself into
  `%APPDATA%\app.zapzap.desktop\updates\` (size + MZ-header verified
  before anything is trusted). The drawer's About card gains the live
  state machine (idle / checking / newer / downloading with progress /
  ready / installing / failed) and a one-click **Install and restart**
  action: the app writes a wrapper batch that waits for the process to
  drain, silent-installs the NSIS setup `/S`, and relaunches Zapdude.
  New endpoints (management tier): `GET /api/update`,
  `POST /api/update/check`, `POST /api/update/download`,
  `POST /api/update/install`; bus events `update_state` carry every
  phase change for SSE/webhook agents. Failures degrade into an honest
  `failed` phase, the running app is never touched without the
  explicit install step, and 26 unit tests cover the version compare,
  tag shapes and the install script.
- **Instant account-switch highlight in the rail** (the user's report:
  clicking account 2 kept the previous shading for up to ~2 s). The
  active state used to ride the 2.5 s poll; now the clicked tile marks
  itself active the instant the click lands (optimistic state) and
  the server payload confirms it on the next poll, so the switch LOOKS
  as fast as it actually is. Server truth always wins when they
  disagree (tray/API switches repaint freely).

## [1.3.3] - 2026-10-30

### Fixed
- **Clicking a chat could leave the conversation pane GRAY** (the user hit
  it live: the click marked the chat read but the right side never
  rendered). Root cause: the empty-state promo card hide painted
  `display:none` on elements AND climbed collapsible ancestors - the
  card sits high in the right pane's tree, and during WhatsApp's
  intro->chat React transitions a sweep could catch a transient where
  the pane held only already-hidden children, collapsing a container
  the CONVERSATION view then mounts into. Born invisible. The hide is
  now a selector RULE (`intro_panel_v2_title_card`, style tag - same
  technique as the call styling): it dies with the card, climbs
  nothing, paints nothing; while the card is unmounted it matches
  nothing at all. Verified live: open-chat -> `#main` rendered,
  composer visible, no gray.

## [1.3.2] - 2026-10-30

### Added
- **Unread badges on the rail account tiles** (the user's request: the
  mobile-style count bubble). Each account page samples its OWN
  document title (`(4) WhatsApp` / `(12) Messenger`) through a shared
  drift-proof watcher (`unread_watcher.js`, 3s pacing, ships on
  change); the beacon transport normalizes both services'
  writer paths (`unread` + Messenger's `title_badge`) into a fresh
  per-account store, `/api/accounts` carries `unread`, and the rail
  paints a green count bubble on the tile's top-right corner (the
  session dot keeps the bottom). 99+ clamp, silver 0 = hidden.
- **The toggle**: a new "Unread badges" switch in the drawer
  Preferences (new `unread_badges` pref, serde default ON so older
  configs read unchanged). Off = the bubble class drops at CSS level
  (stale counts can never pop a bubble back mid-poll).

## [1.3.1] - 2026-10-31

### Fixed
- Missed-call toasts now wear the SAME clean round avatar the incoming
  ring toast used: the ring surface stores its channel avatar keyed by
  the caller's handle (`window.__zapdudeCallerIcon`), and the
  missed-call hook swaps WhatsApp's own composite icon for it (WhatsApp
  ships a re-cropped photo with the WhatsApp badge stamped on - the
  user called it out: "sale diferentemente ajustada y con el logo de
  WhatsApp"). No ring seen -> the composite icon stays (degrade, never
  crash). Verified live: the missed event's icon is the exact CDN URL
  the ring surface carried, title mapped to the human name, silent.
- WhatsApp Web's v2 empty-state promo card ("Descarga WhatsApp para
  Windows / Obtén funciones adicionales, como llamadas…" with the
  Descargar button) is now hidden by its own semantic testid
  (`intro_panel_v2_title_card`) - the generic banner sweep could never
  collapse it because the bare "Descargar" button text blocks the
  ancestor climb. The functional action-tile row stays.
- The "create an account" welcome screen no longer flashes for a second
  at every startup when accounts exist: the shell boots the welcome
  webview as a pure brand splash (`?splash=1`, logo only), and the full
  picker only renders in the real no-accounts state - where it also
  re-gains its add buttons on the delete-last-account path (it
  re-navigates out of the splash variant before taking the screen).

## [1.3.0] - 2026-10-30

### Added
- Multi-service accounts: each account now carries a `service`
  (`whatsapp` legacy default, `facebook`) that selects the REAL web chat
  the embedded view loads (WhatsApp Web / the Messenger chat surface).
  Facebook login (email + password + 2FA) happens INSIDE the view, the
  same way WhatsApp's QR does. Old configs load unchanged as WhatsApp.
- Facebook receive-side phase 1 over the local API: `service` field on
  `/api/accounts`, `/api/status` and on every incoming event; session
  states `logged_in` / `needs_login` (login form detected) / `detecting`;
  `status`, `chats` and the event stream are service-agnostic, while the
  WhatsApp-only operations (`message`, `open-chat`, calls) answer a
  structured `unsupported_for_service` on other accounts to keep the
  API contract clean.
- A new adaptive page bridge for Facebook (`facebook_bridge.js`) under
  the SAME `window.__zapdudeApiV1` contract: honest degraded reads
  (multi-strategy selectors, structured results), a document.title
  unread watcher shipping `title_badge` events, and every entry point
  guarded so markup drift can never break an endpoint.
- The service picker: clicking the rail's "+" opens a horizontal brand
  strip anchored beside the tile (WhatsApp / Messenger logos, real
  simple-icons CC0 glyphs); it follows the tile, and closes on pick,
  Esc, outside click or cursor exit.
- Incoming calls surface as the SAME branded native toast the message
  notifications use (caller identity via the semantic testid first,
  avatar pass-through, per-node dedupe), while the caller's own
  "Calling.../Llamando..." surface stays SILENT (the caller is already
  in the call; the real capture had produced one bogus toast) and toast
  titles arrive clean (WhatsApp's "~" presence glyph stripped). The
  in-page call prompt keeps working with a cosmetic brand-styling pass
  (rounded card, soft shadow) anchored on its stable testids - CSS only,
  buttons keep their native pill shape, the stylesheet self-degrades on
  markup drift.
- Uniform toast law: every native toast renders at THE SAME SIZE at
  birth - the icon slot is always occupied (the contact avatar when the
  payload carries one, the app icon as universal fallback otherwise),
  titles clamp to one line and bodies to exactly two (short bodies get
  padded with an invisible non-breaking space), so message, call and
  status toasts read as one family.
- Missed-call notifications are fully branded: WhatsApp ships the caller
  avatar as a huge inline data URL which the flat 4096-char length gate
  used to reject silently, so the toast rendered bare; the gate now only
  applies to remote URLs and inline payloads decode up to 300 KB. The
  missed-call toast also carries the CALLER's human identity: the ring
  relay learns the handle -> display-name pair at ring time (WhatsApp
  titles the missed Notification with the bare handle) and the page hook
  swaps it in.
- A service-worker guard stops the SQUARE notification: WhatsApp's own
  worker (web.whatsapp.com/sw.js) raises ring and push notifications
  from its own realm via `registration.showNotification` - an un-stylable
  OS bubble the branded pipeline can never touch (the user saw it
  stacking behind the branded toast). The guard intercepts
  `navigator.serviceWorker.register` (the original promise still
  resolves so the page starts normally) and sweeps existing
  registrations every two seconds; the page-level capture keeps feeding
  the branded pipeline.
- A stable-backup practice: `tools/backup-stable.ps1` snapshots the
  verified dev build (exe + symbols + full source zip + verification
  manifest) into `backups/stable-dev-<timestamp>/`, keeping the newest
  twelve.

### Changed
- Toast anatomy: each notification reads contact (bold) + message, with
  the contact avatar as a small round icon at the LEFT; the redundant
  "Zapdude" title and the raw `[account-1]` tag are gone, and clicking
  opens the account - and now the chat - that produced the notification
  (the hook's stored onclick handlers get re-dispatched).
- Rail: real service glyphs with brand colors per account, honest state
  dot (green = connected, yellow = waiting for QR/sign in, red = page
  problem, grey = detecting) backed by stale-while-revalidate session
  caching so the color stops rotating, and the "+" pinned near the
  rail's bottom edge.
- The attribution file now credits the third-party assets in use
  (simple-icons CC0 icons for the embedded services) and carries the
  trademark wording for WhatsApp, Messenger and Facebook.
- The "download the desktop app" promo hider now also matches the
  short Spanish "descarga la app" family of texts (plus `WhatsApp
  Desktop` variants), keeping its protected-zone guardrails.
- `/api/call` survives WhatsApp's call-button label drift: the click
  matches exact labels first ("Llamada", "Llamada de voz", "Voice call",
  "Llamar") and falls back to the first call-ish label that is not the
  video button (live diagnostics caught the plain Spanish "Llamada"
  label leaving the endpoint answering `call_button_not_found`); the
  failure payload now lists observed labels for the next drift.

## [1.2.1] - 2026-10-29

### Added
- Branded notifications, ONE uniform surface: WhatsApp Web's page-level
  `Notification` calls are captured by a deferred page hook and raised as
  native Windows toasts through the registered Zapdude identity (AUMID
  `app.zapzap.desktop`), with the contact avatar when the payload carries
  one (both CDN URLs and inline data:image icons) and click-to-restore.
  One look for every state - window visible, hidden or in the tray. The
  toasts double as push events on the API bus (`SSE`, polling, webhooks).
  (An earlier dual design - in-app overlay when visible, native when
  hidden - read as "different notifications per account" in real use and
  was replaced by the uniform route.)
- Call automation: `POST /api/call` (open a chat and click the voice/video
  call entry point, auto-opening the account webview when needed) and
  `POST /api/call/end` (click the hang-up control of the active call
  surface). Popup lifecycle is now observable: `popup_opened` and
  `popup_closed` events land on the API bus, and the account page gets an
  idempotent recovery prompt (`callRecover`) the moment its popup dies
  without WhatsApp's own end-call flow.
- Chat-opening automation goes through the search drawer (with retry beats
  and a robust pick for own-number and unsaved-number results). The old
  `?send` deep link trapped cold-started webviews on an unresolvable
  redirect (about:blank) and is gone from the flow.
- Debug harness: `ZAPDUDE_DEBUG_EVAL=1` unlocks `POST /api/_debug/eval`
  (live JS evaluation in account webviews), `GET /api/_debug/windows`,
  `POST /api/_debug/windows/{label}/close` and `POST /api/_debug/emit`
  (synthetic bus events). All key-gated and loopback-only, never rate
  limited (so live sessions can poll), production-safe by default.
- Second-launch behavior: when Zapdude already lives in the tray, a second
  activation (desktop shortcut, Start Menu, exe double click) now pings the
  running instance through `POST /api/ui/show`, which brings the window
  back - no more dead clicks. Autostart migration: the Run key entry now
  carries `/startminimized`, so only auto-launched instances stay hidden;
  manual clicks always open the window.

### Fixed
- Media download: the promo hider no longer matches bare "Download"/
  "Descargar" buttons outside the logged-out landing flow. The media
  viewer overlay lives outside the protected chat zone and carries a real
  Download button; the oversweep erased it (the download option that
  vanished). Dialogs and menus are now protected zones as well.
- Chat list scraping: WhatsApp moved the list to `data-testid="list-item-N"`
  grid rows; the scraper follows the new markup (plus the dedicated unread
  badge test id) and reports `unread_count` per chat.
- Recreated account webviews kept the original page bridge, so every
  automation call after a close/re-open cycle timed out against a dead
  webview. Bridges are now replaced on recreation and cancelled (poller
  exits) when the webview goes away.
- Composer readiness tolerates the deferred-bridge installation gap after
  page navigation/reloads (wait state, not failure).

## [1.2.0] - 2026-10-28

### Added
- App-wide light/dark theme: account views pin the theme color scheme on
  their WebView2 profiles (`prefers-color-scheme`) at creation and on
  preference changes, so WhatsApp Web follows the in-app theme live -
  no reload, no session round trip. Native caption bars follow too
  (`Window::set_theme`, applied to the main window and call pop-outs).
- macOS support: first `.dmg` builds (Apple Silicon + Intel) via
  TypeScript-free CI on GitHub Actions (macOS 14+, unsigned, distributed
  directly). `icon.icns` added to the bundle list and the macOS minimum
  system version pinned to 14.0 (multiwebview needs the private APIs and
  only works there).
- Release workflow `.github/workflows/release.yml`: tagged builds for
  macOS (aarch64 + x86_64) and Windows NSIS from the same tag.

## [1.1.0] - 2026-10-28

### Changed
- The whole project (UI, docs, installer, tray, code comments) is now in
  English.
- Project origin and credits consolidated in NOTICE.md and README.md:
  the design reference (ZapZap by Rafael Tosta) and the development
  tooling (OpenCode agent harness + GLM-5.3 Flash model by Z.ai, with every
  change verified on real hardware by the author) are documented there.
- Installer ships English only.

### Fixed
- (from 1.0.0 line carried forward) App icon: the icon.ico embedded in the
  exe was rebuilt byte for byte from the approved green squircle frames so
  the taskbar, title bar, tray and installer all show the green with the
  black bolt (Windows icon caches may need one Explorer restart).

## [1.0.0] - 2026-10-28

### Added
- Multi account WhatsApp Web desktop client written in Rust (Tauri 2 +
  WebView2). Original project by ldpf96; the design reference was ZapZap
  by Rafael Tosta.
- Single window shell v3 (feature `unstable`): an account rail (76 px),
  one WhatsApp Web webview per account with an isolated WebView2 profile
  (`webview/<id>`), a welcome backdrop and a full screen options panel in
  the same window (the rail logo toggles it).
- Multi account flows: "+" adds an account with no forms and opens its QR
  right away; instant account switching via position-only parking
  (parked at full size outside the window, no re-render, under 350 ms);
  lazy loading so every account creates its webview on first open.
- Premium rail: Zapdude logo, avatars with the bolt (no letters or
  numbers), green ring on the active one, state dot (connected/QR/
  loading), curved dividers, a visible red X when right click arms
  deletion, incremental DOM updates (no flicker). No browser context
  menu on the shell UI.
- System tray: "Show <account>" per account, "Hide all windows", "Quit
  Zapdude". Closing the window hides it to the tray. Single instance by
  mutex.
- Stable Chrome identity (UA 156 + `client_hints_fix.js`) and
  pre-approved permissions for microphone, camera, screen, notifications
  and clipboard so WhatsApp Web calls work.
- Local API for people and agents: REST (`healthz`, `config`, accounts
  CRUD + open/hide, `status`, `open-chat`, `message`, `chats`, `events`,
  `webhook`) + SSE (`/events/stream`), loopback by default,
  `X-API-Key`/Bearer auth, 60 rpm, 1 MB body cap, events tagged per
  account and HMAC-SHA256 signed webhooks (`X-Zapdude-Signature`). The
  shell UI (rail/options/welcome) is served by the API server itself
  (same origin, no IPC).
- Automatic WhatsApp promo cleanup: the "Download WhatsApp for Windows"
  banner on the landing and the download card in the empty chat panel,
  with protected zones (never touches the QR, the chat list or the
  composer).
- Resident page bridge (`api_page_bridge.js`) with
  `eval_with_callback` primary transport and a URL hash beacon
  (`#__zapdude=`) fallback, a 30 ms poller thread per account; deferred
  installs with a per-script idempotency flag.
- Deleting the last account shows the welcome screen with an explicit
  "Add account" button (no empty accounts are auto-created); on a virgin
  install the shell auto-creates "Account 1" like the reference client.
- Persistent config at `%APPDATA%\app.zapzap.desktop\config.json`
  (camelCase, BOM tolerant) with a sanitized `api` block, `prefs` and
  `accounts`.
- Preferences: light/dark theme (shell only), start with Windows (HKCU
  Run key via winreg), start minimized (lives in the tray).
- Call pop-outs: `on_new_window` creates a real window that shares the
  opener's WebView2 environment (same session/cookies).
- Symmetric rail spacing with a "more below" indicator and wheel
  scrolling; account renaming (tray, rail and API see the new name).
- NSIS installer (`tauri build --bundles nsis`, `installMode:
  currentUser`, uninstall hook that wipes the data folder).
- License AGPL-3.0 with a NOTICE crediting the design reference (ZapZap
  by Rafael Tosta), the author (ldpf96) and the AI-assisted tooling
  (OpenCode + GLM-5.3 Flash).

### Fixed
- WebView2 gray page (two chained causes): webviews born hidden or tiny
  never start their composition pipeline (they are born at full size and
  parked by position), and the old promo sweep hid whole containers
  (rewritten smallest-node-only with supervised collapse).
- Script injection: only the minimum runs at document-start; the bridge
  and the hider install deferred through the beacon poller (one big
  document-start eval reliably breaks WebView2 painting).
- WebView2 `ExecuteScript` arrives double encoded when the value is a
  String: `evaluate` unwraps one more level (data used to fall back to
  defaults silently).
- axum 0.8 nesting at root panics: flat `/api/...` routes.
- `session_state` is honest with 3 states: `detecting` / `scan_qr` /
  `logged_in`, with a 2.5 s cache so the rail answers instantly.
- The API server starts BEFORE the webviews (no more Edge "can't reach
  this page" on open) and the bind retries 40x250 ms (port changes used
  to collide with error 10048).
- Promo buttons whose `textContent` drags the SVG title ("Downloadic-
  download"): `cleanText()` strips `ic-*` tokens before exact matching;
  the deferred installer uses one flag per script so a bridge failure
  never silences the banner cleaner again.
- App icon: `icons/icon.ico` embedded in the exe carried the stale
  teal/white bolt look; rebuilt byte for byte from the approved green/black
  PNG frames (7 sizes) and the UWP tiles re-rendered with the same GDI+
  recipe (`scripts/regen-icons.ps1`, auditor in `tools/dump-ico.ps1`).
- Options drawer close speed: `toggle_drawer` used to wait for the 2 s
  geometry watchdog; `relayout` now runs inside the toggle (parking
  measured at 1 ms).
- Account rename list: the drawer rebuilt the list every 5 s and reset
  the input you were typing in (and stole its focus). Rows are now
  persistent by id and the refresh never touches a focused or dirty
  input; untouched rows still follow server-side renames.
