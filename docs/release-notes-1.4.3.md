# Zapdude 1.4.3

Release date: 2026-11-03.

This release closes three fronts at once: the update experience the
community asked for, a safety floor for non-technical users, and the first
multi-service chapter (five chat surfaces in one window). All of it was
built with AI assistance (OpenCode + the GLM-5.3 Flash model by Z.ai) and
verified on real hardware before shipping.

## Updates: a choice the user holds

- The auto-updater grew a behavior corner: ask (default), auto or never.
  When a verified installer is ready, a centered card ("New version vX is
  ready") asks ONCE per release with a checkbox: checkbox + Update now
  stores "install automatically from now on" BEFORE the install begins;
  checkbox + No thanks means "never download or install on its own" and
  the card stops appearing; plain No thanks dismisses only this release.
  The drawer's Updates selector makes that choice reversible at any time,
  and "never" is honored to the letter (no background downloads: updates
  stay manual only).
- Update failures still collapse into honest messages; the running app is
  only touched after an explicit install.

## Links that respect your sessions

- Chat links no longer open inside Zapdude's embedded view (that view
  carries no site sessions, which meant stranger login prompts). Embedded
  chat domains stay in-app; every other link opens in YOUR browser with
  your cookies, and the drawer lets you pick the browser (system default
  by default, detected from the Windows registry).

## Two new embedded services + one guard

- X (Twitter) DMs and TikTok DMs joined WhatsApp, Messenger and
  Instagram: five possible surfaces, each with its own isolated profile,
  adaptive page bridge and identity. TikTok ships as EXPERIMENTAL: its
  anti-automation may degrade its automated reads; the surface degrades
  honestly instead of breaking.
- A "only the chat" law now runs at two levels: a logged-in landing on
  the full facebook.com, instagram.com, x.com or tiktok.com site is
  refused and the view floats back to its chat surface before the feed
  can paint.

## Security: observable, not magical

- The embedded pages stay sandboxed (no file access, no geolocation),
  known third-party trackers are blocked before leaving the machine, the
  local API refuses requests from remote sites and other local ports, and
  a new Security lens in the options drawer shows every host each account
  contacted, live, with blocked rows marked.
- A boot curtain with the Zapdude look covers account loading, so the
  chat page's own white/dark boot screens never surface.
- uBlock Origin Lite is embedded per account profile (GLP v3 copy, source
  link and attribution in NOTICE.md; see the license note there).

## Feel

- The "+" menu now opens instantly (the strip stays warm), brand-new
  accounts load invisibly and swap in ready, the rail refreshes the new
  tile in a beat, the options drawer breathes with wider rhythm, and the
  unread badges, call pop-outs and tray flow keep working as before.

## Data and privacy notes

- Accounts, cookies and sessions live in the user's app-data folder,
  never inside the app or the repository. Updating from 1.4.2 to 1.4.3
  keeps that folder untouched: every account survives the upgrade with
  its session intact.
- Ships as the usual Windows setup (per-user install, silent update path
  included): Zapdude_1.4.3_x64-setup.exe.
