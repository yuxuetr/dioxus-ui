use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

static NEXT_FOCUS_SCOPE_ID: AtomicUsize = AtomicUsize::new(0);

// Runs until the scope element is hidden or removed, so focus is restored both
// when `open` turns false and when the app stops rendering the content.
// Keep in sync with `MODAL_FOCUS_SCOPE_SCRIPT` in the CLI `utils.rs` template.
pub(crate) const MODAL_FOCUS_SCOPE_SCRIPT: &str = r#"
const scope = document.querySelector('[data-dxui-focus-scope="__SCOPE_ID__"]');
if (!scope) return;
const previous = document.activeElement;
const selector = 'a[href], button:not([disabled]), input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"]), [contenteditable="true"]';
const focusables = () => Array.from(scope.querySelectorAll(selector)).filter((el) => el.getClientRects().length > 0);
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
scope.addEventListener("keydown", onKeyDown);
(focusables()[0] ?? scope).focus();
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
if (previous instanceof HTMLElement && previous.isConnected) previous.focus();
"#;

pub(crate) fn modal_focus_scope_script(scope_id: &str) -> String {
  MODAL_FOCUS_SCOPE_SCRIPT.replace("__SCOPE_ID__", scope_id)
}

/// Starts the modal focus scope each time `open` turns true.
///
/// Returns the value for the content's `data-dxui-focus-scope` attribute.
pub(crate) fn use_modal_focus_scope(open: bool) -> String {
  let scope_id =
    use_hook(|| format!("dxui-focus-{}", NEXT_FOCUS_SCOPE_ID.fetch_add(1, Ordering::Relaxed)));
  let script = modal_focus_scope_script(&scope_id);

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
    let script = modal_focus_scope_script("dxui-focus-7");

    assert!(script.contains(r#"[data-dxui-focus-scope="dxui-focus-7"]"#));
    assert!(!script.contains("__SCOPE_ID__"));
  }
}
