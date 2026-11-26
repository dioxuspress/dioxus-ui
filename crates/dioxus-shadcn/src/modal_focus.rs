use std::cell::Cell;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::element_id::next_element_id;
use crate::script::{Script, component_script};

// Runs until the scope element is hidden or removed, so focus is restored both
// when `open` turns false and when the app stops rendering the content. A
// modal also locks page scroll (RFC 0068); nested modals share one lock,
// counted on the root element, and the last to close restores it. When
// `listenEscape` is set, a document-level keydown listener is attached
// immediately (before the focus-settle animation frame) so Escape closes the
// topmost modal even right after open or when focus has blurred outside.
// Keep in sync with `modal_focus_scope_script` in the CLI `modal_focus.rs` template.
component_script!(
  modal_focus_scope_script = r#"
export async function run(dioxus) {
  const [scopeId, locksScroll, listenEscape] = await dioxus.recv();
  const scope = document.querySelector(`[data-dxui-focus-scope="${scopeId}"]`);
  if (!scope) return;
  const root = document.documentElement;
  const lockScroll = () => {
    const count = Number(root.dataset.dxuiScrollLocks || 0);
    if (count === 0) {
      // Padding keeps the content from shifting when the scrollbar goes away.
      const scrollbar = window.innerWidth - root.clientWidth;
      root.dataset.dxuiScrollRestore = JSON.stringify([root.style.overflow, root.style.paddingRight]);
      if (scrollbar > 0) root.style.paddingRight = `${parseFloat(getComputedStyle(root).paddingRight) + scrollbar}px`;
      root.style.overflow = "hidden";
    }
    root.dataset.dxuiScrollLocks = String(count + 1);
  };
  const unlockScroll = () => {
    const count = Number(root.dataset.dxuiScrollLocks || 1) - 1;
    if (count > 0) {
      root.dataset.dxuiScrollLocks = String(count);
      return;
    }
    const [overflow, paddingRight] = JSON.parse(root.dataset.dxuiScrollRestore || '["", ""]');
    root.style.overflow = overflow;
    root.style.paddingRight = paddingRight;
    delete root.dataset.dxuiScrollLocks;
    delete root.dataset.dxuiScrollRestore;
  };
  const previous = document.activeElement;
  const selector = 'a[href], button:not([disabled]), input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"]), [contenteditable="true"]';
  // Roving grids keep inactive items at tabindex -1; they are not Tab stops.
  const focusables = () =>
    Array.from(scope.querySelectorAll(selector)).filter((el) => el.tabIndex >= 0 && el.getClientRects().length > 0);
  const onDocumentKeyDown = (event) => {
    if (!listenEscape || event.key !== "Escape" || event.defaultPrevented) return;
    if (!scope.isConnected || scope.hidden) return;
    dioxus.send("escape");
  };
  const onKeyDown = (event) => {
    if (event.key !== "Tab") return;
    const items = focusables();
    if (items.length === 0) {
      event.preventDefault();
      scope.focus();
      return;
    }
    const first = items[0];
    const last = items[items.length - 1];
    const active = document.activeElement;
    if (event.shiftKey && (active === first || active === scope)) {
      event.preventDefault();
      last.focus();
    } else if (!event.shiftKey && active === last) {
      event.preventDefault();
      first.focus();
    }
  };
  if (listenEscape) document.addEventListener("keydown", onDocumentKeyDown);
  await new Promise((resolve) => requestAnimationFrame(resolve));
  if (!scope.isConnected || scope.hidden) {
    if (listenEscape) document.removeEventListener("keydown", onDocumentKeyDown);
    return;
  }
  if (locksScroll) lockScroll();
  scope.addEventListener("keydown", onKeyDown);
  (scope.querySelector("[data-dxui-autofocus]") ?? focusables()[0] ?? scope).focus();
  await new Promise((resolve) => {
    const observer = new MutationObserver(() => {
      if (!scope.isConnected || scope.hidden) {
        observer.disconnect();
        resolve();
      }
    });
    observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["hidden"] });
  });
  scope.removeEventListener("keydown", onKeyDown);
  if (listenEscape) document.removeEventListener("keydown", onDocumentKeyDown);
  if (locksScroll) unlockScroll();
  // Focus still inside the hidden scope, or already blurred to the body, goes
  // back; focus anywhere else was moved on purpose, such as by an outside click,
  // and stays there.
  const active = document.activeElement;
  const focusLeft = active !== null && active !== document.body && !scope.contains(active);
  if (!focusLeft && previous instanceof HTMLElement && previous.isConnected) previous.focus();
}
"#
);

/// Starts the modal focus scope each time `open` turns true, locking page
/// scroll while it is open when `lock_scroll` is set.
///
/// Returns the value for the content's `data-dxui-focus-scope` attribute.
#[allow(dead_code)]
pub(crate) fn use_modal_focus_scope(open: bool, lock_scroll: bool) -> String {
  use_modal_focus_scope_with_escape(open, lock_scroll, None)
}

/// Starts the modal focus scope each time `open` turns true, optionally
/// listening for document-level `Escape` key presses via `on_escape` so
/// dismissal works even before the initial animation-frame focus settle or
/// after focus leaves the dialog content.
///
/// Returns the value for the content's `data-dxui-focus-scope` attribute.
pub(crate) fn use_modal_focus_scope_with_escape(
  open: bool,
  lock_scroll: bool,
  on_escape: Option<Callback<()>>,
) -> String {
  let scope_id = use_hook(|| format!("dxui-focus-{}", next_element_id()));
  let was_open = use_hook(|| Rc::new(Cell::new(false)));
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive((&open, &on_escape), move |(open, on_escape)| {
    if open == was_open.replace(open) || !open {
      return;
    }
    let listen_escape = on_escape.is_some();
    let script = modal_focus_scope_script::start();
    // A send error means the page already finished the script; nothing to scope.
    let _ = script.send((effect_scope_id.as_str(), lock_scroll, listen_escape));
    if let Some(handler) = on_escape {
      spawn(async move {
        while let Ok(message) = script.recv::<String>().await {
          if message == "escape" {
            handler.call(());
          }
        }
      });
    }
  }));

  scope_id
}
