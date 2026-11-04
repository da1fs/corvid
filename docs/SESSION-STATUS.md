# Session status - Corvid (Rust)

> Continuity document for AI agents and future sessions. Newest block
> first; read top down.
>
> ---- SESSION CLOSE (2026-11-04, morning) ----
> WORLD STATE: the publish truth pass is COMPLETE and pushed in both
> repos. corvid main = d912313 (docs pass 6d747ee + workflow body
> e78a245 + close note); corvid-site b5d1f20 + c2b4120, live-verified
> by fetch after the Pages rebuild. The voice law lives in
> docs/VOICE.md and is referenced from the project AGENTS.md (the
> "public voice" section): present tense only for verified facts,
> absolutes only where facts hold (license, architecture), limits said
> inside the feature, breakage path instead of guarantees, no lawyer
> disclaimers. SITE JS resolves downloads per asset kind across the
> whole releases list: macOS buttons point at the last release with
> .dmg (1.4.2 today) with a version note; the updater matcher accepts
> Zapdude_* AND Corvid_*; the updater feed is canonical.
> GITHUB SURFACES: 11 published release bodies patched (the dead
> ldpf96.github.io/zapdude-site URL swapped for the live corvid-site
> URL; slugs canonical), one orphan v1.2.1 draft deleted, repo
> description + topics refreshed (desktop-client, multi-account, rust,
> tauri, whatsapp, whatsapp-web, messenger, macos, privacy).
> CI TRUTH: v1.4.3 and v1.4.4 runs failed on ALL legs on the BOM byte
> combo (0xEF 0xBB 0xBF leading Cargo.toml, the 65279 character); the
> BOM is already dead on main (the stray glyph scrub commit), but those
> tags froze the old tree, so re-running them proves nothing. THE NEXT
> REAL TAG IS THE PROOF RUN for the macOS .dmg legs. Standing ritual
> before any tag: BOM scan across json/toml/yml, secret scan, personal
> number scrub, em-dash and decorative tick check, cargo test, and DO
> NOT announce a platform the CI run did not actually build.
> AUTH STATE: the gh keyring carries the ACTIVE account da1fs with the
> scopes repo, read:org, gist and workflow, plus the inactive duplicate
> labeled ldpf96 (the same renamed account; harmless, optional cleanup
> with gh auth logout someday). The device flow token was minted inside
> this machine by a local script and stored via gh auth login with the
> token on stdin; no token was printed and the temp file was deleted
> after use. NO BACKGROUND WORK PENDING: both pollers ended; the unused
> third device code expires by itself.
> NEXT SESSION RAMP (user decisions, in no fixed order):
>   1) Pick the next build ladder: the GNU/Linux CI row (deb and
>      AppImage), the free engine spike branch (Tauri 3 + CEF), or the
>      shield design doc before any shield code. Ask nothing mid-way,
>      document as we go.
>   2) When a tag is cut: run the full publish ritual above and watch
>      the CI run live; release NAMES still read Zapdude until the
>      rename cut (checklist items 1, 2 and 4 to 6 stay gated on that
>      planned cut).
>   3) Optional tidy-up rides (only with related changes): the three
>      pre-existing compiler warnings (unused cached in server.rs,
>      unused unsafe in netguard.rs, unused mut in window.rs) and the
>      duplicate gh keyring entry.
> SESSION LESSON (auth): gh auth refresh died twice on the
> terminal and browser handoff (one Cancel press, and one window closed
> right after the browser success screen; a grant only SAVES through
> the gh process that issued the code and polls on the same machine).
> Reliable recipe: a local script issues the device code (the GitHub
> CLI app id) and polls in the background while the user presses the
> GREEN button on the device page (it unlocks itself after a short
> security wait; Cancel kills the grant), then gh auth login reads the
> token on stdin. Full narrative in the CLOSE NOTE inside the copy pass
> block below.
> SESSION LESSON (shell, reinforced from the previous session's own
> law): a PowerShell method exception does NOT stop the chain of
> statements separated by semicolons: the next statement ran with a
> null value and emptied docs/SESSION-STATUS.md mid-edit (recovered
> with git restore from the HEAD commit, zero data loss). House law
> restated: bulk file edits go through scalar .Replace with a scalar
> miss counter and every WRITE must guard against null first; the
> source of truth stays git; after ANY scripted write, verify the byte
> count before considering the step done.
>
> ---- COPY + PUBLISH TRUTH PASS (2026-11-03, late night) ----
> TRIGGER: the user ordered a file-by-file review of every public text
> and of the publish configuration, because the site and repo read like
> an AI brochure: promises nobody can keep ("nada que se rompa"), a
> dead download URL in shipped copy, and a macOS story that contradicted
> the code. FACTS established while verifying (old notes were wrong
> twice): (1) GitHub Pages does NOT redirect renamed site URLs:
> ldpf96.github.io/zapdude-site answers 404; the live site is
> https://da1fs.github.io/corvid-site (fetched). (2) CI runs for tags
> v1.4.3 and v1.4.4 FAILED on all three legs with "Unknown character
> 65279" = a UTF-8 BOM leading Cargo.toml at those tags (already
> scrubbed on main by the stray-glyph commit; the tags predate it, so
> re-running them would fail again; the next tag from clean main is the
> proof run for the macOS .dmg legs, see the named standing rule in
> docs/corvid-rename-checklist.md). Release assets today: 1.4.3/1.4.4
> Windows-only, 1.4.2 the last with .dmg. (3) macOS minimum is 13
> (deliberate, 1.2.1, family Mac on Ventura 13.7.8): the site said
> "macOS 14 required": WRONG, fixed everywhere.
> EDITS (zapdude-rs): README full rewrite (Status read-this-first
> section: one-person young project, breakage path via Issues, release
> notes in public; Features six services; macOS 13 + quarantine truth +
> "recent tags may ship Windows-only"; uninstall note that the data
> folder holds the sessions; API table completed with traffic/shield/
> browsers/update-prefs/nudge/read routes; components tree updated;
> AI notice states the review-per-patch method instead of "vibe coding").
> src/updater.rs: feed URL canonical (api.github.com/repos/da1fs/corvid)
> + installer_asset_priority accepts Zapdude_* AND Corvid_* (exact
> first, v-prefixed second) with tests = checklist item 3 DONE.
> Cargo.toml description/authors; drawer.html copy pass (About hears
> da1fs/corvid, agent section six services honest wording, ZAPDUDE ->
> Corvid in the shield note, unverified "no geolocation" claim replaced
> by what the code really does + the honest netguard limit, stray
> Spanish title attribute -> English); .github/workflows/release.yml +
> tools/release-body.md: live site URL, macOS 13, canonical source link;
> docs/architecture.md: Corvid title, bias to current components
> (netguard/updater/notifier/browsers) and the native-toast
> notification route (the old toast-overlay webview paragraphs were
> stale); CHANGELOG Unreleased carries the pass + the ldpf96 name note;
> historical release-notes: 19 canonical URL swaps (no history
> rewrites); NEW docs/VOICE.md (voice law: present tense only for
> verified facts, absolutes only for the license and the architecture,
> limits stated inside the feature, breakage path, learned rewrites).
> EDITS (corvid-site): index.html ES+EN+attribute full pass: "nada que
> se rompa" gone; "Gratis para siempre" reduced to ONE license-anchored
> line in the fine print; coffee simile dropped; SU/services caps
> ironed; macOS truth (13 minimum; quarantine repeats per new
> download); Windows uninstall session-warning; status line added
> under the installs ("proyecto joven, mantenido por una persona, si
> algo se rompe se reporta en Issues y se corrige en publico"); the
> MAC download JS now scans the FULL releases list per asset kind (no
> more offering .dmg the newest tag does not have: it grabs the last
> release that does and prints which version in a dedicated note
> element mac_note); site README redirect claim corrected + new
> behavior documented. node --check green on the page script; no BOM in
> either repo file set (scanned).
> NEXT (user decisions): macOS CI legs need the next real tag to be
> proven (existing 1.4.3/1.4.4 reruns would fail on the old tree); the
> rename cut items 1/2/4-6 stay gated on a planned release; release
> NAMES still read Zapdude on purpose until the cut.
> NOTE: cargo test was run by this pass: 33 green (32 + the new matcher
> test); pre-existing warnings untouched (unused cached var in
> server.rs, unused unsafe in netguard.rs, unused mut in window.rs).
> COMMAND LINE discipline: gh --jq user flags explode under this shell;
> parse JSON through ConvertFrom-Json on a FILE (powershell ">" writes
> UTF-16; read back with [System.IO.File]::ReadAllText).
> CLOSE NOTE (2026-11-04 morning): RESOLVED. The release.yml body fix is
> pushed (e78a245). Root cause of the earlier fail: the account token
> lacked the `workflow` scope and the first two device-flow attempts
> died on the terminal/browser handoff (a Cancel press, and a window
> closed after the browser success screen; a device grant only saves
> itself through the gh process that ISSUED and WAITS on the same
> machine). Working pattern (works, reusable): a script on this machine
> issues the device code (GitHub CLI app id 178c6fc778ccc68e1d6a) and
> polls in the background while the user presses Authorize in the
> browser; the local script then stores the token via `gh auth login
> --with-token`. Auth state now: ACTIVE account **da1fs** with scopes
> repo, read:org, gist, workflow; the old keyring entry labeled ldpf96
> (same real account, renamed) stays inactive as a duplicate; harmless,
> but no more pushes should rely on it. Lesson for the user side: on the
> device screen the GREEN button enables itself after a short security
> wait; Cancel kills the grant.
>
> ---- SESSION CLOSE (2026-11-03, late evening) ----
> WORLD STATE: the user's GitHub account is **da1fs** (formerly
> ldpf96, verified via API). Public profile = **corvid** (the project)
> + **corvid-site** (the download site); every other profile repo
> (c, claw-code, r-basic, curso-algebra-lineal) was set PRIVATE on the
> user's explicit order. The project repo was renamed to da1fs/corvid
> and the site repo to da1fs/corvid-site (GitHub redirects the old
> addresses); local remotes re-pointed. PRODUCT = Corvid in the display
> layer: UI pages, src strings and comments, macOS plist, README
> (credits to da1fs), NOTICE (credits + rename note), CHANGELOG
> Unreleased entry, GitHub repo description. INTERNAL IDENTIFIERS HELD
> for the release cut ON PURPOSE (installed 1.4.4 + updater must keep
> working): tauri.conf productName, NSIS artifact naming, updater asset
> matcher (Zapdude_*), HKCU Run key value, AUMID app.zapzap.desktop and
> its display name, zapdude.exe + Cargo package name, appdata config and
> webview paths, webhook contract headers (X-Zapdude-Signature,
> Zapdude-API/1 user agent). The authoritative cut list is
> docs/corvid-rename-checklist.md. 32 unit tests green. Repo commits:
> 1596c3b (stray glyph scrub), 59801e6 (display rebrand), 85db255 +
> 6404219 + 9dadb01 (docs), site commits 7217fe1 / e86381f / 1aa6d9c.
> SITE LIVE and verified by fetching it:
> https://da1fs.github.io/corvid-site (rebuilt by Pages). It carries
> THE HEART OF THE PROJECT section (who the tool is for: people who
> earn their living through chat networks and deserve protection; the
> three objectives: network allowlist shield, bundled free browser on
> every platform, honest local API; why the name: the corvid bird), a
> six-service feature line, permission shield + traffic lens cards, and
> the honest note that the published build still installs as
> Zapdude.app until the cut.
> DOCTRINE recorded in AGENTS.md: FSF-first vocabulary ("free software",
> never "open source"; GNU/Linux for the system, Linux only for the
> kernel; the FSF classifies licenses, it does not "approve" them;
> MIT/Expat and Apache-2.0 ARE free per the FSF list, while the
> recommended vehicle for this project is copyleft AGPL). The repo stays
> professional and neutral in tone: the user explicitly rejects
> political labels; the message is protection of people, stated
> plainly.
> THE PLAN the user approved ("sí, analiza el repo y actualiza todo"):
> Fase 0 identity pass (DONE); Fase 1 GNU/Linux CI row (pending
> decision on order); Fase 2 free-engine spike on a branch (Tauri 3 +
> CEF, gated on Tauri 3 stable + the CEF Windows sandbox fix); Fase 3
> the shield (netguard evolution to strict allowlist per account +
> service, locked preconfigured uBOL, our own script gate; real
> NoScript only if the spike proves it fits); Fase 4 rename cut with an
> updater bridge + site asset strings.
> NEXT SESSION RAMP: 1) the user decides order: GNU/Linux CI row first
> or the CEF spike branch first; 2) watch the centered update card on
> the installed 1.4.4 (the live test pending from the 1.4.4 hop);
> 3) execute the rename cut ONLY on an explicit publish greenlight;
> 4) a shield design doc before any shield code.
> SESSION LESSON: a PowerShell bulk-replace helper whose function
> pipeline printed into the assignment corrupted the written site file
> (one "MISS:" line landed as content; recovered by cutting the file at
> the <!DOCTYPE index). Law: bulk text edits stay scalar .Replace
> calls with a scalar miss counter; report findings AFTER writing,
> never through the value being written. Also: NEVER emit stray
> non-Latin glyphs (one was scrubbed from this file and the user
> ordered the rule).
> OPERATING RULES carried: the user reads NOTHING mid-task (end-of-task
> recap is the channel) and now asks for SHORT replies; the user comps
> the session; backups stay stable-dev-*; publish ritual unchanged
> (hygiene scan, number scrub, secret scan, tests, release notes
> without em-dashes or decorative ticks, tags single-v).
>
> Continuity document for AI agents and future sessions. Last updated:
> 2026-11-03 - SITE PASS: zapdude-site rebuilt around the Corvid identity
> (title, meta, nav and footer links to da1fs/corvid, dynamic release
> API switched to /repos/da1fs/corvid/releases/latest) and now carries
> THE HEART OF THE PROJECT section (why the tool exists for people who
> earn their living through chat networks, the three objectives: the
> network allowlist shield, the bundled free browser per platform, the
> honest local API) plus two new live feature cards (permission shield,
> traffic lens) and a six-service feature line. ES/EN dictionaries
> updated together; the currently published build still installs as
> Zapdude.app and the site says so honestly (the Corvid-branded cut is
> pending; checklist: docs/corvid-rename-checklist.md). Site pushed
> (7217fe1 + e86381f), Pages rebuilt and live-verified by fetching
> https://da1fs.github.io/zapdude-site/. Site repo keeps the
> zapdude-site slug on purpose. (LATER SAME SESSION: the site repo was
> renamed as the user requested: slug is now **da1fs/corvid-site**;
> README live URL updated; the old zapdude-site/ldpf96 pages URLs
> redirect.)
>
> 2026-11-03 - CORVID NAME PASS. The user adopted the free-software
> doctrine (FSF-first wording: "free software", never "open source";
> GNU/Linux naming) and renamed the product to **Corvid**. Display layer
> renamed repo-wide (UI pages, src strings and comments, README, NOTICE,
> macOS plist, CHANGELOG Unreleased note) with all identifiers HELD for
> the rename cut (updater asset matching, HKCU Run key, productName,
> AUMID, config + webview data paths). Webhook headers stay
> X-Zapdude-Signature (protocol contract). 32 unit tests green; commit
> 59801e6 pushed. GitHub state: the user renamed the account
> (ldpf96 -> da1fs), remotes re-pointed, every profile repo except
> corvid and zapdude-site was set PRIVATE on the user's order, the
> project repo is now **da1fs/corvid** (GitHub redirects the old
> address) and the description mentions the Corvid identity. The exact
> remaining rename steps live in docs/corvid-rename-checklist.md. House
> law new: never emit stray non-Latin glyphs (one was scrubbed here).
>
> 2026-11-03 - update policy phase
> 2026-11-03 - UPDATE POLICY + CENTERED UPDATE CARD built on top of the
> existing auto-updater (NOT yet released; the user's PC still runs the
> v1.4.2 published below). WHAT: `AppPrefs.updateMode` ("ask" default |
> "auto" | "off"; sanitized whitelist in config.rs) +
> `update_dismissed_tag`. Updater gates: auto-download is SKIPPED when
> mode == "off" (pure "never automatic": silent checks keep the drawer
> honest, manual buttons remain the only path); an auto-install thread
> spawns on `Ready` when mode == "auto" (re-reads the pref before
> acting, 1.2 s grace beat). The center card `ui/update_nudge.html`
> (webview label `update-nudge` in window.rs; born full-size, pinched
> into the 380x204 CENTER slot by relayout) shows once per tag on phase
> `ready` + `ask` mode. Card buttons: checkbox + "Update now" stores
> "auto" BEFORE the install POST (a fast exit can never outrun the
> promise); checkbox + "No thanks" stores "off" + the dismissed tag
> (permanent); plain "No thanks" dismisses only THIS tag (a newer tag
> re-asks). Plumbing: POST /api/update/prefs {mode?, dismissed_tag?}
> merges into the stored prefs; POST /api/update/nudge/hide closes the
> card; /api/update and /api/prefs carry mode/dismissed_tag; post_prefs
> PRESERVES the stored mode when its body omits it (theme toggles must
> not reset the policy); `updater::set_phase` calls `window::nudge_poll`
> (create/close, idempotent). Drawer gained an "Updates" selector plus a
> "Download and install vX" button on `newer` while mode == "off".
> TESTS: 27 pass (`nudge_policy_is_strict` pins the decision matrix).
> LIVE-VERIFIED with a python stub harness through the browser tool:
> card render dark/light, all four click flows, both silent self-close
> branches (mode off / already-dismissed tag); every POST arrived with
> the exact expected body. Known test-tool quirk, NOT an app issue: the
> harness browser tunnel once returned 502 for one fetch POST; the
> app's WebView2 same-origin POST path is exactly the drawer's current
> one. NEXT SESSION: build + real-app smoke (run-dev cannot run while
> the installed v1.4.2 holds the single-instance mutex + port; the
> schedule stays: first check ~3 min after boot, then every 5 h).
>
> 2026-11-03 continued - EXTERNAL LINKS + INSTAGRAM + META FOCUS GUARD
> (same unreleased epoch, 30 tests passing). WHY: the user reported
> chat links opening INSIDE the account webview (no site sessions =
> fresh login demanded) and a completed Messenger login landing on the
> FULL facebook.com site instead of the chat. WHAT:
> 1) `on_navigation` gate on EVERY account webview (window.rs
>    `embedded_service_url`): the embedded hosts (whatsapp/messenger/
>    facebook/instagram .com + blob/data/about/javascript) navigate
>    in-app; ANYTHING else hands the URL to the user's own browser
>    (crate::browsers::open_external: `cmd /C start "" <url>`, only
>    http/https/mailto ever spawn) and refuses the in-app step; the
>    popup creator denies non-embedded popouts the same way (calls keep
>    their same-session popup).
> 2) `src/browsers.rs`: StartMenuInternet enumeration (HKCU over HKLM,
>    strict key charset, verb command with %1 as the URL slot,
>    broken-verb choices are not offered and any stale choice falls
>    back to the default at click time). GET/POST /api/browsers +
>    prefs.externalBrowser; drawer Preferences row "Chat links open
>    with" (System default + detected list). Unit tests pin the
>    command parser and the key charset.
> 3) `service: "instagram"` (third embedded service): config consts +
>    KNOWN_SERVICES[3], window.rs instagram.com/direct anchor,
>    `scripts/instagram_bridge.js` (the same adaptive __zapdudeApiV1
>    contract as the Messenger bridge: pageInfo/chatList/composerInfo/
>    sendCurrent/titleBadge; multi-strategy selectors, honest degraded
>    reads, login_form signal for the status dot; three_state treats
>    instagram like facebook = no QR stage -> needs_login), rail +
>    drawer + welcome + picker strip carry the IG identity (icon +
>    colors), service sanitize keeps the destination fallback.
>    NOT DONE (needs live DOM proof): IG send/chatList selectors are
>    best-effort by design; the drawer's /api/chats on an IG account
>    degrades honestly until someone verifies the real page markup.
> 4) `scripts/meta_focus.js` ("only the chat", deferred, idempotent,
>    BOTH facebook and instagram): a logged-in feed detour floats back
>    to the chat surface (facebook.com -> messenger.com; instagram.com
>    outside /direct -> /direct). Gates: DOM proof of logged-in chrome
>    (role=feed/navigation), never on in-progress login pages,
>    3-shot sessionStorage cap dead-ends the guard (no loop), SPA
>    history hooks + interval + load re-arm it.
> IMPORTANT CONTEXT: the user does NOT read mid-task progress notes;
> a COMPLETE end-of-task recap is how every session must report.
>
> 2026-11-03 continued again - SECURITY PHASE 1 (user's framing: even
> Windows and Meta should not track the install; AGPL is the license
> branch of this philosophy; "invisible to the OS itself" is out of
> scope for an in-app webview - stated plainly, the honest ceiling is
> "nobody gets more than the strictly necessary"). SHIPPED (32 tests):
> 1) `loopback_origin_guard` middleware over the WHOLE API router in
>    server.rs: Sec-Fetch-Site:cross-site -> 403; an Origin header is
>    allowed only when its host is loopback AND its port == the API
>    port; plain HTTP clients (no headers) keep working. LIVE-PROVEN:
>    cross-site POST 403, wrong-port same-site POST 403, same-origin
>    UI POST 200, keyless POST 200.
> 2) `src/netguard.rs` + wiring in ensure_account_webview (BOTH paths,
>    once per webview label): WebView2 ICoreWebView2_2 Environment +
>    WebResourceRequested handler + 4 filters x 19 curated third-party
>    tracker hosts -> CreateWebResourceResponse(None, 403); the request
>    never leaves the machine. webview2-com API notes (save the next
>    session the archaeology): facade methods are CamelCase (AddWeb-
>    ResourceRequestedFilter), `.cast` needs `use windows_core::
>    Interface`, the import is webview2_com::WebResourceRequested-
>    EventHandler (the callback module is private), CreateWebResource-
>    Response takes &HSTRING refs, and the event closure receives
>    Options and returns Result<()>.
> NEXT PHASES (queued): B) traffic transparency lens (a proof panel
> where the user SEES every host each account touched; trust by
> observation); C) per-service proxy (SOCKS5/HTTP so the chat dials the
> exit node, not the home IP) + optional DoH, research-first (per-
> webview browser args had a gray-composition bug: see BROWSER_ARGS);
> D) fingerprint noise LAST (canvas/WebGL can break page features;
> evaluate with a real login before taking it live).
>
> 2026-11-03 EVENING: THE LADDER DONE (user approved the full list, in
> order, before leaving; 32 tests; live-run on the desktop). PHASE LADDER:
> 1) UX BATCH: see the previous block (z-epoch picker memory, host-level
>    Meta redirects, born-parked accounts, rail instant refresh).
> 2) DRAWER POLISH ("apelmasado" = too tight): section rhythm 28px,
>    column 720px, card 20/22px radius 16, rows with side padding +
>    rounded dividers, update buttons/footer air. Pure CSS; JS intact.
> 3) TRANSPARENCY LENS (phase 4 of the plan): netguard gained the
>    catch-all filters + TRAFFIC ledger (per label, per host: requests,
>    blocked, last_seen; 256-host cap; shed-but-counted semantics) and
>    the tracker decision moved INTO the handler; GET /api/traffic +
>    drawer Security section (plain-language enforcement summary +
>    live host rows + BLOCKED badges, 4 s poll + Refresh button).
>    LIVE-PROVEN: /api/traffic returns real per-account hosts
>    (web.whatsapp.com, static.whatsapp.net, media cdns...).
> 4) X + TIKTOK (phase 3): config consts + KNOWN_SERVICES[5]; window.rs
>    x.com/messages + tiktok.com/messages anchors; embedded aliases
>    twitter.com + tiktokcdn.com; scripts/x_bridge.js + tiktok_bridge.js
>    (same __zapdudeApiV1 contract, TikTok flagged EXPERIMENTAL);
>    meta_chat_redirect branches for both (host-level "only the chat");
>    meta_focus.js CHAT_URLS + inProgressLogin extended (i/flow, oauth,
>    passport); picker 5 tiles (slot 316px), welcome + rail + drawer
>    identities (i-xcorp + i-tt symbols, svc-x/svc-tiktok colors);
>    three_state carries x/tiktok into needs_login law. LIVE-PROVEN:
>    the user created accounts 2/3/4 during testing session.
> 5) uBOL EMBED (phase 6 spike became REAL): repo = uBlockOrigin/
>    uBOL-home (releases carry chromium zips); downloaded v2026.1103.1743
>    (GPL-3.0), unpacked to zapdude-rs/extensions/ublock-origin-lite/
>    (repo) + %APPDATA%\app.zapzap.desktop\extensions (runtime dev
>    path); window.rs extensions_dir() resolves exe-sibling then data
>    dir; account builder mounts with browser_extensions_enabled(true)
>    + extensions_path (wry 0.57 platform API; shell pages stay OFF).
>    LIVE-PROVEN: "extensions dir ready" for every opened account.
>    PENDING LIVE CHECK (user): uBOL's banner/UI presence inside the
>    chat page cannot be seen from the API; his eyes on a chat now.
> KNOWN WARTS (documented, not hidden):
>    - account-ACH payload label shows "account-account-1" in the lens
>      (GET /api/traffic keys are the webview labels); cosmetic, fix
>      next pass if the user cares.
>    - full uBO (MV2) is dead on Chromium; NoScript's interactive model
>      cannot fit embedded surfaces; uBOL is the right fit.
>    - TikTok's anti-automation may degrade its bridge; honest degraded
>      reads are the protocol (no fake success).
>
> 2026-11-03 FEEDBACK FIXES ROUND (user reports, all read carefully and
> confirmed back before work):
> 1) THE + STRIP GHOSTING ("se buguea y deja de aparecer"): root cause
>    found: the parked strip's PAGE self-dismisses on cursor-leave
>    (panel opacity:0 per its own hide() law) and the cached reveal
>    reused the page AS-IS, so the next reveal showed an invisible
>    panel (ghost strip). FIX: the cached reveal RELOADS the local page
>    before relayout (a fresh panel every time, milliseconds: no more
>    webview recreation at all): show_picker now: create-or-reload.
> 2) THE "YOU CAN TELL IT IS A BROWSER" FLASH: replaced by a full BOOT
>    CURTAIN (ui/boot_curtain.html): created immediately with every
>    fresh account webview, and it COVERS the content slot (relayout
>    branch for labels starting "curtain-") with the SHELL's OWN look
>    (logo, spinner, service title). The curtain self-dismisses when
>    its account's session state is HONEST (logged_in / needs_login /
>    scan_qr / page_error) or its 9 s safety timer, through POST
>    /api/ui/bootcurtain/hide; the account swap lands BEHIND it, so the
>    chat page's native white boots never surface. close_account_webview
>    kills the curtain too. The curtain page polls /api/accounts.
> 3) TIKTOK ICON "se ve feo": rebuilt as the BRAND tri-layer (cyan halo
>    offset up-left, red halo down-right, pale core) in picker + rail
>    symbols and the welcome button (inline svg layers). No currentColor
>    dependency anymore.
> 4) X ACCOUNT BLACK SCREEN + dead-feeling tile: the X anchor now opens
>    DIRECTLY on the login flow (x.com/i/flow/login?redirect_after_login
>    =%2Fmessages): a visible sign-in form instead of a silent dark
>    /messages redirect, and the boot curtain covers the arrival + the
>    needs_login state surfaces to the rail dot. (If X still renders its
>    anti-bot wall inside the webview, that is their side; the honest
>    state and the curtain make it legible instead of "black".)
> 5) WHITE-ON-WHITE DROPDOWN POPUPS in the drawer (theme dark): the UA
>    popup used its own scheme; drawer.html now pins color-scheme
>    dark/light per theme and paints select option/optgroup with the
>    card tokens + explicit light variant.
> BUILD STATE: compiles, 32 tests green, relaunched on the desktop
> (healthz 1.4.3, accounts alive, netguard + uBOL mounted per account).
>
> 2026-11-03 RELEASE NIGHT: v1.4.3 PUBLISHED and live for the update
> test (the user's request: THEY update from the button this time).
> RITUAL RAN CLEAN: hygiene scan (no em-dashes in the new doc texts;
> personal phone numbers SCRUBBED to placeholders across 7 files:
> docs/SESSION-STATUS + 6 tools scripts), CHANGELOG Unreleased ->
> [1.4.3] - 2026-11-03, NOTICE.md gained the uBOL attribution (GPL-3.0,
> source tag 2026.1103.1743, folder extensions/ublock-origin-lite),
> 958 files committed (the whole unpacked uBOL copy travels in the repo
> on purpose: reproducible artifact), pushed main + tag v1.4.3 (single
> v this time: the previous v1.4.2 tag was "vv1.4.2", harmless because
> the updater strips leading v chars, but we keep tags clean now),
> tauri build produced target/release/bundle/nsis/Zapdude_1.4.3_x64-
> setup.exe (3.1 MB, exact name the updater's asset matcher wants),
> gh release create v1.4.3 --latest with docs/release-notes-1.4.3.md;
> API verified: latest release = v1.4.3 with the setup asset. The dev
> instance was CLOSED so the installed 1.4.2 can own the mutex: the
> user's update hop is 1.4.2 -> (drawer button flow, since the centered
> card ships WITH 1.4.3) -> 1.4.3. KNOWN GAP for the next pass: fresh
> external installs do not carry the uBOL folder yet (this machine has
> it through appdata); next release should add "extensions" to the
> bundle resources so fresh installs mount uBOL too.
>
> 2026-11-03 UX BATCH (the user approved the full ladder: UX batch ->
> drawer polish -> transparency lens -> X/TikTok -> uBO-Lite spike, ask
> nothing mid-way, document as we go). SHIPPED (compiles, 32 tests):
> 1) THE PLUS GETS A MEMORY: z-epoch law formalized (window.rs Z_EPOCH +
>    PICKER_SEED). Accounts/popups/nudge creation bump the epoch; the +
>    strip webview is PARKED on hide (never destroyed) and revealed
>    INSTANTLY while its birth token equals the current epoch; a newer
>    child owes one recreation (z law preserved).
> 2) META "ONLY THE CHAT" AT HOST LEVEL: account_webview_builder's
>    on_navigation now runs meta_chat_redirect(service,url): facebook
>    account + facebook.com landing (not login/checkpoint/accounts/
>    recover/reg) -> webview.navigate(messenger.com) and the feed nav is
>    refused: the logged-in facebook page NEVER paints. Instagram same
>    (anything not /direct + login paths -> instagram.com/direct). The
>    meta_focus.js DOM guard stays as the safety net.
> 3) NO-FLASH ACCOUNT OPENING: fresh account webviews are BORN PARKED
>    (PARKED_X, full size: paint law), registered in HELD; relayout
>    parks held views even when active; release_hold on
>    on_page_load(Finished) (wired in the builder) + 3.5 s fallback
>    timer + HELD cleanup on close_account_webview. The white/black
>    boot flash of a new account never reaches the screen (the welcome
>    or previous chat covers the wait).
> 4) RAIL INSTANT REFRESH: /api/ui/rail/refresh endpoint evals
>    window.__zapdudeRailRefresh in the rail (rail.html defines it);
>    picker + welcome call it right after /api/accounts succeeds: the
>    new tile appears in a beat instead of waiting the 2.5 s poll.

