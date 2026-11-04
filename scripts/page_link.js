/**
 * Zapdude page link.
 *
 * The SLIM resident page helper that survived the local automation API
 * removal (v2.0.0): no message sending, no chat list reads, no call
 * starting, no phone book scraping. What remains is pure telemetry and
 * self-healing, the page-neighborhood a chat hub needs to feel native:
 *
 *   - pushEvent(payloadJson): outbound events from the page watchers
 *     (Notification capture, unread badge sampling) into the host
 *     through the beacon transport.
 *   - stateInfo(): the tiny page state snapshot (URL, title, composer
 *     presence, QR canvas, password field) that drives the rail dots
 *     and the boot curtain. No contacts, no conversations, no text.
 *   - callRecover(probeOnly): one recovery gesture for a STUCK call pill
 *     after the call popup window closed the hard way (session-safe
 *     reload as the failsafe). Serves human-initiated calls only.
 *
 * Every entry point returns a plain JSON-able object and never throws:
 * a changed WhatsApp markup must degrade the helper, never the page.
 *
 * Injected as a deferred, idempotent script by the beacon poller.
 *
 * Event transport (in order of preference):
 *   1. window.__zapdude.respond(serializedJson) - native bridge object
 *      when the host exposes one (WebView2 host-injected helper).
 *   2. location hash beacon (`#__zapdude=<urlencoded payload>`) polled
 *      by the host through WebviewWindow::url().
 */
(function () {
    'use strict';

    var LINK_ID = '__zapdudeLinkV1';

    function emitError(operation, error) {
        return {
            ok: false,
            error: 'unexpected_error',
            detail: String((error && error.message) || error || ''),
            operation: operation,
        };
    }

    /**
     * Composer presence. The WhatsApp-specific selectors stay first
     * (historic behavior, never regress); the generic contenteditable
     * fallback covers the chat editors of the other embedded chat webs
     * (Messenger, X, TikTok, Twitch) without carrying their markup.
     */
    function findComposer() {
        const selectors = [
            'footer div[contenteditable="true"]',
            'div[role="textbox"][data-tab="10"]',
            'div[role="textbox"][data-tab="6"]',
            'div[role="textbox"][contenteditable="true"]',
            'div[contenteditable="true"][data-tab]',
        ];
        for (const selector of selectors) {
            const nodes = document.querySelectorAll(selector);
            for (const node of nodes) {
                try {
                    if (node.offsetParent !== null && !node.closest('div[role="dialog"]')) {
                        return node;
                    }
                } catch (_hiddenCheckError) {
                    return node;
                }
            }
        }
        // Generic fallback: a visible free-text editor outside dialogs.
        try {
            const generic = document.querySelectorAll(
                '[contenteditable="true"], textarea'
            );
            for (const node of generic) {
                if (node.isContentEditable || node.tagName === 'TEXTAREA') {
                    if (node.offsetParent !== null) {
                        return node;
                    }
                }
            }
        } catch (_genericError) {}
        return null;
    }

    function decodeFinalUrl() {
        try {
            return String(location.href || '');
        } catch (_error) {
            return '';
        }
    }

    function hashBeacon(payload) {
        try {
            location.replace('#__zapdude=' + encodeURIComponent(payload));
            return true;
        } catch (_beaconError) {
            return false;
        }
    }

    function emitResponse(id, payload) {
        var envelope = JSON.stringify({ id: String(id), payload: payload });
        try {
            // WebView2 host helpers may be exposed via AddHostObject once
            // a later version enables them; prefer direct call first.
            if (window.__zapdude && typeof window.__zapdude.respond === 'function') {
                window.__zapdude.respond(envelope);
                return;
            }
        } catch (_respondError) {
            /* fall through to beacon */
        }
        hashBeacon(envelope);
    }

    (function build() {
        const hidden = function (node) {
            try {
                return !(node && node.offsetParent !== null);
            } catch (_visError) {
                return true;
            }
        };

        const link = {
            version: 1,

            /**
             * Outline page level state for the shell (rail dots, boot
             * curtain). Deliberately read-only: no DOM scraping beyond
             * these presence checks.
             */
            stateInfo: function () {
                try {
                    let chatList = false;
                    try {
                        chatList = !!document.querySelector(
                            '[aria-label="Chat list"], #pane-side, div[role="listitem"]'
                        );
                    } catch (_chatListError) {}
                    let qrcode = false;
                    try {
                        qrcode = document.querySelectorAll('canvas').length > 0;
                    } catch (_qrError) {}
                    let passwordField = false;
                    try {
                        const passwords = document.querySelectorAll(
                            'input[type="password"]'
                        );
                        for (const node of passwords) {
                            if (!hidden(node) && !node.closest('div[role="dialog"]')) {
                                passwordField = true;
                                break;
                            }
                        }
                    } catch (_pwError) {}
                    return {
                        ok: true,
                        url: decodeFinalUrl(),
                        title: String(document.title || ''),
                        chat_list: chatList,
                        composer: !!findComposer(),
                        qrcode: qrcode,
                        login_form: passwordField,
                    };
                } catch (error) {
                    return emitError('stateInfo', error);
                }
            },

            /**
             * Reconcile the page after its call popup vanished without
             * going through WhatsApp's own end-call flow (the popup X
             * button bypasses it, the call keeps ringing/burning on the
             * page side, and the "Llamada / Llamando..." pill stays on
             * screen with nothing reachable - the stuck state seen live).
             *
             * Phase 1: click the leftover pill (WhatsApp then re-issues
             * its own call window/shutdown path). Phase 2 (probe mode):
             * report whether a pill would still match, without clicking -
             * the host uses it to decide a page reload as the failsafe.
             */
            callRecover: function (probeOnly) {
                try {
                    var pill_pattern = /\b(\d+\s*min|llamando|calling|reanudar|volviendo|volver a la llamada|return to call|rejoin|llamada en curso|llamada actual|current call|rejoin call)/i;
                    var candidates = document.querySelectorAll(
                        'div[role="button"], button, [role="button"]'
                    );
                    var probe = [];
                    for (var index = 0; index < candidates.length; index++) {
                        var node = candidates[index];
                        try {
                            var label = String(node.getAttribute('aria-label') || '');
                            var text = String(node.textContent || '').replace(/\s+/g, ' ').trim();
                            var composed = (label + ' ' + text).trim();
                            if (!pill_pattern.test(composed)) {
                                continue;
                            }
                            var rect = node.getBoundingClientRect();
                            if (rect.width < 40 || rect.height < 30) {
                                continue;
                            }
                            probe.push(composed.slice(0, 60));
                            if (probeOnly) {
                                return { ok: true, in_call: true, probe: probe };
                            }
                            node.click();
                            return { ok: true, recovered: 'clicked', probe: probe.slice(0, 6) };
                        } catch (_candidateError) {}
                    }
                    return { ok: true, in_call: false, probe: probe.slice(0, 6) };
                } catch (error) {
                    return emitError('callRecover', error);
                }
            },

            /**
             * Dispatch an outbound page event to the host (from the
             * Notification capture, the unread badge watcher, and the
             * call-state watcher).
             */
            pushEvent: function (eventJson) {
                var event = null;
                try {
                    event = JSON.parse(eventJson);
                } catch (_error) {
                    event = { kind: 'unexpected' };
                }
                emitResponse(
                    'evt_' + String(Date.now()) + '_' + Math.floor(Math.random() * 1e6),
                    event
                );
                return { ok: true };
            },
        };

        window[LINK_ID] = link;
    })();
})();
