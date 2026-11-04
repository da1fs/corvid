// Pane click probe: reproduces the user's gesture (click a chat row) and
// records the right pane's state across the WhatsApp intro->chat React
// transition. First eval installs it and clicks; a second eval reads the
// recording (timers live in the page between evals).
(function () {
  "use strict";
  if (window.__zapdudePaneProbe) {
    return "probe already installed";
  }
  window.__zapdudePaneProbe = { t: 0, log: [], done: false };

  function enc(n) {
    // JSON-safety for className (SVG className is an object).
    return String(typeof n.className === "object" ? (n.className.baseVal || "") : (n.className || ""));
  }
  function pathOf(n) {
    var parts = [];
    var depth = 0;
    while (n && depth < 8 && n !== document.body) {
      var tag = String(n.tagName || "").toLowerCase();
      var t = n.getAttribute && n.getAttribute("data-testid");
      parts.push(tag + (t ? "(" + t + ")" : "") + (enc(n).slice(0, 26) ? "." + enc(n).slice(0, 26) : ""));
      n = n.parentElement;
      depth += 1;
    }
    return parts.slice(0, 6).join("<");
  }
  function snapshot() {
    var main = document.getElementById("main");
    var hidden = [];
    try {
      var all = document.querySelectorAll("body *");
      for (var i = 0; i < all.length; i++) {
        var n = all[i];
        if (n.style && n.style.display === "none") {
          var inMain = !!n.closest("#main");
          var inSide = !!n.closest("#pane-side");
          // Skip the known intro-card scenario to keep noise low; the card
          // is styled away by rule, its children inline-hidden by the sweep.
          var inCard = !!n.closest('[data-testid="intro_panel_v2_title_card"]');
          if (!inCard) {
            if (hidden.length < 14) {
              hidden.push({
                tag: String(n.tagName).toLowerCase(),
                id: n.id || "",
                testid: (n.getAttribute && n.getAttribute("data-testid")) || "",
                cls: enc(n).slice(0, 44),
                inMain: inMain,
                inSide: inSide,
                nearMain: main ? n.compareDocumentPosition(main) & Node.DOCUMENT_POSITION_CONTAINS : false,
                text: String((n.textContent || "").slice(0, 40)),
              });
            }
          }
        }
      }
    } catch (e) {
      hidden.push({ err: String(e) });
    }
    var item = {
      t: Date.now() - t0,
      mainExists: !!main,
      mainRects: main ? main.getClientRects().length : -1,
      mainDisplay: main ? (function(){ try { return getComputedStyle(main).display; } catch (e) { return "err"; } })() : "",
      mainHeader: main ? !!main.querySelector("header") : null,
      composer: !!document.querySelector('#main div[contenteditable="true"]'),
      hidden: hidden,
    };
    return item;
  }

  var t0 = Date.now();
  // t=0 BEFORE the click: the honest baseline (intro visible? card rule?).
  window.__zapdudePaneProbe.log.push(snapshot());

  var row = document.querySelector('#pane-side div[data-testid^="list-item"]');
  if (!row) {
    window.__zapdudePaneProbe.done = true;
    window.__zapdudePaneProbe.fatal = "no chat row found";
    return "no chat row";
  }
  var title = document.querySelector('#pane-side div[data-testid^="list-item"] span[title]');
  window.__zapdudePaneProbe.rowTitle = title ? title.getAttribute("title") : "(untitled)";
  row.click();

  var beats = [500, 900, 1300, 2000, 3000];
  beats.forEach(function (ms) {
    setTimeout(function () {
      window.__zapdudePaneProbe.log.push(snapshot());
      if (ms === beats[beats.length - 1]) {
        window.__zapdudePaneProbe.done = true;
      }
    }, ms);
  });
  return "probe clicked: " + window.__zapdudePaneProbe.rowTitle;
})()
