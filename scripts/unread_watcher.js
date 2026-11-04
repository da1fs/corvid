/**
 * Zapdude unread badge source.
 *
 * Every account page samples its OWN document title and ships the unread
 * counter it carries: WhatsApp Web publishes "(n) WhatsApp" natively and
 * Messenger does the same. One truthful, self-maintaining number per
 * account - no chat list selectors to drift with chat-web redesigns. If a
 * future title format drops the "(n)", the count becomes 0 and the badge
 * simply stays hidden (degrade, never fake data).
 *
 * Ships only on CHANGE (3s pacing, page-side timer): the beacon never
 * floods. Delivery goes through the page link helpers (with a raw
 * hash-beacon fallback that works even while the link helper mounts).
 */
(function () {
    'use strict';

    if (window.__zapdudeUnreadV1) {
        return;
    }
    window.__zapdudeUnreadV1 = true;

    var last = null;

    function ship(count) {
        try {
            if (window.__zapdudeLinkV1 && typeof window.__zapdudeLinkV1.pushEvent === 'function') {
                window.__zapdudeLinkV1.pushEvent(JSON.stringify({
                    kind: 'unread',
                    count: count,
                }));
                return;
            }
        } catch (_shipError) {}
        // Raw fallback (the same envelope the page link would emit): the
        // badge never dies while the page link helper is still mounting.
        try {
            location.replace('#__zapdude=' + encodeURIComponent(JSON.stringify({
                id: 'evt_' + String(Date.now()) + '_' + Math.floor(Math.random() * 1e6),
                payload: { kind: 'unread', count: count },
            })));
        } catch (_beaconError) {}
    }

    function sample() {
        try {
            var match = String(document.title || '').match(/\((\d+)\)/);
            var count = match ? parseInt(match[1], 10) : 0;
            if (!isFinite(count) || count < 0) {
                count = 0;
            }
            if (count > 999) {
                count = 999; // a runaway parse never paints "1000"
            }
            if (count !== last) {
                last = count;
                ship(count);
            }
        } catch (_sampleError) {
            // next beat
        }
    }

    function start() {
        sample();
        setInterval(sample, 3000);
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', start);
    } else {
        start();
    }
})();
