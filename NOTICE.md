# NOTICE - Corvid

Copyright (c) 2026 da1fs (https://github.com/da1fs)
License: **GNU Affero General Public License v3.0** (see `LICENSE`).

## Origin and credits

- **Design reference.** Corvid's interface was designed with
  [ZapZap](https://github.com/rafatosta/zapzap) by Rafael Tosta as its
  reference: the account rail, one-click account creation, opening
  straight into the QR screen and the tray flow follow that model.
- **Implementation.** Corvid is an original implementation written in
  Rust for Tauri 2 (WebView2/WKWebView/WebKitGTK). It shares no source
  with ZapZap, which is a Python/GTK application, and the product has
  since grown into its own shape: a full-screen options drawer,
  instant account switching, per-account call pop-outs, a permission
  shield and a network lens.
- **Development tooling.** The author implemented Corvid with AI
  assistance (the OpenCode agent harness driving the GLM-5.3 Flash model by
  Z.ai), reviewing and verifying every change on real hardware before
  release.
- **License rationale.** AGPL-3.0 was chosen so that anyone building on
  Corvid - or serving a modified version to others over a network -
  shares the corresponding source.

## Third-party assets and attributions

- **Service icons in the shell (rail tiles, add-account menu, welcome
  buttons).** The WhatsApp and Messenger glyphs are taken from
  [Simple Icons](https://simpleicons.org)
  ([github.com/simple-icons/simple-icons](https://github.com/simple-icons/simple-icons),
  version 13): CC0 1.0 Universal (public domain dedication). The icons are
  used to IDENTIFY which chat web each account embeds (nominative use in a
  third-party desktop client). They remain trademarks of their owners and
  this does not imply endorsement. More icons will join this same CC0
  source as new services are embedded. The X and TikTok glyphs added in
  1.4.3 come from the same CC0 source (simple-icons X and TikTok paths).
- **Embedded uBlock Origin Lite (shipped with 1.4.3).** The update and
  content blocking experience bundles a copy of
  [uBlock Origin Lite](https://github.com/uBlockOrigin/uBOL-home) version
  2026.1103.1743 ("uBOLite_2026.1103.1743.chromium.zip", the official
  chromium build by Raymond Hill and the uBOL-home contributors),
  published under **GPL-3.0-only**. Per its license, the source it was
  built from is that exact repository and release tag
  (github.com/uBlockOrigin/uBOL-home/releases/tag/2026.1103.1743; the
  extension files live in the "extensions/ublock-origin-lite" folder of
  this repository). The extension runs ONLY inside the per-account
  WebView2 profiles of the chat pages and is loaded as an unpacked,
  unmodified distribution. AGPL users distributing Corvid must keep the
  folder and this notice intact.
- **Runtime dependencies.** All Rust crates (Tauri 2, wry, axum, tokio,
  ureq, `tauri-winrt-notification`, serde, sha2/hmac/base64 and the rest
  of the dependency graph) keep their own license terms and are listed
  with versions in `Cargo.toml` / `Cargo.lock`; their notices apply as
  shipped by `cargo`.
- **Embedded web services.** WhatsApp Web, Messenger and any other web
  chat the app embeds are the property of their operators; Corvid
  displays them inside a webview and automates nothing beyond the page
  surface the site itself renders.

## Trademark and ownership

- Corvid is an independent project and is **not affiliated with,
  endorsed or sponsored by** WhatsApp, Messenger, Facebook, Meta
  Platforms, Inc., or any other chat-service operator. The names
  **WhatsApp**, **Messenger** and **Facebook** are trademarks of their
  respective owners; the app only displays embedded views of your own
  sessions, and your data stays yours.
- **ZapZap** is and remains the project of Rafael Tosta.

## License

Copying, modifying and redistributing Corvid is free under the terms
of the GNU AGPL v3. Modified works stay AGPL-3.0 or later.
