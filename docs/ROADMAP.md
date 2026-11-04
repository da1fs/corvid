# Roadmap - Corvid

One page the maintainer and the agent use to agree on the order of
work. The doctrine lives in docs/RED-LINES.md (red lines, the page-hook
audit, the devtools law) and the public voice follows docs/VOICE.md.
Session notes live in docs/SESSION-STATUS.md; attribution in
NOTICE.md. Written 2026-11-04 after the v2.0.0 integrity pass.

## Invariants (they never move, in any stage)

- NO automation surface toward the six chat services, no bulk sending
  ever, page hooks stay telemetry + shell rendering only (RED-LINES
  sections 1 to 3 and 7).
- Sessions live ONLY in the user's app-data folder, ONE isolated
  profile per account, never inside builds or repositories.
- The shell ships without telemetry, keys or servers; the only local
  door is the loopback shell server, same-origin guarded.
- License AGPL-3.0; the maintainer's identity stays public.
- Stability beats completeness; honesty beats shine. Every release
  rides the hygiene rituals: tests green, BOM + secrets + em-dash
  scan clean, release notes that tell the truth.

## Stage 0 (DONE 2026-11-04): integrity

The automation surface cut, the repo recreated with ONE clean commit
of history, the official channel carrying exactly one release (v2.0.0),
the red lines and the devtools law written, and the site speaking the
same honest voice. The old binaries remain nothing but OFFLINE
household copies (never re-pushed).

## Stage 1 (current): Windows excellent

Standing directive (user, 2026-11-04): focus Windows ONLY until it is
excellent and functional; the other systems wait for stage 2.

- REAL smoke test of the slim build on a live session: toasts, unread
  badges, rail dots and one call; the page link must behave like the
  old bridge on the telemetry side, with none of its surface.
- Build the ENGINE SEAM now, while exactly one engine exists (user
  question 2026-11-04: "can we change engines later without touching
  the rest of the code?" yes, with this seam): ONE `Runtime` type alias
  in main.rs (the AppHandleBox alias already carries most of it) plus
  ONE `src/engine/` module collecting the per-engine behaviors: the
  netguard request filter, the theme color-scheme pin, the popup
  policy, the eval-transport quirk table (WebView2's double-encoded
  unwrap), and the permission shield gate. Verified inventory
  2026-11-04: the raw-engine code lives in TWO sites only (netguard.rs
  and the theme pin in window.rs); twenty `Wry` type mentions ride the
  alias path; the page link protocol (hash beacon + JSON) is already
  engine-neutral. Stage 2 then swaps an implementation, not the code.
- The devtools harness v1 under its law (RED-LINES section 7), plus
  the tools/ sweep of dead page-op drivers (the probes stay: they are
  human-attached DevTools class).
- The updater dress rehearsal BY THE USER: the centered update card
  1.4.4 -> 2.0.0 through the installed flow.
- The rename cut on its own checklist (binary name, AUMID, config
  paths, Run key, Corvid_* asset matching, release naming); sessions
  migrate, never wiped.
- The release workflow keeps build boxes ONLY for Windows during this
  stage (cut the macOS matrix rows; the dmg rerun stays opportunistic
  at most: the GitHub macOS pool gave nothing on 2026-11-04, four
  attempts).
- Carried niceties: uBOL folder shipped by the installer, per-service
  proxy research (before the engine era, re-verified against it),
  fingerprint hardening LAST, DoH.

## Stage 2: the engine era (macOS and GNU/Linux join)

The objective already public on corvid-site: a free-software browser
bundled INSIDE the program, isolated, on Windows, macOS and GNU/Linux,
without depending on the WebView2 runtime of Microsoft or the
proprietary engine of Apple; on GNU/Linux the whole stack compiles as
free software, end to end.

What changes, precisely:
- The app stops borrowing the OS webview (WebView2 / WKWebView /
  WebKitGTK) and pins ONE free-software engine of its own, embedded
  and version-pinned by the project.
- The gain is NOT one build for every OS (Rust still compiles per
  target; that part stays). The gain is ONE behavior contract: the
  same engine everywhere, so Windows QA transfers directly and the
  platform runtimes stop being a variable we do not control.
- The engine gets built ONCE per engine release and vendored as a
  project artifact (the "a bit heavier, not much" line: payload grows
  by the engine, build times stay bounded; the app itself stays Rust
  for exactly that kind of control). License and attribution land in
  NOTICE.md like every other dependency.
- Engine decision checklist (ALL verified before any commitment):
  free software by the FSF's criteria, actively maintained upstream,
  embeddable from Rust through a sane glue layer, supports the hooks
  the shell needs (Notification surface or a workable equivalent),
  a bounded payload size for a desktop chat hub, and a license that
  coexists with AGPL-3.0 linking.

### The seam (why the swap stays contained)

Corvid talks to its pages in HTTP + JSON + JavaScript strings; engines
meet the app only inside `src/engine/<impl>`. The five behaviors in
that module are CONCEPTS each engine reimplements (request
interception, permission gating, color scheme, popup policy, eval
quirks): the seam makes that a contained reimplementation, not code
surgery. Two honest limits stay on paper: the wry-specific callback
slots (permission request, NewWindowResponse) differ across runtime
crates, and Servo's missing capabilities (multi-webview, transparency,
request bodies) demand shell adaptations BEYOND the seam, which is
exactly why Servo keeps the quarterly watch instead of a deadline.

