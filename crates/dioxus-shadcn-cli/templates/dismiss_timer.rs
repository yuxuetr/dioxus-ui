use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

static NEXT_DISMISS_TIMER_ID: AtomicUsize = AtomicUsize::new(0);

// Counts down only while the pointer is outside the toast and focus is not
// inside it, and reports "timeout" once the remaining time runs out. Exits
// quietly when the toast is hidden or removed first.
pub(crate) const DISMISS_TIMER_SCRIPT: &str = r#"
const [scopeId, duration] = await dioxus.recv();
const toast = document.querySelector(`[data-dxui-dismiss-timer="${scopeId}"]`);
if (!toast) return;
await new Promise((resolve) => requestAnimationFrame(resolve));
if (!toast.isConnected || toast.hidden) return;
let remaining = duration;
let startedAt = 0;
let timer = null;
let hovered = false;
let focused = false;
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const resume = () => {
  if (timer !== null || hovered || focused) return;
  startedAt = performance.now();
  timer = setTimeout(() => finish("timeout"), remaining);
};
const pause = () => {
  if (timer === null) return;
  clearTimeout(timer);
  timer = null;
  remaining -= performance.now() - startedAt;
};
const onPointerEnter = () => {
  hovered = true;
  pause();
};
const onPointerLeave = () => {
  hovered = false;
  resume();
};
const onFocusIn = () => {
  focused = true;
  pause();
};
const onFocusOut = (event) => {
  if (toast.contains(event.relatedTarget)) return;
  focused = false;
  resume();
};
const observer = new MutationObserver(() => {
  if (!toast.isConnected || toast.hidden) finish("closed");
});
observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["hidden"] });
toast.addEventListener("pointerenter", onPointerEnter);
toast.addEventListener("pointerleave", onPointerLeave);
toast.addEventListener("focusin", onFocusIn);
toast.addEventListener("focusout", onFocusOut);
toast.dataset.timer = "running";
resume();
const reason = await ended;
clearTimeout(timer);
observer.disconnect();
toast.removeEventListener("pointerenter", onPointerEnter);
toast.removeEventListener("pointerleave", onPointerLeave);
toast.removeEventListener("focusin", onFocusIn);
toast.removeEventListener("focusout", onFocusOut);
delete toast.dataset.timer;
if (reason === "timeout") dioxus.send("timeout");
"#;

/// Starts the dismiss countdown each time `open` turns true with a non-zero
/// `duration_ms`, and calls `on_dismiss(timeout_reason)` when it runs out. The
/// duration is read when the toast opens.
///
/// Returns the value for the toast's `data-dxui-dismiss-timer` attribute.
pub(crate) fn use_dismiss_timer<R: Clone + 'static>(
  open: bool,
  duration_ms: u64,
  on_dismiss: Option<EventHandler<R>>,
  timeout_reason: R,
) -> String {
  let scope_id =
    use_hook(|| format!("dxui-timer-{}", NEXT_DISMISS_TIMER_ID.fetch_add(1, Ordering::Relaxed)));
  let was_open = use_hook(|| Rc::new(Cell::new(false)));
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive(
    (&open, &duration_ms, &on_dismiss),
    move |(open, duration_ms, on_dismiss)| {
      if open == was_open.replace(open) || !open || duration_ms == 0 {
        return;
      }
      let timeout_reason = timeout_reason.clone();
      let mut eval = document::eval(DISMISS_TIMER_SCRIPT);
      // A send error means the page already finished the script; nothing to time.
      let _ = eval.send((effect_scope_id.as_str(), duration_ms));
      spawn(async move {
        let timed_out = matches!(eval.recv::<String>().await, Ok(message) if message == "timeout");
        if let Some(handler) = on_dismiss.filter(|_| timed_out) {
          handler.call(timeout_reason);
        }
      });
    },
  ));

  scope_id
}