> Continuity document for AI agents and future sessions. Last updated:
> 2026-11-02 evening - v1.4.2 (the right pane lives forever) built,

> Continuity document for AI agents and future sessions. Last updated:
> 2026-11-02 evening - v1.4.2 (the right pane lives forever) built,
> verified with REAL trusted clicks, installed and PUBLISHED; see the
> STATE OF THE WORLD block below (the 2026-10-30 closing block at the
> end holds the call/notifications history).

> **STATE OF THE WORLD** (2026-11-02 evening): **v1.4.2 PUBLISHED,
> INSTALLED and LIVE on the user's PC** (healthz 1.4.2; accounts alive).
> HEADLINE: the user's "chat disappears / right pane blank" report got a
> STRUCTURAL strike. Root cause proven live, not guessed: the promo
> hider's generic sweep kept painting inline display:none + climbing
> ancestors (1.3.3 covered only the v2 card); during intro->chat
> transitions a transient promo-ish region could collapse a shared
> container and the conversation was born invisible forever. THE FIX
> (hide_download_banners.js FULL rewrite): rule-based hider - stamp
> `data-zapdude-promo` + one stylesheet rule, NO painting, NO climbing;
> stamps are corrected every sweep (unstamped when the copy leaves, a
> functional element appears inside, or the node stops being the
> smallest match). Live-proof battery (all REAL trusted OS clicks,
> synthetic .click() does NOT work on WhatsApp rows anymore):
> boot census (stamps only on promo spans, outside the chat), chat
> opens complete (header + composer, hiddenAncestors=[]),
> THREE chat->chat switches all render with stampsInMain=0, and the
> stamped-node-with-functional-descendant loses its stamp within one
> beat (the anti "born invisible" guarantee). TOOLS: tools/pane-census.js,
> pane-click-probe.js, pane-click-faithful.js, row-click-repro.ps1,
> row-switch-repro.ps1 (the faithful click harness: win32 cursor +
> mouse_event at page-derived coords + rail 75px offset), stamp-cycle.js,
> sweeper-health.js. NOTE: the electron-style row click gate: WhatsApp
> rows now need isTrusted events; agent flows go through the drawer
> search bridge (works). The v1.4.1 instant-shell state below stays true.

