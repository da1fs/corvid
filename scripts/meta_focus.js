/**
 * Zapdude meta-surfaces focus guard ("only the chat").
 *
 * The facebook + instagram account webviews exist for ONE thing: the chat.
 * A login completed through facebook.com / instagram.com can land the user
 * on the full logged-in site (feed, suggestions...) instead of the chat
 * surface they asked for. This guard watches the top page and returns it:
 *
 *   facebook service  -> any logged-in facebook.com landing  -> messenger.com
 *   instagram service -> any logged-in instagram.com non-DM  -> /direct
 *
 * Anti-loop laws:
 *   1. Redirect ONLY on DOM proof of a logged-in page (the login wall
 *      renders no feed/navigation chrome; cookie-less guessing would send
 *      a mid-login user in circles).
 *   2. A shot counter in sessionStorage caps the relocation per in-app
 *      session: a surface that keeps bouncing back dead-ends the guard
 *      instead of fighting the user (they handle the login manually; the
 *      API bridge still reports pageInfo honestly).
 *   3. It never touches messenger.com / instagram.com/direct / in-progress
 *      login pages; SPA history mutations and plain reloads both re-arm it.
 */
(function () {
    'use strict';

    if (window !== window.top || window.__zapdudeFocusV1) {
        return;
    }
    window.__zapdudeFocusV1 = true;

    var CHAT_URLS = {
        'messenger.com': 'https://www.messenger.com/',
        'facebook.com': 'https://www.messenger.com/',
        'instagram.com': 'https://www.instagram.com/direct/',
        'x.com': 'https://x.com/messages',
        'twitter.com': 'https://x.com/messages',
        'tiktok.com': 'https://www.tiktok.com/messages',
    };
    var MAX_SHOTS = 3;

    function currentTarget() {
        try {
            var url = new URL(String(location.href || ''));
            for (var suffix in CHAT_URLS) {
                if (Object.prototype.hasOwnProperty.call(CHAT_URLS, suffix)) {
                    var host = String(url.hostname || '').toLowerCase();
                    if (host === suffix || host.slice(-(suffix.length + 1)) === '.' + suffix) {
                        return { suffix: suffix, chatUrl: CHAT_URLS[suffix], url: url };
                    }
                }
            }
        } catch (_parseError) {
            // keep null
        }
        return null;
    }

    function inProgressLogin(url) {
        var path = String(url.pathname || '').toLowerCase();
        if (path.indexOf('/login') === 0) {
            return true;
        }
        // X runs its login inside /i/flow/; TikTok uses /oauth + /passport;
        // Meta land on /accounts / /checkpoint. Every one is IN-FLIGHT so
        // the relocation never fights the person mid-login.
        if (/\/(i\/flow|accounts|checkpoint|oauth|passport)\//.test('/' + path.replace(/^\/?/, ''))) {
            return true;
        }
        return false;
    }

    // DOM proof of a logged-in surface: the feed container or the app
    // navigation chrome. Login walls render none of these.
    function loggedInChromeSeen() {
        var marks = [
            'div[role="feed"]',
            'div[role="navigation"]',
            'div#mount_0_0 div[role="main"]',
        ];
        for (var i = 0; i < marks.length; i++) {
            try {
                if (document.querySelector(marks[i])) {
                    return true;
                }
            } catch (_probeError) {
                // next marker
            }
        }
        return false;
    }

    function shotCount() {
        try {
            var shots = Number(window.sessionStorage.getItem('__zapdudeFocusShots') || '0');
            if (!window.isFinite(shots) || shots < 0) {
                return 0;
            }
            return shots;
        } catch (_storageError) {
            return 0;
        }
    }

    function bumpShots() {
        try {
            window.sessionStorage.setItem('__zapdudeFocusShots', String(shotCount() + 1));
        } catch (_storageError) {
            // best effort only
        }
    }

    function shouldReturn() {
        var found = currentTarget();
        if (!found) {
            return null;
        }
        var path = String(found.url.pathname || '').toLowerCase();
        if (found.suffix === 'facebook.com') {
            // Logged-in facebook landings (feed, home, posts) go back to
            // Messenger; the chat-of-service surface never runs here.
            if (!loggedInChromeSeen() || inProgressLogin(found.url)) {
                return null;
            }
            return found.chatUrl;
        }
        if (found.suffix === 'instagram.com') {
            // instagram.com ANYWHERE but the DM surface (and not mid-login)
            // with the logged-in chrome: back to the DMs.
            if (path.indexOf('/direct') === 0 || inProgressLogin(found.url)) {
                return null;
            }
            if (!loggedInChromeSeen()) {
                return null;
            }
            return found.chatUrl;
        }
        if (found.suffix === 'x.com') {
            // X: /messages is the product; a logged-in feed/home detour
            // floats back. The login flow pages finish untouched.
            if (path.indexOf('/messages') === 0 || inProgressLogin(found.url)) {
                return null;
            }
            if (!loggedInChromeSeen()) {
                return null;
            }
            return found.chatUrl;
        }
        if (found.suffix === 'tiktok.com') {
            if (path.indexOf('/messages') === 0 || inProgressLogin(found.url)) {
                return null;
            }
            if (!loggedInChromeSeen()) {
                return null;
            }
            return found.chatUrl;
        }
        return null;
    }

    function review() {
        try {
            var destination = shouldReturn();
            if (!destination || shotCount() >= MAX_SHOTS) {
                return;
            }
            bumpShots();
            window.location.replace(destination);
        } catch (_reviewError) {
            // The guard never fights the page.
        }
    }

    // SPA guard: Meta surfaces rewrite history constantly; re-arm on both
    // mutation APIs plus reloads, and keep a slow DOM beat as the safety
    // net (interval only when the document is idle-cheap).
    try {
        var push = history.pushState;
        if (push) {
            history.pushState = function () {
                var result = push.apply(this, arguments);
                review();
                return result;
            };
        }
        var replace = history.replaceState;
        if (replace) {
            history.replaceState = function () {
                var result = replace.apply(this, arguments);
                review();
                return result;
            };
        }
        window.addEventListener('load', review);
        document.addEventListener('DOMContentLoaded', review);
        setInterval(review, 1500);
    } catch (_setupError) {
        // The guard is best-effort; the chat page keeps working without it.
    }
})();
