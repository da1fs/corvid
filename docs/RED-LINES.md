# Red lines - Corvid

Project-level decisions that are NOT to be reversed casually. Written
2026-11-04 (the v2.0.0 cut), on a review of the six embedded chat
services' terms of service and their real-world enforcement history.
The chat voice for public copy lives in [VOICE.md](VOICE.md); this file
governs engineering behavior.

## 1. No automation surface, no exceptions

Corvid must never ship any feature that drives the embedded chat pages
from outside: no message sending, no chat or conversation reads, no
programmatic calls, no "unofficial API" framing, no browser-extension
or accessibility-style helpers outside the app window.

Why: every one of the six services prohibits unofficial automation in
its own terms (WhatsApp: unofficial clients + automation lead to
account bans; Meta: no automated data collection while logged in
without written permission; X: no access outside published interfaces,
scraping forbidden without prior written consent; TikTok: no automated
scripts interacting with the services; Twitch: no automated access).
Meta has sued tool vendors (the scraping-for-hire cases), WhatsApp
runs cease-and-desist campaigns against non-official client makers,
and a wrapper wave got fresh legal attention in late 2026.

What was CUT with v2.0.0, never again by default: REST/SSE/webhook
event surfaces, key-authenticated automation endpoints, composer and
chat-list page ops, call starting, phone-book scraping, the per-service
automation bridges, the debug eval endpoint. The unit test
(`api::link` guarded-expression test) enforces that new page-side
expressions keep answering to the SLIM link, not to an act surface.

## 2. The page hooks keep being telemetry + shell rendering only

The surviving page hooks (all of them listed here so the boundary stays
auditable):

- `client_hints_fix.js`: Chrome identity in JS (Sec-CH-UA is not
  writable from Tauri). This is feature-gate hygiene for the page the
  user already opened.
- `notification_hook.js`: capture of the page's own Notification calls,
  re-rendered by the shell as native toasts. No storage beyond the
  toast icon cache, nothing leaves the machine.
- `unread_watcher.js`: page title sampling (the "(n)" the page itself
  publishes) for the rail badge. Presence data only.
- `page_link.js`: the transport (envelope out via hash beacon) plus
  the state probe (composer/QR/password presence) plus the single
  stuck-call-pill recovery gesture for HUMAN-initiated calls.
- `call_style.js` / `call_anchor.js`: call pop-out cosmetics and the
  caller-name map the missed-call toasts reuse.
- `hide_download_banners.js`, `sw_guard.js`, `meta_focus.js`: cosmetics
  and focus hygiene.

Anything that wants to GROW this list must answer one question first:
"does this read or write conversation content, contact data, or send
anything on the user's behalf?" If yes, the answer is no.

## 3. Bulk is poison, forever

No bulk sending, no "send to N contacts", no marketing features, no
rate-limiting to survive anti-abuse systems, no monetization of any
interaction with the chat services. Acceptance criterion for feature
requests, permanently: if a feature only makes sense as automation,
it is out of scope.

## 4. If legal mail ever arrives

Cease-and-desist letters are to be answered (never ignored), read in
full, and complied with by default (modify or remove what the letter
names). Escalation over a free, local, personal-use tool is not a
battle worth fighting; rebuilding far from the line is. The GitHub
repository and the download site carry the maintainer's public
identity on purpose: hiding the author makes platform enforcement
worse, not better.

## 5. Voice contract (public copy)

The README and every surface that describes the project say plainly
that the services prohibit unofficial automation, that this build
ships without that surface, and that account restrictions (if a
platform ever applies them) happen between the platform and the
account holder, not through this project. No lawyer tone, no
anti-promises: the honest description of reality IS the voice.

## 6. A compliant re-introduction path (for the future)

If the automation idea ever returns, it returns ONLY through something
the services authorize in writing: an official business API with
accounts that opted in through that API's own terms, never through
a user's consumer session. Until such an authorized path exists for
each service, the answer stays no.
