/**
 * Zapdude - hide WhatsApp Web's "download the app" promos.
 *
 * RULE-BASED VERSION (the 1.3.3 gray-pane lesson made universal).
 * The previous rule-based fix covered ONLY the empty-state card; the
 * generic sweep kept element painting (inline display:none) plus
 * ancestor climbing; during WhatsApp's intro->chat React transitions a
 * transient "promo-ish only" region could get collapsed, and the
 * CONVERSATION then mounted INSIDE a display:none ancestor: the chat
 * never rendered (the user's blank right pane with the row still
 * selected). Nobody un-hides inline styles, so the pane dies forever.
 *
 * This version can never do that:
 *
 *   1. NOTHING is ever painted. The script only adds/removes a
 *      `data-zapdude-promo` attribute, and one stylesheet rule hides
 *      stamped elements: `[data-zapdude-promo]{display:none!important}`.
 *   2. Stamps are REVERSIBLE and corrected EVERY sweep: a node loses its
 *      stamp the moment it stops matching the promo copy, gains a
 *      functional descendant (composer, chat list, media controls...),
 *      or is no longer the smallest match. A transient over-catch
 *      flickers at worst; it can never stay hidden forever.
 *   3. NO ancestor climbing, ever. A container is hidden only if the
 *      copy ITSELF matches. Functional territory ( QR, chat list,
 *      composer, overlays, images ) is untouchable in both directions.
 *   4. The v2 empty-state card hides by pure selector rule (its own
 *      testid), same as 1.3.3 - the card dies with its rule, matches
 *      nothing while unmounted, and the conversation view is never part
 *      of it.
 */