> Older STATE block (2026-10-30 mid-session, superseded): v1.2.1 is the
> last PUBLISHED release; the user's PC runs the OLD **v1.2.0** installer
> edition (their "start-menu click does nothing / opens only in the tray"
> symptoms: both fixed since 1.2.1 - second-launch `/api/ui/show` ping +
> manual launches always show the window - and cured entirely by
> installing the 1.3.0). The working tree is **v1.3.0** with the call
> part-2 afternoon folded in (uniform toast law, sw-guard square-
> notification fix, missed-call photo+name, startCall label drift,
> invisible dev launcher); the NSIS 1.3.0 re-cut is running with ALL of
> it (the 09:37 cut is stale, do not install from it). PUBLISH flow:
> local folder is NOT a git repo yet - init + remote + commit + tag
> first; gh is authed as ldpf96 (scope repo).

> **SESSION CLOSE PROTOCOL (2026-10-30)**: the user comps each session;
> keep this block as the entry ramp. Backups: several
> `backups/stable-dev-<stamp>` snapshots exist (tools/backup-stable.ps1;
> the newest 12 are kept). The dev launcher is `tools/run-dev.cmd`
> (ZAPDUDE_DEBUG_EVAL=1, console to zapdude-console.log). The INSTALLED
> app on the user's PC is still the OLD v1.2.0 unless they run the new
> NSIS installer - any double-click of the OLD shortcut silently
> resurrects the old binary (it rewrote config.json to its schema once,
> wiping `service` fields), so v1.3.0 install replaces it.

