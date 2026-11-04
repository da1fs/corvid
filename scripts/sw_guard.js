/**
 * Zapdude service-worker notification guard.
 *
 * ROOT CAUSE (user report + live probe, 2026-10-30): WhatsApp's own
 * service worker (https://web.whatsapp.com/sw.js) raises ring and push
 * notifications from its OWN realm through
 * `registration.showNotification`, which renders as the OS-native SQUARE
 * bubble the branded pipeline can never touch (the page-level
 * Notification stub in notification_hook.js cannot reach a worker
 * realm). The user saw it stack BEHIND the branded toast.
 *
 * The page cannot patch the worker's internals, so this guard removes
 * the relay at the REGISTRATION point:
 *   1. intercepts `navigator.serviceWorker.register` - the original
 *      promise still resolves (WhatsApp's startup flow keeps working)
 *      and the registration unregisters immediately after, and
 *   2. sweeps already-registered workers on a short interval to catch
 *      lazy or late registrations.
 *
 * Degrade-never-crash: every step is best-effort inside try/catch; the
 * app itself runs over its own WebSocket, and the page-level
 * notification capture keeps feeding the branded pipeline, so nothing
 * user-visible depends on the worker being alive.
 */
(function () {
    'use strict';

    if (window.__zapdudeSwGuardV1) {
        return;
    }
    window.__zapdudeSwGuardV1 = true;

    function sweep() {
        try {
            var sw = navigator.serviceWorker;
            if (!sw || typeof sw.getRegistrations !== 'function') {
                return;
            }
            sw.getRegistrations().then(function (regs) {
                var killed = 0;
                (regs || []).forEach(function (reg) {
                    try {
                        if (reg && typeof reg.unregister === 'function') {
                            reg.unregister();
                            killed += 1;
                        }
                    } catch (_sweepError) {}
                });
                 // Gentle proof for the live session: turned out to feed
                 // only the removed event ring, so the pushes are gone;
                 // the guard's own verification lives in the DOM result.
            }).catch(function () {});
        } catch (_outerError) {}
    }

    try {
        var sw = navigator.serviceWorker;
        if (sw && typeof sw.register === 'function') {
            var originalRegister = sw.register.bind(sw);
            sw.register = function () {
                try {
                    var pending = originalRegister.apply(null, arguments);
                    if (pending && typeof pending.then === 'function') {
                        pending.then(function (reg) {
                            try {
                                if (reg && typeof reg.unregister === 'function') {
                                    reg.unregister();
                                }
                            } catch (_lateError) {}
                        }).catch(function () {});
                    }
                    return pending;
                } catch (_registerError) {
                    // The page never even reached the original register.
                }
            };
        }
    } catch (_patchError) {}

    sweep();
    window.setInterval(sweep, 2000);
})();
