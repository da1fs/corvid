# Zapdude 1.4.1 - every button answers at once

The 1.4.0 account-switch speed is now the standard for every button in
the shell: each press answers on the click frame. **AGPL-3.0.**

## Changed

- The brand logo flips its green ring the instant it is clicked; the
  full-screen options panel follows right behind (the click lands, the
  ring confirms, the panel appears in the same breath).
- The "+" tile fills instantly on press while its service strip builds;
  the account tiles and the welcome-screen buttons gained the same
  pressed-scale feel.
- The options drawer (the X button or Escape) fades out at the click
  frame; picking WhatsApp or Messenger in the "+" strip collapses the
  card immediately (and comes back if the account creation fails, so an
  optimistic dismissal never hides a real error).
- The settings "Save" button confirms the click instantly; the "saved"
  toast follows the round trip.

## Agents

- `GET /api/accounts` now carries `shell: { drawer_open, picker_open }`
  so clients can mirror the shell's own button states instead of
  guessing (the rail flips optimistically and resyncs on the poll).

## Downloads

- Windows: `Zapdude_1.4.1_x64-setup.exe` (NSIS, per-user install)
- macOS Apple Silicon: `Zapdude_1.4.1_aarch64.dmg`
- macOS Intel: `Zapdude_1.4.1_x64.dmg`

Zapdude is free software under GNU AGPL-3.0 - source on
[github.com/da1fs/corvid](https://github.com/da1fs/corvid).
Not affiliated with WhatsApp, Messenger or Meta Platforms.
