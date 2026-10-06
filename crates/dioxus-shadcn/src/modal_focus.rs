use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

static NEXT_FOCUS_SCOPE_ID: AtomicUsize = AtomicUsize::new(0);

// Runs until the scope element is hidden or removed, so focus is restored both
// when `open` turns false and when the app stops rendering the content. A
// modal also locks page scroll (RFC 0068); nested modals share one lock,
// counted on the root element, and the last to close restores it.
// Keep in sync with `MODAL_FOCUS_SCOPE_SCRIPT` in the CLI `modal_focus.rs` template.
pub(crate) const MODAL_FOCUS_SCOPE_SCRIPT: &str = r#"
const scope = document.querySelector('[data-dxui-focus-scope="__SCOPE_ID__"]');
if (!scope) return;
const locksScroll = __LOCK_SCROLL__;
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
await new Promise((resolve) => requestAnimationFrame(resolve));
if (!scope.isConnected || scope.hidden) return;
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
if (locksScroll) unlockScroll();
// Focus still inside the hidden scope, or already blurred to the body, goes
// back; focus anywhere else was moved on purpose, such as by an outside click,
// and stays there.
const active = document.activeElement;
const focusLeft = active !== null && active !== document.body && !scope.contains(active);
if (!focusLeft && previous instanceof HTMLElement && previous.isConnected) previous.focus();
"#;

pub(crate) fn modal_focus_scope_script(scope_id: &str, lock_scroll: bool) -> String {
  MODAL_FOCUS_SCOPE_SCRIPT
    .replace("__SCOPE_ID__", scope_id)
    .replace("__LOCK_SCROLL__", if lock_scroll { "true" } else { "false" })
}

/// Starts the modal focus scope each time `open` turns true, locking page
/// scroll while it is open when `lock_scroll` is set.
///
/// Returns the value for the content's `data-dxui-focus-scope` attribute.
pub(crate) fn use_modal_focus_scope(open: bool, lock_scroll: bool) -> String {
  let scope_id =
    use_hook(|| format!("dxui-focus-{}", NEXT_FOCUS_SCOPE_ID.fetch_add(1, Ordering::Relaxed)));
  let script = modal_focus_scope_script(&scope_id, lock_scroll);

  use_effect(use_reactive((&open,), move |(open,)| {
    if open {
      document::eval(&script);
    }
  }));

  scope_id
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn focus_scope_script_targets_the_scope_id() {
    let script = modal_focus_scope_script("dxui-focus-7", true);

    assert!(script.contains(r#"[data-dxui-focus-scope="dxui-focus-7"]"#));
    assert!(script.contains("const locksScroll = true;"));
    assert!(!script.contains("__SCOPE_ID__"));
    assert!(modal_focus_scope_script("dxui-focus-7", false).contains("const locksScroll = false;"));
  }
}
