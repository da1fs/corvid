// Stamp lifecycle probe: (1) a promo node gets stamped by the next sweep;
// (2) a functional element mounted INSIDE that same node forces the next
// sweep to REMOVE the stamp (the anti "conversation born invisible"
// guarantee); (3) final cleanup state. The answer lands in
// window.__zapdudeStampProbe via timers between two evals.
(function () {
  "use strict";
  if (!window.__zapdudeHidePromos) {
    return "hider not installed";
  }
  if (window.__zapdudeStampProbe) {
    return "probe already ran";
  }
  window.__zapdudeStampProbe = { log: [], done: false };
  var t = Date.now();
  var node = document.createElement("div");
  node.id = "zapdude-stamp-probe-node";
  node.textContent = "Descarga la aplicación para empezar";
  node.style.position = "fixed";
  node.style.width = "120px";
  node.style.height = "30px";
  node.style.bottom = "-200px"; // out of the user's way, still rendered
  node.style.left = "-300px";
  document.body.appendChild(node);
  window.__zapdudeStampProbe.node = node;

  function entry(label, state) {
    window.__zapdudeStampProbe.log.push({
      t: Date.now() - t,
      at: label,
      stamped: node.hasAttribute("data-zapdude-promo"),
      visible: (function () { try { return node.getClientRects().length > 0; } catch (e) { return null; } })(),
      note: state || "",
    });
  }

  entry("before any sweep (node just added)", "text: Descarga la aplicación");

  // Sweep 1 arrives ~600ms after the mutation -> the node should be stamped.
  setTimeout(function () {
    entry("after sweep 1", "expect stamped=true");
    // Sweep 2 forcing: mount a functional descendant INSIDE the stamped
    // node (this is exactly what WhatsApp does when a conversation mounts
    // into a container). The next sweep MUST unstamp.
    var live = document.createElement("span");
    live.setAttribute("contenteditable", "true");
    live.textContent = "composer territory";
    node.appendChild(live);
    entry("functional element mounted inside the stamped node", "expect unstamp on sweep 2");
  }, 900);

  // Read the guarantee at ~1.8s (sweep beat is 600ms after the mutation).
  setTimeout(function () {
    entry("after sweep 2", "expect stamped=false again");
    window.__zapdudeStampProbe.done = true;
  }, 2000);

  // Cleanup at 2.6s: remove the node so the app is untouched afterward.
  setTimeout(function () {
    try {
      if (node.parentElement) { node.parentElement.removeChild(node); }
    } catch (_e) {}
  }, 2600);

  return "stamp probe running";
})()