## Closing block - what v1.3.0 carries (all verified as noted)
## What changed on 2026-10-30 (working epoch, knowledge for future sessions)

- **Toast anatomy rebuilt (user feedback: "la foto sale abajo y gigante",
  "dice account-1 en lugar del nombre").** Root cause: `notifier.rs` used
  `Toast::image()` (tauri-winrt-notification) which ships the image with NO
  placement attribute - Windows renders that as a full-width banner at the
  BOTTOM of the toast. Also the layout was title "Zapdude" + `[account-1]`
  id glued at the tail of the message. New anatomy:
  `.icon(path, IconCrop::Circular, ...)` -> appLogoOverride round avatar at
  the LEFT; contact name as the bold first line (the redundant "Zapdude"
  title was dropped: the AUMID footer already attributes Zapdude); message
  as line 2; account LABEL (renamed via the drawer, resolved through
  `SHARED.settings.current_accounts()` by id) as line 3 only in
  multi-account shells (<= 1 enabled account = no account line, no
  click-through, the footer covers it).
- **Toast click-through**: activation now opens the account that produced
  the event (`open_account_webview`, same choreography as tray
  "Show <account>": window first, webview on a worker thread; re-checks the
  account still exists at click time so a deleted account is not
  resurrected from the snapshot).
- **Verified live** 2026-10-30: 14/14 unit tests green; debug build rebuilt
  and the user's dev instance restarted with the fix; synthetic
  `POST /api/_debug/emit` (kind "incoming", data.kind "notification",
  account-1, data-URL icon) produced the toast on the user's screen and
  decoded the icon to
  `toast-icons/avatar-test_zapdude_toast.png` (533 B = the source icon).
  Live-tested labels: config has account-1 -> "+5550101234" and
  account-2 -> "Account 2" (2 enabled accounts = the line shows).
