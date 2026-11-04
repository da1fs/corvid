// Health check for the stamp-based promo hider.
JSON.stringify({
  sweeps: window.__zapdudePromoSweeps,
  rule: !!document.getElementById("zapdude-promo-rule"),
  introRule: !!document.getElementById("zapdude-intro-card-style"),
  stamps: document.querySelectorAll("[data-zapdude-promo]").length,
  stampList: (function () {
    var out = [];
    var els = document.querySelectorAll("[data-zapdude-promo]");
    for (var i = 0; i < els.length; i++) {
      out.push({
        tag: String(els[i].tagName).toLowerCase(),
        testid: els[i].getAttribute("data-testid") || "",
        cls: String(typeof els[i].className === "object" ? (els[i].className.baseVal || "") : (els[i].className || "")).slice(0, 40),
        text: String((els[i].textContent || "").slice(0, 34)),
      });
    }
    return out;
  })(),
  introCardMounted: !!document.querySelector('[data-testid="intro_panel_v2_title_card"]'),
  main: !!document.getElementById("main"),
})
