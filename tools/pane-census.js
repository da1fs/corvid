// Pane census: report the right pane's real state (used to debug the
// "chat disappears" reproduction). Read-only, defensive.
JSON.stringify((function () {
  function path(n) {
    var parts = [];
    var depth = 0;
    while (n && depth < 9 && n !== document.body) {
      var tag = String(n.tagName || "").toLowerCase();
      var t = n.getAttribute && n.getAttribute("data-testid");
      var c = String(n.classList && n.classList.length ? Array.prototype.slice.call(n.classList, 0, 2).join(".") : "").slice(0, 40);
      parts.push(tag + (t ? "(" + t + ")" : "") + (c ? "." + c : ""));
      n = n.parentElement;
      depth += 1;
    }
    return parts.slice(0, 7).join("<");
  }
  var inline = [];
  try {
    var all = document.querySelectorAll("div,aside,header,section,span,p,h1,h2,h3,button,a");
    for (var i = 0; i < all.length; i++) {
      var n = all[i];
      if (n.style && n.style.display === "none") {
        var isMain = n.id === "main";
        var inMain = !!n.closest("#main");
        var warn = !inMain && !isMain && n.closest("#pane-side");
        if (inline.length < 24) {
          inline.push({
            id: n.id || "",
            testid: (n.getAttribute && n.getAttribute("data-testid")) || "",
            cls: String((typeof n.className === "object" ? (n.className.baseVal || "") : (n.className || ""))).slice(0, 50),
            inMain: isMain ? "self" : inMain,
            paneSide: !!n.closest("#pane-side"),
            rects: n.getClientRects().length,
            path: path(n),
          });
        }
      }
    }
  } catch (e) { inline.push({ err: String(e) }); }
  var main = document.getElementById("main");
  var mainInfo = null;
  if (main) {
    var hiddenAncestors = [];
    var node = main.parentElement;
    var depth = 0;
    while (node && depth < 12 && node !== document.body) {
      var disp = "";
      try { disp = getComputedStyle(node).display; } catch (e) { disp = "err"; }
      var inline = node.style ? node.style.display : "";
      if (disp === "none") {
        hiddenAncestors.push({ disp: disp, inline: inline, cls: String(node.className || "").slice(0, 60), testid: node.getAttribute("data-testid") || "" });
      }
      node = node.parentElement;
      depth += 1;
    }
    mainInfo = {
      rects: main.getClientRects().length,
      children: main.children.length,
      header: !!main.querySelector('header'),
      composer: !!main.querySelector('div[contenteditable="true"]'),
      hiddenAncestors: hiddenAncestors,
      ownDisplay: (function(){ try { return getComputedStyle(main).display; } catch (e) { return "err"; } })(),
    };
  }
  var paneSide = document.getElementById("pane-side");
  var rowSelected = null;
  try {
    var sel = document.querySelector('#pane-side [data-testid^="list-item"][aria-selected="true"]');
    if (sel) rowSelected = (sel.getAttribute("data-testid") || "").slice(0, 40);
  } catch (e) {}
  return {
    url: location.href,
    title: (document.title || "").slice(0, 40),
    bridge: typeof window.__zapdudeLinkV1,
    paneSide: !!paneSide,
    rowSelected: rowSelected,
    introStyleInstalled: !!document.getElementById("zapdude-intro-card-style"),
    introCard: !!document.querySelector('[data-testid="intro_panel_v2_title_card"]'),
    header: !!document.querySelector("header"),
    main: mainInfo,
    inlineHiddenCount: inline.length,
    inlineHidden: inline,
  };
})())
