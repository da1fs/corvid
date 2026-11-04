// Faithful row click: full pointer sequence + selection read-back.
(function () {
  "use strict";
  var row = document.querySelector('#pane-side div[data-testid^="list-item"]');
  if (!row) { return "no row"; }
  var before = row.getAttribute("aria-selected");
  function fire(name, target) {
    try {
      var evt = new PointerEvent(name, { bubbles: true, cancelable: true, composed: true, pointerId: 1, pointerType: "mouse", isPrimary: true, buttons: name === "pointerup" ? 0 : 1, button: 0 });
      target.dispatchEvent(evt);
    } catch (e) {
      try {
        var me = new MouseEvent(name, { bubbles: true, cancelable: true, composed: true, button: 0, buttons: name === "pointerup" ? 0 : 1 });
        target.dispatchEvent(me);
      } catch (e2) {}
    }
  }
  var target = row.querySelector("div, span") || row;
  if (!window.PointerEvent) {
    try { target.click(); } catch (e) {}
  } else {
    fire("pointerdown", target);
    fire("mousedown", target);
    fire("pointerup", target);
    fire("mouseup", target);
    fire("click", target);
  }
  return JSON.stringify({
    clickedTitle: (row.querySelector("span[title]") || {}).title || title,
    before: before,
    after: (function () { var r = document.querySelector('#pane-side div[data-testid^="list-item"]'); return r ? r.getAttribute("aria-selected") : "none"; })(),
    main: !!document.getElementById("main"),
  });
})()
