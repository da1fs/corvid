# Zapdude 1.4.0 - it updates itself

The app now watches its own GitHub Releases and installs the newest
build with one click. **AGPL-3.0.**

## Added

- **Auto-updater (middle tier).** Zapdude checks
  `github.com/da1fs/corvid/releases/latest` every few hours (and the
  first time a few minutes after boot). When a newer version is out it
  downloads the Windows installer by itself into
  `%APPDATA%\app.zapzap.desktop\updates\`, verifying the byte size
  against the release manifest and the executable header before
  trusting a single byte. The options screen (the brand logo) now
  shows the whole cycle: new version detected, download with progress,
  verified and ready, and a single **Install and restart** button that
  quits the app, runs the installer silently and brings the window
  right back. Nothing installs without that explicit click.
- **Instant rail switching.** The account tiles used to wait for the
  background poll (up to ~2.5 s) before moving the active shading to
  the account you had just clicked; the highlight now lands the
  instant you click, looking as fast as the switch it is.

## Agents

- `GET /api/update` reports the phase machine
  (idle / checking / newer / downloading / ready / installing / failed).
- `POST /api/update/check`, `POST /api/update/download` and
  `POST /api/update/install` drive each step.
- Bus event `update_state` carries every phase change to SSE and
  webhook listeners.

## Downloads

- Windows: `Zapdude_1.4.0_x64-setup.exe` (NSIS, per-user install)
- macOS Apple Silicon: `Zapdude_1.4.0_aarch64.dmg`
- macOS Intel: `Zapdude_1.4.0_x64.dmg`

Zapdude is free software under GNU AGPL-3.0 - source on
[github.com/da1fs/corvid](https://github.com/da1fs/corvid).
Not affiliated with WhatsApp, Messenger or Meta Platforms.
