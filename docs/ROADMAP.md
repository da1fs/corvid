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
- Order inside stage 2: GNU/Linux first (it is the free-software
  finish line: everything compiles free, end to end), macOS next.

Between stages nothing ships in a hurry: a stage closes when the
previous one is genuinely excellent on REAL sessions, not on hope.
