use dioxus::prelude::*;

use super::element_id::next_element_id;
use super::script::{Script, component_script};

// Runs for the root's lifetime and finds the trigger and content on every
// event. Sends "open" or "close" only when the request changes the content's
// `data-state`. Parts inside a nested root belong to that root.
component_script!(hover_open_script = r#"
export async function run(dioxus) {
  const [scopeId, openDelayMs, closeDelayMs, closeOnPress, describes] = await dioxus.recv();
  const root = document.querySelector(`[data-dxui-hover-open="${scopeId}"]`);
  if (!root) return;
  const part = (name) =>
    Array.from(root.querySelectorAll(`[data-dxui-hover-${name}]`)).find(
      (element) => element.closest("[data-dxui-hover-open]") === root,
    );
  const trigger = () => part("trigger");
  const content = () => part("content");
  const isOpen = () => content()?.dataset.state === "open";
  const inside = (element, node) => node instanceof Node && !!element && element.contains(node);
  const insideParts = (node) => inside(trigger(), node) || inside(content(), node);
  let openTimer = null;
  let closeTimer = null;
  let hovered = false;
  // A press's own focus does not open. With `closeOnPress`, hover opens only
  // when the pointer enters the trigger, so after a press it stays closed until
  // the pointer leaves and returns.
  let pressing = false;
  const clearTimers = () => {
    clearTimeout(openTimer);
    clearTimeout(closeTimer);
    openTimer = closeTimer = null;
  };
  const request = (open) => {
    clearTimers();
    if (open !== isOpen()) dioxus.send(open ? "open" : "close");
  };
  // Moving between the trigger and the content, across the gap within the close
  // delay, counts as staying.
  const onPointerOver = (event) => {
    if (event.pointerType === "touch") return;
    if (!insideParts(event.target) || insideParts(event.relatedTarget)) return;
    hovered = true;
    clearTimeout(closeTimer);
    if (isOpen() || !inside(trigger(), event.target)) return;
    clearTimeout(openTimer);
    openTimer = setTimeout(() => request(true), openDelayMs);
  };
  const onPointerOut = (event) => {
    if (event.pointerType === "touch") return;
    if (!insideParts(event.target) || insideParts(event.relatedTarget)) return;
    hovered = false;
    clearTimeout(openTimer);
    if (!isOpen()) return;
    clearTimeout(closeTimer);
    closeTimer = setTimeout(() => request(false), closeDelayMs);
  };
  const onPointerDown = (event) => {
    if (!inside(trigger(), event.target)) return;
    pressing = true;
    if (closeOnPress) request(false);
  };
  const onPointerUp = () => {
    pressing = false;
  };
  const onFocusIn = (event) => {
    if (inside(trigger(), event.target) && !pressing) request(true);
  };
  // Focus moving between the parts keeps it open. Focus leaving while the
  // pointer rests on the parts, such as a press on the content's text, leaves
  // closing to the pointer.
  const onFocusOut = (event) => {
    if (insideParts(event.target) && !insideParts(event.relatedTarget) && !hovered) request(false);
  };
  const describe = () => {
    const button = trigger();
    const described = content();
    if (!describes || !button) return;
    if (described && described.id && isOpen()) button.setAttribute("aria-describedby", described.id);
    else button.removeAttribute("aria-describedby");
  };
  let finish;
  const ended = new Promise((resolve) => {
    finish = resolve;
  });
  const observer = new MutationObserver(() => {
    if (!root.isConnected) return finish();
    describe();
  });
  describe();
  observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["data-state"] });
  root.addEventListener("pointerover", onPointerOver);
  root.addEventListener("pointerout", onPointerOut);
  root.addEventListener("pointerdown", onPointerDown);
  root.addEventListener("focusin", onFocusIn);
  root.addEventListener("focusout", onFocusOut);
  document.addEventListener("pointerup", onPointerUp, true);
  document.addEventListener("pointercancel", onPointerUp, true);
  await ended;
  clearTimers();
  observer.disconnect();
  document.removeEventListener("pointerup", onPointerUp, true);
  document.removeEventListener("pointercancel", onPointerUp, true);
}
"#);

/// How a hover-open root times and links its parts.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct HoverOpenOptions {
  pub(crate) open_delay_ms: u32,
  pub(crate) close_delay_ms: u32,
  /// A press on the trigger closes the content, and it stays closed while the
  /// pointer rests on the trigger.
  pub(crate) close_on_press: bool,
  /// The trigger's `aria-describedby` names the content while it is open.
  pub(crate) describe_trigger: bool,
}

/// Runs the hover-open script for the component's lifetime. Hover and focus
/// requests call `on_open_change`; `options` are read when the root mounts.
///
/// Returns the value for the root's `data-dxui-hover-open` attribute. The
/// trigger carries `data-dxui-hover-trigger` and the content
/// `data-dxui-hover-content`.
pub(crate) fn use_hover_open(
  on_open_change: Option<EventHandler<bool>>,
  options: HoverOpenOptions,
) -> String {
  let scope_id = use_hook(|| format!("dxui-hover-{}", next_element_id()));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let mut script = hover_open_script::start();
    // A send error means the page already finished the script; nothing to track.
    let _ = script.send((
      effect_scope_id.as_str(),
      options.open_delay_ms,
      options.close_delay_ms,
      options.close_on_press,
      options.describe_trigger,
    ));
    spawn(async move {
      while let Ok(message) = script.recv::<String>().await {
        if let Some(handler) = on_open_change {
          handler.call(message == "open");
        }
      }
    });
  });

  scope_id
}