### Candidates, verified against 2026 sources (2026-11-04)

Four families considered; the two hardened browser-builds the user
mentioned are NOT engines:

| Candidate | Shape | Verified state (2026-11-04 sources) | Shell fit today |
| --- | --- | --- | --- |
| CEF (Chromium Embedded Framework) via `tauri-apps/cef-rs` (the official Tauri org project; crate `cef`, Apache-2.0 OR MIT, v156.3.0 on 2026-11-01; a `tauri-runtime-cef` runtime crate exists) | embeddable Chromium, binary distribution per OS (CPE de hecho: libcef.dll/.so/framework shipped with the app) | live and frequent releases (278 releases, push cadence measured in days-weeks; targets: Windows x64/arm64/i686, Linux x64/arm64, macOS) | STRONG: real multi-webview model like today; needs an audit for the popout/call window choreography; Windows-first proof path |
| Servo via the `servo` crate (0.1.0 April 2026, monthly through 0.4.0 July 2026) plus the experimental `tauri-runtime-servo` (Windows/macOS/Linux-x11, in-process libservo) | embeddable free-software engine written in Rust (MPL-2.0), the all-Rust dream | alive and monthly, BUT the runtime docs state multiple webviews in one native window NOT supported yet, no transparent webviews yet, custom-protocol request bodies not exposed | NOT YET for this shell: Corvid's entire model is N webviews in one window (rail, N accounts, drawer, curtain). Watch monthly; revisit each quarter |
| Verso (Servo-based webview, the earlier Tauri experiment) | project | STALLED/archived 2025-10 (upstream absorbed partially into Servo) | dead as a runtime choice; its ideas live in Servo itself |
| Gecko (Firefox family: LibreWolf included) / ungoogled-chromium | hardened browser BUILDS, not embeddable engines; Gecko's only maintained embedding path is GeckoView, Android-only | LibreWolf and ungoogled-chromium stay EXCELLENT as the user's own external browser for chat links (the app's browser list already picks up installed browsers) | not applicable as embedded engines; their privacy philosophy informs how WE harden whatever engine we pin |

Privacy under the engine (whichever one wins the checklist):

- The protection does not get carried by a browser brand; it is
  built: engine sandbox ON (CEF's Windows sandbox needs the bootstrap
  packaging shape; qualify it, do not ship unsandboxed), Google
  services stripped at profile/switch level the way ungoogled does
  with patches, one isolated profile per account as today, the
  netguard/lens/shield unchanged, zero telemetry anywhere.
- VERIFY items before any commitment: codec situation of the CEF
  binary distribution the project would ship (free-software bar),
  uBlock-under-engine feasibility (today's uBOL mount may need to
  become CEF request-handler filtering: the netguard pattern already
  proves the host can answer 403s itself), and the CEF payload size
  measured, not guessed.
- Order inside stage 2: GNU/Linux first (it is the free-software
  finish line: everything compiles free, end to end), macOS next.

### Decision shape (the user's framing, 2026-11-04, recorded)

CEF is not "the best engine"; it is the only candidate whose shell
model works TODAY (multiple webviews in one window, popouts,
custom-protocol shape), so it is the BRIDGE into the engine era. Servo
is the DESTINATION candidate: the only all-Rust, all-free-software
engine (MPL-2.0, Linux Foundation Europe), not yet capable of our
shell (multi-webview and more pending), but shipping monthly. Because
both live at the SAME tauri runtime layer (`tauri-runtime-cef` and
`tauri-runtime-servo` are siblings by design), the shell work done in
the CEF era transfers to the Servo era instead of being thrown away.
The swap, when it happens, is a runtime-and-verification project, not
a rewrite. Until Servo passes the checklist on a real session baton,
the decision stays: stage 1 keeps WebView2 (Windows excellent first,
user directive), stage 2 opens with CEF as the bridge, Servo re-tested
quarterly.

Between stages nothing ships in a hurry: a stage closes when the
previous one is genuinely excellent on REAL sessions, not on hope.
