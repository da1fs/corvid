# Zapdude 1.4.2 - the right pane lives forever

A structural strike on the "chat disappears" failure. **AGPL-3.0.**

## Fixed

- The right pane can no longer stay blank after clicking a chat. The
  promo hider's generic sweep still painted inline `display:none` and
  climbed ancestors (the 1.3.3 fix covered only the empty-state
  card), so during WhatsApp's intro-to-chat transitions a transient
  region full of promo-ish children could get collapsed, and the
  conversation then mounted inside a hidden container: the pane was
  born invisible and nothing ever un-hid it.
- The whole promo hider is rule-based now: nothing is painted,
  nothing is climbed. Hiding is a reversible stamp
  (`data-zapdude-promo` attribute) plus one stylesheet rule, and the
  sweep CORRECTS itself every pass: any stamped node loses the stamp
  the moment its copy changes, a functional element (composer, chat
  list, media controls) appears inside it, or it stops being the
  smallest match. A wrong catch can flicker; it can never hide a
  conversation forever.
- Verified live with real (trusted) mouse clicks: a chat opens
  complete from the intro, three consecutive chat-to-chat switches all
  render the header and the composer, and zero promo stamps exist
  inside the conversation at every step.

## Downloads

- Windows: `Zapdude_1.4.2_x64-setup.exe` (NSIS, per-user install)
- macOS Apple Silicon: `Zapdude_1.4.2_aarch64.dmg`
- macOS Intel: `Zapdude_1.4.2_x64.dmg`

Zapdude is free software under GNU AGPL-3.0 - source on
[github.com/da1fs/corvid](https://github.com/da1fs/corvid).
Not affiliated with WhatsApp, Messenger or Meta Platforms.