- **CALL notifications go UNIFORM (user request: "hazla como las de
  mensajes, que se ven muy bonitas").** The incoming-call prompt is
  WhatsApp's own in-page surface, so the branded look now comes from the
  unified pipeline instead: the call anchor script, when a call surface
  mounts, emits the SAME `notification` payload the message notifications
  use (title = caller identity best-effort: `@handle` match or short
  line, body = "Incoming (voice|video) call", and the caller avatar
  decoded from the surface `<img>` blob: URL -> fetch -> FileReader ->
  data URL, which the notifier's icon cache consumes exactly like message
  icons). Result: the SAME branded native toast (round avatar, bold
  name) for incoming calls, usable even for parked/hidden accounts that
  would otherwise ring invisibly. Guards: WeakSet identity dedupe (the
  ongoing-call panel mutates constantly - no toast spam), active
  (answered) surfaces are excluded from toasting, all best-effort. The
  prompt itself stays functional; once the surface outerHTML capture
  lands (next real call), the styling pass gives it the brand look
  without touching the buttons. Build verified 16/16 + backup
  `stable-dev-20261030-092102`.
- **STABLE BACKUPS discipline (the user demanded it after the session's
  rail outage).** NO git repo exists in this working folder, so the anchor
  is: (a) `tools/backup-stable.ps1` - run AFTER a verified build; makes
  `..\backups\stable-dev-<stamp>\` with zapdude.exe + .pdb + source.zip
  (tar excludes target/ + logs) + a MANIFEST.txt of what passed, prunes to
  the newest 12; (b) GitHub release artifacts (CI-built) for real
  releases. Snapshots so far: `stable-dev-20261030-083233` (rail recovery
  + facebook phase 1), `stable-dev-20261030-085537` (+ pinned strip,
  click-open, promo v2 patterns, call-surface HTML capture). HONEST
  NOTE: earlier snapshots of pre-today states were NOT taken on disk -
  the exe changes in place with every build; recoverable prior-stable
  source lives in the v1.2.1 RELEASE (GitHub) + the backups from now on.
- **Session refinement batch (user screenshot feedback, 2026-10-30 late
  morning).** Four items:
  1. Desktop-banner promo leaked on account-2: the hider's Spanish
     patterns covered "descarga la aplicaci[on]" but the umbrella flag
     text was likely the SHORT "descarga la app" - patterns extended
     (descarga/obten/instala + "la app", "download our app", "get our
     app", "WhatsApp Desktop"/"para escritorio", "con whatsapp en tu
     computad[ora]"). The smallest-match rule + collapse climb + protected
     zones stay the safety rails. The bottom banner's X button is an
     svg-only child (promo-ish) so the climb can collapse the bar.
  2. Call notifications look "cuadradas y toscas": the incoming-call
     prompt (ACCEPT/DECLINE) is WHATSAPP's OWN in-page surface (it never
     flows through the Notification API, so the branded native toast
     cannot carry it; missed calls DO). No styling by guess work - the
     call_anchor script now captures the surface outerHTML (6KB cap)
     through the bridge, so the NEXT real call provides the exact classes
     and testids for a brand-styling pass (phase 2 pending that data).
  3. The strip menu sat misaligned vs the + (drawered screenshot): the +
     tile was INSIDE the scroll area, so with few accounts it drifted up
     right after the avatars while the strip anchored to the client
     bottom. FIXED: .add-wrap moved OUT of .rail-scroll as a body-level
     sibling - the + is PINNED to the rail bottom edge always; the strip
     slot geometry (RAIL_WIDTH+6, H-74, 178x74) now hugs it exactly.
  4. The strip now opens by CLICK (+ click -> /toggle), per the user's
     preference; the hover opener was removed entirely. Soft-close grace
     on leaving the + grew to 1200ms (touch pings cancel it - the
     4px gap crossing). The strip ALSO got a self-kill fix: close-on-
     leave now ARMS only after a real mouseenter (the birth-shrink
     synthesized leaves raced the fixed 700ms guard and some mounts
     self-closed the menu before it was used). toggle_drawer now
     dismisses the strip so the options page never has it floating.
- **RAIL SCRIPT CRASH root cause (the user's "accounts disappeared + hover
  gone" report, 2026-10-30).** During the hover-menu wiring, the rail's
  `addWrap` got USED without a declaration AND a duplicate
  `const hoverTimer` sat in the same scope - one ReferenceError under
  `use strict` killed the ENTIRE rail page script at boot: no poll loop,
  no account avatars, no dots, no hover bindings, drawer still worked
  (host-side). LESSON recorded as a hard rule: whenever editing the shell
  pages, `new Function`-style check or at minimum a string sanity pass on
  the served page (index of every referenced identifier) BEFORE shipping;
  the crash shipped because the rail pages have no automated test today.
  Fixed: addWrap declared, duplicate const removed. Verified live: both
  WhatsApp accounts render, dots stable green.
- **Toast click-through now opens THE CHAT, not just the account (user
  report #3: "te lleva a la cuenta, el chat desaparece, gris").** That
  grey pane was WhatsApp's own "no chat selected" placeholder: the old
  click-through switched accounts but never fired the page-side click.
  NEW: after `open_account_webview`, the host fires the notification's
  OWN click handlers on that account page
  (`window.__zapdudeFireNotificationClick(tag)` - the hook stores every
  instance's onclick; empty tag fires empty-tag instances). The call is
  an async retry loop (6 x 300ms) through the page bridge, tolerating
  bridge mounting beats. Hook presence verified live via debug eval:
  `typeof window.__zapdudeFireNotificationClick` -> "function" on
  account-1. A real user toast click is the remaining live proof.
- **Service picker FINAL shape: in-rail HOVER opening a horizontal strip
  (2026-10-30 late).** The user loved the hover menu but wanted it
  extending to the RIGHT ("lo hiciste para arriba en lugar de hacia la
  derecha"). An on-screen CLIP MARKER test proved the rail webview
  CLIPS at its 76px viewport, so an in-rail horizontal menu is
  impossible. FINAL ARCHITECTURE (all verified live):
  - rail.html keeps only the HOVER trigger: + mouseenter (130ms) ->
    POST /api/ui/picker/show; wrapper mouseleave -> POST
    /api/ui/picker/soft-close; + click -> /toggle; document outside
    click + Esc -> /hide.
  - window.rs: `SERVICE_PICKER_WINDOW` child webview ("service-picker").
    show = DESTROY any existing + add_child born at FULL content size
    (paint law) + relayout immediately shrinks it to the strip slot
    (RAIL_WIDTH+6, content.height-78, 178x78) - the born-full-then-
    shrink path PAINTS correctly (captured), the born-tiny path does
    not. hide = close webview (drawer-style recreate per open keeps
    z-order on top of accounts).
  - ui/picker_strip.html: transparent body + rounded dark card with the
    two brand items horizontal. ALPHAWORKS on child webviews (chat
    visible outside the card) although the top-level alpha sheet failed
    earlier. Self-dismissal: mouseleave (guarded against the synthetic
    leave caused by the birth-to-slot shrink - ignore first 700ms), Esc,
    pick. Cursor presence pings /api/ui/picker/touch so the rail's
    soft-close (420ms grace) does not kill the menu while the cursor
    crosses the 4px gap + -> strip.
  - Endpoints: /api/ui/picker/show|hide|toggle|touch|soft-close (the old
    /api/ui/service-picker* names stay gone).
- **Rail dot semantics fixed (rotating-colors bug).** Root cause:
  SESSION_STATE_TTL (2.5s) EXACTLY matched the rail poll cadence, so the
  cached state expired between every poll and list_accounts answered
  "detecting" while the refresh ran - the dot cycled green/grey forever
  (the user's exact report). Fixed with stale-while-revalidate:
  session_state_entry() serves the stale honest value up to a 30s hard
  cap while the refresh runs in the background; the rail keeps the last
  color until the truth changes. New honest vocabulary the user
  specified: green = connected (logged_in), yellow = needs the human
  (scan_qr OR needs_login), red = real page problem (NEW state
  `page_error`, set by spawn_session_state_refresh when a page evaluation
  fails for any reason other than "bridge not installed yet" - that one
  stays detecting = grey). Rail maps page_error to a red dot + a
  "connection problem" tooltip.
- **Real service logos + ATTRIBUTIONS (user request).** Rail account
  tiles show the REAL service glyphs with brand colors: WhatsApp green
  (#25d366) and Messenger blue (#00b2ff) - both paths from simple-icons
  v13 (CC0/public domain), the same source Franz/Ferdium-class apps use;
  nominative identification of the embedded service. The Zapdude bolt
  remains ONLY on the brand logo top-of-rail. The welcome buttons and
  the hover-menu items use the same two logos. **NOTICE.md now carries
  the full attributions** (simple-icons CC0, embedded web services'
  ownership, trademark not-affiliation wording, runtime dependencies
  pointer) per the user's request to attribute everything borrowed.
- **Messenger QR login = NOT POSSIBLE (answered to the user).** Facebook
  has no cross-device QR pairing for the web chat (that is a WhatsApp
  linked-device exclusive). The equivalent smoothness here is the
  per-account persistent WebView2 profile: sign in once (email + password
  + 2FA) and the session survives restarts like a normal browser;
  session_state needs_login only appears when the profile is logged out.
- **Multi-platform chat hub: FACEBOOK PHASE 1 SHIPPED (2026-10-30).** The
  user's doctrine: NO third-party APIs - Zapdude embeds the REAL web chat
  and offers only its own local API on top ("nuestra app interactÃºa
  mediante la pÃ¡gina web"; the API must NEVER break). Implemented:
  - `AccountConfig.service` ("whatsapp" legacy default, "facebook";
    whitelist + WhatsApp fallback so legacy/typo configs can never wedge).
  - `window.rs`: per-service URL (whatsapp -> web.whatsapp.com, facebook
    -> messenger.com, the standalone chat surface so the embedded view
    stays anchored to the chat, never the full Facebook feed. Login
    happens INSIDE the view: email + password + 2FA exactly like the user
    does on Facebook). Deferred scripts are per service: WhatsApp keeps
    its historic set UNTOUCHED (bridge + promos + call anchor +
    notification hook); Facebook gets ONLY the shared notification hook +
    the new adaptive `scripts/facebook_bridge.js`.
  - `facebook_bridge.js` contract: SAME `window.__zapdudeApiV1` name (the
    host's `bridge_expression` guards stay service-agnostic); every op
    try/catches into structured results, selector strategies are
    candidate LISTS (original + fallbacks), honest `degraded` fields
    instead of fake reads. Ops: pageInfo (incl. `login_form`),
    composerInfo, chatList (NOTE: op name must MATCH the WhatsApp bridge's
    - the first cut used `chats` and the host read "bridge not present";
    renamed and verified), sendCurrent (generic composer write, host
    still gates sends), titleBadge + a 2 s document.title unread watcher
    that ships `title_badge` events ONLY on change (observability, NOT
    toasts).
  - host (`server.rs`): `POST /api/accounts` optional body
    `{service:"facebook"}` (legacy no-body callers unchanged); `service`
    returned by /api/accounts AND /api/status; facebook session states:
    logged_in / needs_login (login form detected) / detecting;
    receive-side endpoints (status, chats, events) are service-agnostic;
    WhatsApp-only ops (message, open-chat, call, call/end) answer HTTP
    400 `unsupported_for_service` for other services - explicit "not
    supported yet" beats silent misfires. Events and chat_opened payloads
    carry additive `service` (old consumers keep working).
  - UI: rail "+" no longer silently creates a WhatsApp account - a mini
    picker WINDOW (`ui/service_picker.html` at /picker, new endpoints
    /api/ui/service-picker + /hide, always-on-top borderless ~250x178,
    anchored above the rail's + tile, hides on blur/Esc/main-window
    focus) asks WhatsApp OR Facebook. The welcome screen grew the same
    two brand buttons. Rail avatars show the service glyph (bolt /
    Facebook f) and the tooltip names the service; accounts list rows in
    the drawer show a service chip. `needs_login` maps to the amber dot
    ("waiting for sign in").
  - **Verified live 2026-10-30**: 16/16 tests; dev build running; the
    picker toggled through the API; a facebook account was created
    (account-3) and the REAL messenger page loaded - status reports
    service=facebook, title=Messenger, login_form_visible=true ->
    session_state needs_login; /api/chats on it answers
    ok:true/degraded:true (no threads while logged out - honest); /api/
    message on account-3 answered the structured unsupported_for_service
    error; both legacy WhatsApp accounts still logged_in (nothing
    regressed); a synthetic notification showed the cleaned toast
    WITHOUT the account line.
- **Toast anatomy closure (user feedback: the account line read as if
  the number belonged to the CONTACT)**: the account label is fully OUT
  of the toast text now (contact + message + round avatar only);
  WinRT's attribution row cannot be rebranded per toast (AUMID display
  name), so the account story lives in the click-through: activation
  opens the exact account that produced it (multi-account shells).
- **Pending user actions**: sign into Facebook inside the embedded
  messenger view (email + password + 2FA) to exercise the live scan:
  thread-row selectors, unread heuristics and the notification capture
  need one real login + real incoming messages before SEND for Facebook
  graduates from gated (`unsupported_for_service`) to supported. Verify
  the toast click-through opens the right account when os notifications
  fire for account-2.


## What changed in v1.2.1 (working epoch, knowledge for future sessions)

- **Branded notifications (dual).** Page-level `Notification` on WhatsApp
  webviews is captured by a deferred hook (`scripts/notification_hook.js`,
  flag `__zapdudeNotifyV1`; the DOCUMENT-START variant of this hook once
  killed startup - the deferred slot is the proven safe install point).
  The stub ships notification events through the beacon transport into
  the bus (kind "incoming", payload kind "notification", which kept the
  documented "incoming" contract; the ORIGINAL constructor is never
  called, so WebView2 never shows its web-style toast). `src/notifier.rs`
  routes them: main window visible -> the `toast-overlay` child webview
  (branded stack, `ui/toast.html`, polling `/api/ui/toasts?since=N`,
  click -> `/api/ui/toast/action` -> account focus + page-level onclick
  re-dispatch through `window.__zapdudeFireNotificationClick(tag)`, idle
  -> `/api/ui/toast/release`); main window hidden -> native Windows toast
  via `tauri-winrt-notification` direct (AUMID `app.zapzap.desktop` =
  the registered Start Menu identity; the notification plugin's default
  fell back to the "powershell" attribution the user flagged) with the
  contact avatar cached in `<data>/toast-icons/`.
- **Call surface observations (live, key facts).** As of the observed
  build, placing a call from the webview did NOT open a popup for our
  automated clicks (no window.open observed at any sample; 900 ms
  samples over windows). The call log lands as in-chat bubbles
  (`voip_icce_bubble_1on1_call`, `call-log-system-message`,
  `call-log-icon-chip`) and the callee side registered a real ringing +
  missed-call notification (captured live on the callee account, hook
  working). The user reports the manual call UI as an in-page draggable
  square ("cuadradito"); the passive recorder `scripts/call_anchor.js`
  (flag `__zapdudeCallAnchor`) ships a `call_surface` event through the
  bridge whenever a call surface mounts - the next natural user call
  will give the exact markup to anchor it. Popup windows still exist in
  the app (real windows, `popup-*` labels, shared environment, native
  drag): paths that DO call window.open (media popups and older call
  flows) are handled by `popup_window()` with lifecycle tracking
  (POPUP_INDEX + Destroyed -> `popup_opened`/`popup_closed` events +
  `callRecover` reconciliation prompt on the account page; the recovery
  clicks the "volviendo a la llamada/return to call" control when
  present).
- **Call/chat automation API.** `POST /api/call` opens the chat (drawer
  search flow, auto-opens the account webview when missing) and clicks
  the voice/video header button; `POST /api/call/end` clicks hang-up.
  Chat opening rewrote: `?send?phone=` deep links DROP cold webviews to
  about:blank (unresolvable redirect); the drawer flow
  (`openChatByPhone` -> typing beat -> `pickPhoneResult` -> composer)
  works from any state and matches unsaved numbers AND the own-number
  self-chat row (`message-yourself-row`). Verified live end to end:
  SELF-SEND on account-1 and CROSS-SEND account-2 -> +51 number both
  delivered real WhatsApp messages.
- **Second launch shows the window.** Single-instance mutex now signals
  the first instance through `POST /api/ui/show` (config-file port,
  retries). Autostart Run key value became `"<exe>" /startminimized`;
  only auto-launched processes honor the start-minimized pref: manual
  clicks always open the window. Verified: the Run key migration happens
  best-effort at every startup (prefs on) and /api/ui/show works live.
- **Bridge lifecycle fixed.** `PAGE_BRIDGES`: closing and re-creating an
  account webview KEPT the first bridge (dead target, eval timeouts
  forever). New `page_ops::replace`/`remove`; the beacon poller exits
  when the bridge is cancelled (`cancelled` flag on PageBridge).
  Verified: account-2 close/open cycle recovers.
- **Chat list scraper follows the 2025-10 markup.** Rows:
  `#pane-side [data-testid^="list-item"]`, title `span[title]` first,
  snippet `cell-frame-secondary span[title]`, unread badge
  `[data-testid="icon-unread-count"]` (digits from its aria-label,
  locale-independent). Verified live with real chats (288/311 unread
  group).
- **Debug harness.** Under `ZAPDUDE_DEBUG_EVAL=1`:
  `POST /api/_debug/eval` (raw expression eval via the page bridge with
  the defensive wrapper), `GET /api/_debug/windows` (windows + webviews),
  `POST /api/_debug/windows/{label}/close` (shell core refused; account
  labels cancel their bridge first), `POST /api/_debug/emit` (synthetic
  bus events for SSE/webhook/toast testing). Debug auth skips the rate
  limiter (monitors poll fast) but keeps the API key + enabled checks.
  Helper scripts live in `tools/` (see debug-eval.ps1 for evaluation;
  sse-listen.cmd + webhook-receiver.ps1 = the verification battery).
- **Promo hider hardened.** The generic sweep abandoned the bare
  "Download"/"Descargar" exact-text match (it erased the REAL Download
  button of the in-page media viewer overlay, the "download option
  disappears" report); dialogs/menus are protected zones now. The
  landing flow keeps its stricter copy+card logic.

## What changed in v1.2.0 (knowledge for future sessions)

- **App-wide theme.** Each account view's WebView2 profile pins
  `ICoreWebView2Profile::PreferredColorScheme` (LIGHT/DARK) to the shell
  theme: applied at view creation (`ensure_account_webview`) and on
  preference changes (`post_prefs`). WhatsApp Web follows it live (it
  listens to `matchMedia` changes) without reloading or losing the
  session. Native caption bars follow via `Window::set_theme` on the
  main window and pop-outs. Implemented in `window.rs`
  (`apply_theme_to_message_views`, `apply_window_theme`,
  `apply_theme_everywhere`), hooked in `src/api/server.rs` `post_prefs`.
  New dependencies (windows-only): `webview2-com = "0.39"`,
  `windows-core = "0.62"` (same version family tauri/wry already
  resolved). Verified live via PrintWindow captures: light boot, dark
  toggle, light back, session intact (`session_state: logged_in`
  throughout).
- **macOS support.** Tauri 2 multi-webview DOES run on macOS 14+ via
  private APIs (unstable feature), which rules out App Store
  distribution but works for direct distribution. `tauri.conf.json`
  pins `bundle.macOS.minimumSystemVersion: "14.0"`, `icons/icon.icns`
  (already generated) is part of the icon list. Read the docs before
  repeating the old "macOS discarded" claim - that is OBSOLETE.
- **CI**: `.github/workflows/release.yml` builds from a tag
  (+ workflow_dispatch): macOS aarch64/x86_64 (--bundles app,dmg) and
  Windows (--bundles nsis). tauri-apps/tauri-action@v0, GITHUB_TOKEN,
  releaseDraft: false. Runtime observed ~7.5 minutes, 3/3 green.
- **Release copy bug worth remembering**: PowerShell 5.1 `Get-Content`
  wraps the string with PSObject metadata (PSPath, PSParentPath...)
  and `ConvertTo-Json` serializes the wrapper. Use
  `[System.IO.File]::ReadAllText()`, then `ConvertTo-Json`, then send
  `[Text.Encoding]::UTF8.GetBytes($json)` as the request body. Both
  release bodies (v1.1.0 title "Full English release", v1.2.0) were
  PATCHed clean through the API after this bug.

## Current user state

- Since 2026-11-02: the user's PC runs the INSTALLED **v1.4.2**
  (`C:\Users\PC\AppData\Local\Zapdude\zapdude.exe`) with the autostart
  Run key (`/startminimized`). The auto-updater lives inside it: next
  published tag auto-downloads and a one-click install replaces the
  binary. Dev builds remain available via `tools\run-dev.cmd` (debug
  binary at `target\debug\zapdude.exe`, `ZAPDUDE_DEBUG_EVAL=1`,
  console captured to `zapdude-console.log`).
- Session note: the instance currently running was started WITH
  `ZAPDUDE_DEBUG_EVAL=1` (debug endpoints live, still key-gated +
  loopback-only). A normal boot does not set the env, so debug
  endpoints stay OFF; set the env before launching when the harness is
  needed (the verified click harnesses below depend on it).
- The loopback API stays ENABLED in
  `%APPDATA%\app.zapzap.desktop\config.json` (port 29977, key rotated
  2026-10-29); the updater endpoints ride it management-style (no key).
- All v1.2.1 features were verified against the DEV build with real
  traffic and real sends (self-send, cross-send account-2 -> +51). The
  installed v1.2.0 exe will gain them only through the v1.2.1
  installer.
- The family Mac project (sister-in-law, other user): macOS **13.7.8
  Ventura, Intel**. The `Zapdude_1.2.1_x64.dmg` was handed over (user's
  downloads folder, SHA-256 verified against the release asset). The
  Ventura run is the pending live test of the macOS 13 minimum gamble:
  awaiting their report (window up, QR, chats readable). If Ventura
  misbehaves: plan B is redesigning the shell for macOS (one window
  per account) or asking for a macOS 14 update.
- The user prefers Spanish in chat; repo text stays English.
- Windows is in dark mode; the in-app theme follows its preference.

## Requirements from the user (stable, in importance order)

1. Multi account is CRITICAL: all WhatsApp accounts in one app.
2. Premium human interface (green brand, no browser chrome).
3. His name (ldpf96) on the credits.
4. Local loopback API for humans and AI agents (REST + SSE + signed
   webhooks).
5. Calls/video/screen sharing (Chrome 156 JS identity).
6. No terminal on launch, NSIS installer, clean uninstall.
7. "Never break": failures degrade, never crash.
8. Publish on GitHub with professional release copy.
9. English everywhere in the repo; Spanish in chat only.

## Paths

- Project: `C:\Users\PC\Desktop\Projects\Zapdude\zapdude-rs`
- Runtime config: `%APPDATA%\app.zapzap.desktop\config.json`
  (identifier stays `app.zapzap.desktop` to preserve user data).
- WebView2 profiles: `%APPDATA%\app.zapzap.desktop\webview\<account-id>`
- ZapZap reference copy (READ ONLY): `C:\Users\PC\Desktop\Projects\
  Zapdude\zapzap-main`

## Architecture snapshot (see README + architecture.md)

- ONE window "main" (Tauri 2 `unstable` multi-webview): rail webview +
  welcome webview + N account webviews + a lazy full-screen drawer
  webview. Active content webview sits at `(RAIL_WIDTH, 0)`; idle
  content webviews park at `x=-24000` logical at FULL SIZE (never
  hidden). Geometry watchdog re-aligns every 2 s and on events.
- The loopback API server (axum) SERVES the shell pages: same-origin
  fetches, no Tauri IPC for the UI.
- Tray: per account "Show", "Hide all", "Quit". Startup order:
  single instance mutex, server, shell, first account.
- Account/popup labels: `account-<id>` / `popup-<account>-<n>`.

## Lessons that must not regress

1. Webviews born hidden or tiny stay gray FOREVER: born at full
   content size, then parked by position only (never hide()).
2. Custom `additional_browser_args` + Chrome UA on child webviews
   breaks WebView2 painting (do not ship custom args).
3. Keep document-start scripts minimal; install the rest deferred
   through the beacon poller with per-script idempotency flags.
   (The Notification hook in particular MUST stay deferred: at
   document-start it once killed WhatsApp startup.)
4. WebView2 `ExecuteScript` arrives double encoded when the value is a
   String - unwrap twice. Also: a returned PROMISE resolves fine, but
   plain JS objects jumped straight from async code inside one eval do
   not ship - keep browser-op scripts synchronous, pace with the host.
5. WebView2 COM work MUST happen on the main thread: use
   `webview.with_webview(...)` (that is what the theme pinning uses).
6. Debug harness: PrintWindow with `PW_RENDERFULLCONTENT` (2) captures
   DirectComposition content of background windows without stealing
   focus; bisection env var `ZAPDUKE_WEBVIEW_OPTS` remains valid.
7. Check `github.com/settings/tokens` facts before claiming platform
   limits: several past conclusions ("macOS impossible") were wrong
   and got fixed in the same session they were re-verified.
8. Bridge ops receive POSITIONAL arguments as rendered by
   `bridge_expression` - passing an object-shaped args string makes the
   op see "[object Object]" (the "phone_too_short" live bug).
9. The `?send?phone=` WhatsApp deep link can drop a cold webview onto
   about:blank; the drawer search flow is the reliable opener.
10. API rate limit (60/min) is shared with live debug monitors - the
    debug auth path skips it, keep debug polling on that path.
11. Crates with unguarded target deps go in
    `[target.'cfg(windows)'.dependencies]`, not `[dependencies]`
    (tauri-winrt-notification drags windows 0.62 -> windows-future 0.3.2
    into the macOS graph and both mac CI jobs failed to compile; the
    gated move restored the green build).
12. The user's family Mac runs macOS 13.7 (Ventura): the v1.2.1 build
    opened LSMinimumSystemVersion to 13.0 and shipped usage-description
    plist keys for mic/camera/screen. State on Ventura is EXPERIMENTAL
    (the multi-webview engine is verified on macOS 14+); the release body
    and Pending item 5 carry this.

## Git / GitHub / auth state

- Remote: `https://ldpf96@github.com/ldpf96/zapdude.git` (username-
  anchored).
- **Secret hygiene (2026-10-29)**: the day's helper scripts initially
  hard-coded the loopback API key and got pushed (tools/debug-eval.ps1,
  tools/sse-listen.cmd). Scrubbed the same session: both helpers now
  read the key at runtime from
  `%APPDATA%\app.zapzap.desktop\config.json`, and the API key was
  ROTATED afterwards - the key inside the pushed git history is DEAD
  (no value left for anyone). The key is only meaningful to local
  processes anyway (the API binds 127.0.0.1 unless expose_network is
  switched on). Keep the runtime-read pattern for any future helper.
- Two credentials exist locally: Git Credential Manager OAuth
  (`gho_...`, scopes `gist, read:org, repo` - NO workflow scope) and a
  classic PAT the user pasted in chat (scopes `repo, workflow`). The
  PAT was used inline (URL-embedded) for the v1.2.0 push and was NOT
  stored in the keyring; recommended the user revoke it after use.
- Pushes that CREATE/UPDATE workflow files require the `workflow`
  scope (any token lacking it gets rejected at the remote). Code/docs
  pushes work with the stored OAuth token.
- GitHub CLI exists at `C:\Program Files\GitHub CLI\gh.exe` but is not
  on PATH; invoke with the full path if needed.
- The v1.2.1 tag push DID fire the release workflow (the tag-delete +
  re-tag cycle also works cleanly).

## Pending (the NEXT session opens on these)

1. DONE 2026-11-02: **v1.4.0, v1.4.1 AND v1.4.2 published, installed and
   verified** (the 1.4.2 published in 7m45s CI, all five assets; body
   patched; site pill pushed; real-click verification on the INSTALLED
   cut passed with zero stamps inside the conversation).
2. **Optional dress rehearsal**: the full drawer check -> progress ->
   Install button flow has not yet been run BY THE USER on a real
   newer release; cut any next release and let them drive it (the
   updater machinery itself is verified end to end: install script +
   silent NSIS + relaunch ran live during the 1.4.0 cut).
3. 1.3.2 treasures live (verified twice now): unread badges + drawer
   toggle (and they saved the user's sanity while the gray bug hid).
4. Facebook phase 2: only meaningful once a Facebook account exists -
   re-add via the "+" strip when wanted; phase-1 contract stays
   untouched.
5. Backlog (carried): Linux ubuntu matrix row in release.yml, macOS
   toast path (NSUserNotification), own-number endpoint; GitHub
   hosted-runner capacity flakiness on macOS x86_64 (rerun recovers,
   consider a pin or retry for that job if it repeats).
6. Light-tier leftovers cleaned: the drawer no longer touches GitHub
   directly; `verCmp`/`currentVersion`/`GITHUB_LATEST` live only in
   history (no dead code left in drawer.html).
7. If the blank pane ever returns (WhatsApp is a moving target): the
   1.4.2 hider CANNOT produce it structurally (no painting, no
   climbing, reversible stamps); suspect WhatsApp-side mount failures
   first and use tools/pane-census.js + row-switch-repro.ps1 (they
   redeem the DOM state around the click).

## Old pending (from the 1.2.1 era)


1. Install v1.2.1 on the user's Windows PC FIRST (replace the v1.2.0
   install; remember the single instance mutex). Everything verified
   today ran on the dev build, not the installed one.
2. Family Mac report: did the v1.2.1 x64 DMG open and pair on Ventura
   13.7.8? (lsmin 13.0 experiment; the release body carries the caveat -
   feed the outcome into docs and, if needed, plan the macOS shell
   redesign.)
3. Call surface anchor phase 2: the passive recorder
   (`scripts/call_anchor.js`) ships `call_surface` events whenever a
   real call surface mounts. With the user's next natural call, take
   that data and pin the in-page call panel (no drag inside the app
   box). The popup path is already recovered automatically
   (`callRecover` clicked the pill live; the reload failsafe exists but
   remained unused so far).
4. `own_number`: a first-class endpoint for each account's own number
   (DOM does not expose it; candidates: WAM state, settings deep-dive).
   The cross-send direction account-1 -> account-2 still needs the +1
   callee number the user has not shared yet.
5. Push-event coverage from the SERVICE WORKER path (hidden-window
   notifications may only fire through sw.js; the current capture covers
   page-level Notification calls). No COM interception attempted: the
   native toast fallback covers the hidden case already.
6. macOS toast path (NSUserNotification) so Mac builds float avatars like
   Windows does now; also call pop-out windows on macOS (window.open
   handling without WebView2 environment sharing).
7. Linux builds: add an ubuntu matrix row (webkit2gtk-4.1 dependency
   set) to `.github/workflows/release.yml`.
8. User-side checks they agreed to feel: close-to-tray then double
   click the installed icon (window must come back), and a media
   download sanity pass (the option must stay visible).

## Useful commands

```powershell
cd C:\Users\PC\Desktop\Projects\Zapdude\zapdude-rs
cargo test
cargo build --release
Get-Process -Name zapdude | Stop-Process -Force   # before rebuilding
curl http://127.0.0.1:29977/api/healthz
curl http://127.0.0.1:29977/api/accounts
```

## Environment notes

- PowerShell: cargo/tauri write to stderr (cosmetic
  NativeCommandError noise; judge by content and exit code).
- `Start-Process` needs absolute paths.
- git: `C:\Program Files\Git\bin\git.exe` (PATH may not be refreshed).
- The user speaks Spanish; answer in Spanish in chat, but the REPO
  TEXT stays English per his standing decision.

> 2026-11-03 UPDATE-PATH REAL TEST + v1.4.4: the user updated 1.4.2 ->
> 1.4.3 through the built-in updater (the drawer button flow) and
> reported the polish round (plus spam ghosting, browser-tell flashes,
> tiktok icon look, X black screen, white-on-white selects, the ugly
> "127.0.0.1 says" dialog, twitch wish + the permission-wall idea).
> ALL LANDED in v1.4.4 (shields in prefs + /api/shield + drawer rows +
> permission_response consultation; twitch service full identity +
> twitch_bridge.js anchoring the logged-in site; strip recreate-per-open
> with drain retry, z-epoch cache removed; drawer two-tap install
> confirm, no native dialog; curtain service branding + account-id
> params; netguard display label). v1.4.4 PUBLISHED (asset
> Zapdude_1.4.4_x64-setup.exe, --latest): the user's installed 1.4.3
> will surface THE CENTERED CARD for this hop (the don't-show-again
> checkbox flow finally live from the "ask" default). NEXT: watch the
> card UX + ask + auto + never behaviors.

> ---- SESSION CLOSE (2026-11-03 late evening) ----
> WORLD STATE: the user's PC runs the INSTALLED Zapdude 1.4.4 (updated
> from 1.4.3 through the self-updater), process alive at the moment of
> closing (healthz green, version 1.4.4). The dev tree equals the
> published v1.4.4 (main is pushed at the doc settle commit; repo:
> github.com/ldpf96/zapdude). Stable backup:
> backups/stable-dev-20261103-190032 (exe + pdb + source snapshot +
> manifest note). 32 unit tests green at close.
> What v1.4.4 carries (shipped): update policy card (ask/auto/off +
> don't-show-again checkbox flow), external links through the user's
> browser + browser choice, SIX embedded services (whatsapp, facebook,
> instagram, x, tiktok EXPERIMENTAL, twitch), permission shield (camera/
> mic/screen/clipboard switches + /api/shield), security lens panel
> (/api/traffic), tracker blocklist, boot curtain with per-service
> branding, + strip recreate-per-open with drain retry, two-tap install
> confirm in the drawer (no native dialog), select popups theme-bound,
> drawer breathing pass.
> WHERE WE LEFT OFF / NEXT SESSION RAMP:
>   1. The user was about to test the CENTERED UPDATE CARD flow live
>      (the 1.4.3 -> 1.4.4 hop: ask default, checkbox semantics, two
>      policies). Collect reactions + fix whatever seems off.
>   2. Twitch bridge selectors are UNPROVEN on a real login (receive-
>      first doctrine; degrade until anchored). Ditto TikTok (kept
>      EXPERIMENTAL by declared risk).
>   3. uBOL FRESH-INSTALL GAP: the installer does not ship the
>      extensions folder yet (this machine works via appdata). Next
>      release: add "extensions" to bundle.resources in tauri.conf.json
>      so every fresh install mounts uBOL too.
>   4. Queued from the security ladder: per-service proxy (research
>      first: webview2 env proxy), fingerprint hardening LAST, DoH.
>   5. Candidate smalls: X anchor deep-link already lands on their
>      login flow (watch real sign-ins), picker speed TESTS on replay,
>      curtain wording polish.
> OPERATING LAWS THIS SESSION (keep carrying): sessions live in appdata
> only (never in builds/repos), publish ritual = hygiene scan + number
> scrub + secret scan + tests + release notes WITHOUT em-dashes or
> decorative ticks, tags single-v, the user reads NOTHING mid-task:
> end-of-task recap is the channel. V identity + naming rules per
> AGENTS.md remain.

> ---- SESSION CLOSE (2026-11-04) ----
> v2.0.0 ON THE SLAB: THE AUTOMATION API IS REMOVED, ON PURPOSE.
> Trigger: a terms-of-service review in chat (WhatsApp/Meta/X/TikTok/
> Twitch all prohibit unofficial automation; enforcement history
> verified: Meta sued scraper vendors, WhatsApp C&D'd client makers,
> wrapper wave got fresh legal attention in Oct 2026). The user drew
> the line himself: gray territory fine, legal problems not.
> THE CUT (all landed, cargo check clean, 26/26 unit tests green):
> - src/api: page_ops.rs, protocol.rs, events.rs DELETED; links,
>   ring, SSE, webhook, key auth, rate limiter, debug eval harness
>   GONE. server.rs rewritten as the shell's own loopback surface
>   (127.0.0.1 ALWAYS, same-origin guard kept).
> - src/api/link.rs NEW: slim page link (beacon poller + telemetry
>   ingest + state probe + call-pill recovery), page_for/status_
>   snapshot/reconcile_after_popup_close keep their names so the
>   call sites barely moved.
> - scripts: the 5 service bridges + api_page_bridge.js DELETED;
>   page_link.js NEW (window.__zapdudeLinkV1); notification_hook +
>   unread_watcher + call_anchor switched to the new name; sw_guard
>   lost its dead diagnostic push; client_hints/call_style/meta_
>   focus/sw_guard/hide-download unchanged.
> - config.rs: the `api` section is GONE; top-level `port` with a
>   one-time carry-over from legacy `api.port` (test included). No
>   key, no webhook, no network exposure.
> - main.rs/window.rs wired to the slim link; popup lifecycle
>   events removed (the ring died with them); POPUP_INDEX stays for
>   the call recovery.
> - UI: drawer API settings + AI agent panels REMOVED; welcome +
>   About copy now say the app has no automation surface.
> - docs: README rewritten (What Corvid does NOT do + a ToS honesty
>   section per service, and the TikTok/"Meta's anti-automation"
>   factual slip fixed: TikTok's anti-bot is TikTok's own),
>   architecture.md updated to the slim flow, NOTICE.md updated,
>   tools/release-body.md refreshed, NEW docs/RED-LINES.md (the
>   anti-regression law + the C&D protocol + the compliant
>   reintroduction path), CHANGELOG carries the 2.0.0 entry.
> - Hygiene: BOM purged from 7 pre-existing files (SESSION-STATUS +
>   tools/*.js: the 65279 killer stays at zero), zero em-dashes in
>   the tree.
> WHERE WE LEFT OFF / NEXT SESSION RAMP:
>   1. THE REPO CUT (user pre-approved "borrar y recrear con v2.0.0",
>      still owed its final explicit GO): delete da1fs/corvid (0
>      forks, 0 stars, deletes v1.0.0..v1.4.4 + binaries), recreate
>      fresh single-commit history, tag v2.0.0, watch CI, verify the
>      release. corvid-site copy needs the API feature removal +
>      live-URL check the same day.
>   2. Version normalization: Cargo.toml + tauri.conf.json already
>      at 2.0.0; the release workflow asset naming/patterns check.
>   3. Run a REAL login session on the slim build (notifications +
>      badges + dots + a call): confirm the page link behaves like
>      the old bridge on the telemetry side.
>   4. Rename cut (Corvid binary/AUMID/paths) still waits on its own
>      checklist; the red-lines doc must ride along unchanged.
> OPERATING LAWS THIS SESSION (further): the repo language stays
> English; the ToS honesty section keeps zero lawyer tone; every
> page hook must be able to name its field in the audit table in
> docs/RED-LINES.md.
