# Corvid rename checklist

Goal: complete the Zapdude -> Corvid rename at the next release cut
without breaking installed users, sessions or the auto-updater.

Already done (display pass, this commit):

- All UI pages (rail, drawer, welcome, boot curtain, picker strip,
  toast overlay, update card) read Corvid.
- Source comments and visible strings (window title, tray "Quit
  Corvid", API info name, netguard reason, error surfaces) read Corvid.
- macOS Info-privacy.plist permission strings read Corvid.
- README and NOTICE updated (credits now point to github.com/da1fs).
- Kept untouched on purpose: the webhook contract headers
  (`X-Zapdude-Signature`, `Zapdude-API/1` user agent), the app
  identifier (`app.zapzap.desktop`), the config and webview data paths,
  and the updater asset matching.

Remaining items (execute all in one release cut):

1. `tauri.conf.json`: `productName` "Zapdude" -> "Corvid". This changes
   the NSIS artifact to `Corvid_<ver>_x64-setup.exe`, the install folder
   and the Start menu entry.
2. `.github/workflows/release.yml`: release name "Zapdude ..." ->
   "Corvid ...". The body underneath is already canonical (live site
   URL, da1fs/corvid source link, macOS 13 text) as of the copy pass.
3. `src/updater.rs`: asset matcher must accept BOTH `Zapdude_*` and
   `Corvid_*` installers for one bridge release, so installed 1.4.x
   clients can hop to the Corvid build; drop the Zapdude pattern only
   after the feed proves everyone moved. DONE as of the copy pass:
   `installer_asset_priority` accepts both spellings (exact first,
   v-prefixed second), with tests; the feed URL is canonical
   (api.github.com/repos/da1fs/corvid/releases/latest).
4. `src/window.rs`: HKCU Run key value "Zapdude" -> "Corvid" (both the
   set and the cleanup branches), and verify the leftover old value is
   removed on first run.
5. AUMID: keep `app.zapzap.desktop` (toast identity, config continuity);
   the display name Windows shows comes from the new product name;
   verify a live notification reads Corvid.
6. `zapdude` binary/package name (Cargo) and `versions` in CI paths:
   optional deep rename; only do it in the same commit as the CI path
   updates, never alone.
7. GitHub repo slug rename: DONE 2026-11-03 (da1fs/corvid; redirects
   active). README links, `corvid-site` links and CI badges now point
   at the canonical addresses.
8. `corvid-site` (formerly `zapdude-site`): the site repo is renamed
   and live at https://da1fs.github.io/corvid-site/. PENDING in that
   repo: the copy pass for the publish truth (macOS 13 minimum, honest
   macOS build state, project status line) and the asset-name swap
   once the Corvid build is live (the page must accept BOTH product
   asset spellings until the hop completes). NOTE: the "old pages URLs
   redirect" belief under which the former site pass was written is
   FALSE for GitHub Pages (ldpf96.github.io/zapdude-site answers 404),
   so every published reference now uses the live URL.
9. Release notes for the first Corvid tag: identity paragraph, no
   invented history for old releases.
10. Live-verify battery: build the NSIS artifact, install over an
    existing 1.4.4, run one update hop, then a notification + tray +
    autostart + uninstall pass.

Standing rules: nothing here is published while unpublished; the cut
happens only as a planned release. Before any tag: scan for BOM bytes
(0xEF 0xBB 0xBF, the 65279 character CI fatally rejects) across the
json/toml/yml sources, plus the usual publish ritual (hygiene scan,
number scrub, secret scan, tests). The v1.4.3 and v1.4.4 CI runs died
exactly on the Cargo.toml BOM (already scrubbed on main by 1596c3b; the
tags predate that commit, their re-run would fail again): the next tag
cut from clean main is the one that must prove the macOS .dmg legs
before the release is announced as carrying them.
