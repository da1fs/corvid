# Release body template flow (v2.x era)

Per-release bodies live in the repo: use `docs/release-notes-<version>.md`
as the body source (see the 1.4.x notes for the house format: headline,
what changed, download list with the exact asset names CI uploads,
closing license line; the "agents section" of those old notes described
the automation API the project no longer ships, so 2.0.0+ notes do not
carry one unless a compliant surface ever returns).

Download page: https://da1fs.github.io/corvid-site/

**Windows** - `x64-setup.exe`, per-user NSIS installer, no admin needed.
**macOS** - `.dmg` for Apple Silicon (aarch64) and Intel (x64), macOS 13
(Ventura) or newer. The first open asks for one confirmation because
builds ship without paid Apple signing, and it asks again on each new
downloaded build (a re-download carries a fresh quarantine stamp).

**Source** - https://github.com/da1fs/corvid (AGPL-3.0; not affiliated
with WhatsApp or Meta).

Patch reminder: the PATCH endpoint needs the release ID, not the tag
(`PATCH /releases/{id}`; `PATCH /releases/tags/x` is a 404).