(function () {
    'use strict';

    if (window !== window.top) {
        return; // top frame only
    }
    if (window.__zapdudeHidePromos) {
        return; // already installed
    }
    window.__zapdudeHidePromos = true;
    // Live counter for debug evaluations: the census can ask how many
    // stamps exist without knowing the stylesheet mechanics.
    window.__zapdudePromoSweeps = 0;

    var PROMO_PATTERN = new RegExp(
        [
            'descarga\\w*\\s+(la\\s+aplicaci|whatsapp)',
            'descarga\\w*\\s+la\\s+app\\b',
            'download\\s+(the\\s+app|whatsapp)',
            'download\\s+our\\s+app\\b',
            'obtener\\s+la\\s+(aplicaci|app)',
            'obt\\w*\\s+la\\s+app\\b',
            'get\\s+(the|our)\\s+app',
            'instala\\w*\\s+(la\\s+aplicaci|whatsapp)',
            'instala\\w*\\s+la\\s+app\\b',
            'install\\s+(the\\s+app|whatsapp)',
            'usar\\s+whatsapp\\s+en\\s+tu',
            'use\\s+whatsapp\\s+on\\s+your',
            'whatsapp\\s+para\\s+windows',
            'whatsapp\\s+for\\s+windows',
            'whatsapp\\s+(desktop|para\\s+escritorio)',
            'con\\w*\\s+whatsapp\\s+en\\s+tu\\s+computad',
            'use\\s+whatsapp\\s+(on|from)\\s+your\\s+computer',
            'disponible\\s+para\\s+windows',
            'available\\s+for\\s+windows',
            'obten\\w*\\s+funciones\\s+adicionales',
            'funciones\\s+adicionales',
            'get\\s+extra\\s+features',
            'extra\\s+features\\s+like'
        ].join('|'),
        'i'
    );

    /// Standalone promo buttons ("Descargar", "Download") - part of the
    /// LOGGED-OUT landing flow only (below). The generic logged-in sweep
    /// never touches bare single-word buttons: WhatsApp's media viewer
    /// carries REAL Download buttons whose aria-label is just "Download".
    var BUTTON_PROMO_PATTERN = /^\s*(descarga\w*|download|obtener|get|instala\w*|install)\s*$/i;

    /// WhatsApp's inline SVG icons carry a <title> like "ic-download" that
    /// textContent concatenates onto the label ("Downloadic-download").
    /// Strip those tokens (and collapse whitespace) before exact-text tests.
    function cleanText(raw) {
        return String(raw || '')
            .replace(/ic-[\w-]+/g, ' ')
            .replace(/\s+/g, ' ')
            .trim();
    }

    var CANDIDATE_SELECTOR =
        'div, aside, header, section, span, p, h1, h2, h3, button, a';

    /// Chat/composer/overlay territory is off-limits: nothing inside these
    /// gets stamped, whatever it says. Overlays (dialogs and menus) carry
    /// functional media controls; the media viewer's real Download button
    /// was the victim of an exact-text sweep before this file existed.
    function inProtectedZone(node) {
        try {
            return !!node.closest(
                '#pane-side, div[role="listitem"], [contenteditable="true"], ' +
                '[contenteditable], #main, div[role="textbox"], canvas, input, textarea, ' +
                'div[role="dialog"], div[role="menu"], div[role="tooltip"]'
            );
        } catch (_error) {
            return true; // when in doubt, protect
        }
    }

    /// Absolute no-stamp roots: the shell surfaces WhatsApp mounts every
    /// view into. A stamp on (or inside) any of these is the bug the user
    /// keeps reporting: the right pane pays for one cosmetic card.
    function isAppLevel(node) {
        for (var n = node; n; n = n.parentElement) {
            var id = n.id || '';
            if (id === 'main' || id === 'pane-side' || id === 'app' ||
                id === 'wa-web-main-screen') {
                return true;
            }
        }
        return false;
    }

    function matches(node) {
        try {
            if (inProtectedZone(node)) {
                return false;
            }
            // Phrase matches only: the bare "Download"-button rule lives in
            // the landing flow below where it cannot reach overlays (the
            // media viewer keeps real Download buttons outside #main).
            var own = (node.getAttribute('aria-label') || '') + ' ' + (node.textContent || '');
            own = own.slice(0, 400);
            return PROMO_PATTERN.test(own);
        } catch (_error) {
            return false;
        }
    }

    /// The one stylesheet rule of the whole feature. Stamp = harmless
    /// attribute; the rule does the hiding; unstamp = visible again.
    function ensureRuleStyle() {
        try {
            if (document.getElementById('zapdude-promo-rule')) {
                return;
            }
            var style = document.createElement('style');
            style.id = 'zapdude-promo-rule';
            style.textContent =
                '[data-zapdude-promo]{display:none!important}';
            document.head.appendChild(style);
        } catch (_error) {
            // Never break the page because of cosmetics.
        }
    }

    /// "Functional" content: a live part of the page the user drives. A
    /// stamped node that contains ANY of these must lose its stamp the
    /// same beat, whatever its copy says (WhatsApp may reuse containers
    /// across views).
    function containsFunctional(node) {
        try {
            return !!node.querySelector(
                '#pane-side, #main, [contenteditable="true"], [contenteditable], ' +
                'canvas, input, textarea, video, iframe, svg[data-icon], ' +
                'div[role="listitem"], div[role="option"], button:not([data-zapdude-promo])'
            );
        } catch (_error) {
            return true; // when in doubt, do not hide
        }
    }

    function sweep() {
        try {
            window.__zapdudePromoSweeps += 1;

            // 1) UNSTAMP pass first: the truth anchor. Anything stamped that
            //    stopped matching (or became functional) becomes visible at
            //    once - reversible correctness beats one-way cosmetics.
            var stamped = document.querySelectorAll('[data-zapdude-promo]');
            for (var s = 0; s < stamped.length; s++) {
                var st = stamped[s];
                var keep =
                    matches(st) &&
                    !containsFunctional(st);
                if (!keep) {
                    st.removeAttribute('data-zapdude-promo');
                }
            }

            // 2) STAMP pass: only the SMALLEST matching candidate per
            //    region. No climbing: the match itself, nothing above it.
            var candidates = document.querySelectorAll(CANDIDATE_SELECTOR);
            for (var i = 0; i < candidates.length; i++) {
                var node = candidates[i];
                try {
                    if (node.hasAttribute('data-zapdude-promo') || !matches(node)) {
                        continue;
                    }
                    if (node === document.body || node === document.documentElement) {
                        continue;
                    }
                    // Functional territory check BOTH directions: inside a
                    // protected zone (closest) or holding one (querySelector).
                    if (inProtectedZone(node) || containsFunctional(node)) {
                        continue;
                    }
                    // Only the smallest element per matching subtree: skip
                    // containers when a descendant already matches.
                    var descendants = node.querySelectorAll(CANDIDATE_SELECTOR);
                    var smallest = true;
                    for (var j = 0; j < descendants.length; j++) {
                        if (matches(descendants[j])) {
                            smallest = false;
                            break;
                        }
                    }
                    if (!smallest) {
                        continue;
                    }
                    // A promotion-worthy candidate gets the attribute; the
                    // stylesheet rule hides it THIS frame. Reversible.
                    node.setAttribute('data-zapdude-promo', '');
                } catch (_error) {
                    // Never break the page because of cosmetics.
                }
            }

            hideLandingPromoCard();
        } catch (_error) {
            // Never break the page because of cosmetics.
        }
    }

    /// Landing-page promo card ("Download WhatsApp for Windows" next to the
    /// QR). Only runs on the LOGGED-OUT landing (no chat list, no chat pane):
    /// find the control that just says "Download"/"Descargar" and stamp the
    /// smallest ancestor with promo copy (the whole card). No painting, no
    /// climbing: a wrong stamp is undone by the next sweep; a functional
    /// element (canvas/QR, inputs, editables) stops the loop, so the QR card
    /// can never be stamped.
    function hideLandingPromoCard() {
        // Visibility-based guard: WhatsApp's React shell may keep a HIDDEN
        // #main/#pane-side in the DOM even on the logged-out landing, so a
        // mere existence check would disable this flow forever.
        var landingVisible = false;
        try {
            var probe = document.querySelectorAll('#pane-side, #main');
            for (var i = 0; i < probe.length; i++) {
                if (probe[i].getClientRects().length > 0) {
                    landingVisible = true;
                    break;
                }
            }
        } catch (_error) {
            landingVisible = true; // when in doubt, assume logged-in UI is up
        }
        if (landingVisible) {
            return; // logged-in UI: the generic sweep already handles promos
        }
        var buttons = document.querySelectorAll('button, a, [role="button"]');
        for (var b = 0; b < buttons.length; b++) {
            var btn = buttons[b];
            if (btn.hasAttribute('data-zapdude-promo')) {
                continue;
            }
            if (inProtectedZone(btn) || isAppLevel(btn)) {
                continue;
            }
            var text = cleanText(btn.textContent);
            if (text.length > 12 || !BUTTON_PROMO_PATTERN.test(text)) {
                continue;
            }
            var card = null;
            var node = btn;
            for (var depth = 0; depth < 8 && node; depth++) {
                node = node.parentElement;
                if (!node || node === document.body || node === document.documentElement) {
                    break;
                }
                if ((node.textContent || '').trim().length > 400) {
                    break; // text budget: this is a layout, not a card
                }
                if (PROMO_PATTERN.test(node.textContent || '')) {
                    card = node; // smallest enclosing element with promo copy
                    break;
                }
            }
            var target = card || btn;
            if (!inProtectedZone(target) && !containsFunctional(target) && !isAppLevel(target)) {
                target.setAttribute('data-zapdude-promo', '');
            }
        }
    }

    // The v2 empty-state promo card (real testid census 2026-10-31): the
    // whole "Descarga WhatsApp para Windows / Obtén funciones adicionales,
    // como llamadas… / [Descargar]" en bloc carries its own testid. HIDDEN
    // VIA A SELECTOR RULE, never by painting an element: this card sits
    // HIGH in the right pane's tree, next to containers the CONVERSATION
    // view also mounts into. A stylesheet rule dies with the card itself:
    // while the card is unmounted the rule matches nothing, and the chat
    // view is never touched. The functional action-tile row (Enviar
    // documento / Añadir contacto / Pregúntale a Meta AI) is untouched.
    function ensureIntroCardStyle() {
        try {
            if (document.getElementById('zapdude-intro-card-style')) {
                return;
            }
            var style = document.createElement('style');
            style.id = 'zapdude-intro-card-style';
            style.textContent =
                '[data-testid="intro_panel_v2_title_card"]{display:none!important}';
            document.head.appendChild(style);
        } catch (_error) {
            // Never break the page because of cosmetics.
        }
    }

    var scheduled = null;
    function scheduleSweep() {
        if (scheduled) {
            return;
        }
        scheduled = setTimeout(function () {
            scheduled = null;
            sweep();
        }, 600);
    }

    function start() {
        sweep();
        ensureRuleStyle();
        // The empty-state promo card hides by selector rule - no element
        // painting, no ancestor climbing (the gray-pane lesson).
        ensureIntroCardStyle();
        var observer = new MutationObserver(scheduleSweep);
        observer.observe(document.documentElement, { childList: true, subtree: true });
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', start);
    } else {
        start();
    }
})();
