# Corvid voice notes (how the project speaks in public)

Corvid is maintained by one person in spare time, and it is software
libre under AGPL-3.0. That fact defines the voice more than any style
guide: the text should sound exactly like a one-person project that has
nothing to hide and no marketing operation behind it. Promises that the
project cannot keep are the enemy, both for trust and for the author.

These notes apply to every public surface: the site, the README and
docs, release notes, the app's own UI copy, and every future social or
issue text.

## The rules

1. **Present tense only for what is verified today.** Describe what the
   build in front of you does. Plans get "we are working on" or
   "planned", no matter how close they look. Never write a roadmap item
   in present tense (the site once described the strict network
   allowlist while the code shipped the curated list; both exist, they
   are not the same, the text must not blend them).
2. **Absolute words are rationed.** Never, always, forever, never
   breaks, instant, guaranteed, 100%, risk-free: these are only allowed
   where a fact holds. Two facts hold: the AGPL-3.0 license keeps the
   code free, and the architecture has no project server (your chats
   move between your computer and the service, like in a browser).
   Everything else rephrases: "no se rompe" becomes nothing; "la
   sesion queda viva" becomes "se mantiene la sesion entre cambios".
3. **State the limits inside the feature.** The netguard docs carry
   their own limits in-module; public copy does the same wherever a
   list or a shield appears: curated list, static, services still see
   their own pages' traffic. An honest limit reads as competence, not
   weakness.
4. **Breakage has a path.** Public copy never claims absence of
   failure; it names what to do: open an Issue, watch Releases. One
   sentence of "this is young, report what breaks and it gets fixed in
   public" is the whole legal and reputational shield the project
   needs.
5. **Software libre vocabulary, FSF-first:** "software libre" (or "free
   software" in English text), GNU/Linux for the system, "Linux" only
   for the kernel, AGPL-3.0 named when freedom is claimed. Never "open
   source".
6. **Facts re-verified at publish time.** Versions, minimum OS, asset
   names, feature truth: check the file, the code or the release
   itself. Copy ages fast (the site said macOS 14 required for months
   after the minimum became 13; the latest tags shipped without dmg
   while the page offered them).
7. **Say who did what.** da1fs is the author's handle (formerly
   ldpf96). ZapZap by Rafael Tosta is the design reference and gets
   named. Development assistance is disclosed as OpenCode + GLM-5.3
   Flash (always the full model name). Trademark disclaimers stay.
8. **Hygiene, always:** no em-dashes, no decorative symbols, no stray
   characters from other alphabets, repo text in English and chat in
   Spanish, release notes describe the version and nothing else.

## Rewrites that were learned the hard way

| Before (what went wrong) | After |
|---|---|
| "tus cuentas a un clic y nada que se rompa" | "tus cuentas a un clic. Lo demas se esta construyendo: si algo falla, se reporta y se corrige en publico" |
| "Eso pasa una sola vez" (macOS quarantine) | "ocurre otra vez con cada instalador nuevo que descargues" |
| "la desinstalacion deja la carpeta de datos limpia" | same, plus the truth that folder holds your sessions, so copy it first if you want to keep them |
| "Gratis para siempre" x4 in one page | once, in the fine print, anchored to AGPL-3.0 |
| "un cafe se metaboliza en mas cosa buena" | "un cafe se convierte en mas cosas utiles" |
| "Voz, video y pantalla compartida funcionan" | "funciona en WhatsApp hoy; los demas servicios muestran su chat sin automatizar llamadas aun" |
| "en GNU/Linux la cadena entera ya es libre" | "la cadena para compilar desde fuente ya es libre; los instaladores empaquetados aun no existen" |

## Before publishing, check

- Every claim survives one question: si el usuario me muestra lo
  contrario, que respondo? If the answer is "tenia razon y yo menti",
  rewrite it first.
- The site and the repo say the same thing about status, platforms and
  minimums.
- Nothing promises what the next release might not deliver.
- The only "forever" left in the text belongs to the license.
