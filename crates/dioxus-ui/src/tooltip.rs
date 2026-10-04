use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  DismissBehavior, OverlayAlign, OverlaySide, TooltipPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};

static NEXT_TOOLTIP_ID: AtomicUsize = AtomicUsize::new(0);

pub const TOOLTIP_CONTENT_BASE_CLASS: &str =
  "z-50 overflow-hidden rounded-md bg-zinc-950 px-3 py-1.5 text-xs text-white shadow-md";

pub fn tooltip_content_class(class: &str) -> String {
  classes([Some(TOOLTIP_CONTENT_BASE_CLASS), Some(class)])
}

// Runs for the root's lifetime and finds the trigger and content on every
// event. Sends "open" or "close" only when the request changes the content's
// `data-state`, and keeps the trigger's `aria-describedby` on the content
// while it is open.
// Keep in sync with `TOOLTIP_SCRIPT` in the CLI `tooltip.rs` template.
pub(crate) const TOOLTIP_SCRIPT: &str = r#"
const [scopeId, delayMs] = await dioxus.recv();
const root = document.querySelector(`[data-dxui-tooltip="${scopeId}"]`);
if (!root) return;
// Covers the gap between the trigger and the content.
const graceMs = 100;
const trigger = () => root.querySelector("[data-dxui-tooltip-trigger]");
const content = () => root.querySelector('[role="tooltip"]');
const isOpen = () => content()?.dataset.state === "open";
const inside = (element, node) => node instanceof Node && !!element && element.contains(node);
let openTimer = null;
let closeTimer = null;
// Hover opens only when the pointer enters the trigger, so after a press it
// stays closed until the pointer leaves and returns. The press's own focus
// does not open it.
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
const onPointerOver = (event) => {
  if (event.pointerType === "touch") return;
  const [target, from] = [event.target, event.relatedTarget];
  if (inside(trigger(), target) && !inside(trigger(), from)) {
    clearTimeout(closeTimer);
    if (isOpen()) return;
    clearTimeout(openTimer);
    openTimer = setTimeout(() => request(true), delayMs);
  } else if (inside(content(), target) && !inside(content(), from)) {
    clearTimeout(closeTimer);
  }
};
const onPointerOut = (event) => {
  if (event.pointerType === "touch") return;
  const [target, to] = [event.target, event.relatedTarget];
  const fromTrigger = inside(trigger(), target) && !inside(trigger(), to);
  const fromContent = inside(content(), target) && !inside(content(), to);
  if (!fromTrigger && !fromContent) return;
  clearTimeout(openTimer);
  if (inside(trigger(), to) || inside(content(), to) || !isOpen()) return;
  clearTimeout(closeTimer);
  closeTimer = setTimeout(() => request(false), graceMs);
};
const onPointerDown = (event) => {
  if (!inside(trigger(), event.target)) return;
  pressing = true;
  request(false);
};
const onPointerUp = () => {
  pressing = false;
};
const onFocusIn = (event) => {
  if (inside(trigger(), event.target) && !pressing) request(true);
};
const onFocusOut = (event) => {
  if (inside(trigger(), event.target)) request(false);
};
const describe = () => {
  const button = trigger();
  const tooltip = content();
  if (!button) return;
  if (tooltip && tooltip.id && isOpen()) button.setAttribute("aria-describedby", tooltip.id);
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
"#;

#[derive(Clone, PartialEq)]
struct TooltipContext {
  base_id: String,
  on_open_change: Option<EventHandler<bool>>,
}

impl TooltipContext {
  fn trigger_id(&self) -> String {
    format!("{}-trigger", self.base_id)
  }

  fn content_id(&self) -> String {
    format!("{}-content", self.base_id)
  }
}

/// Opens the tooltip after `delay_ms` of hover, or at once on keyboard focus,
/// and keeps it open while the pointer is over the trigger or the content.
/// Pointer leave, blur, and trigger presses close it. Requests reach the app
/// through `on_open_change`; `delay_ms` is read when the root mounts.
#[component]
pub fn Tooltip(
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = 700)] delay_ms: u32,
  children: Element,
) -> Element {
  let base_id =
    use_hook(|| format!("dxui-tooltip-{}", NEXT_TOOLTIP_ID.fetch_add(1, Ordering::Relaxed)));
  use_context_provider(|| TooltipContext { base_id: base_id.clone(), on_open_change });
  let effect_scope_id = base_id.clone();

  use_effect(move || {
    let mut eval = document::eval(TOOLTIP_SCRIPT);
    // A send error means the page already finished the script; nothing to track.
    let _ = eval.send((effect_scope_id.as_str(), delay_ms));
    spawn(async move {
      while let Ok(message) = eval.recv::<String>().await {
        if let Some(handler) = on_open_change {
          handler.call(message == "open");
        }
      }
    });
  });

  rsx! {
    div {
      style: "display: contents",
      "data-dxui-tooltip": base_id,
      {children}
    }
  }
}

/// A button that describes itself with the content while the tooltip is open.
/// Use it inside `Tooltip`.
#[component]
pub fn TooltipTrigger(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let id = try_use_context::<TooltipContext>().map(|context| context.trigger_id());

  rsx! {
    button {
      r#type: "button",
      id,
      class,
      disabled,
      "data-dxui-tooltip-trigger": "",
      {children}
    }
  }
}

/// Inside `Tooltip`, the content anchors to `TooltipTrigger` and uses the
/// root's `on_open_change` for Escape unless `anchor_id` or `on_open_change`
/// is set.
#[component]
pub fn TooltipContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] anchor_id: Option<String>,
  #[props(default = OverlaySide::Top)] side: OverlaySide,
  #[props(default)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::tooltip_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let class = tooltip_content_class(&class);
  let context = try_use_context::<TooltipContext>();
  let id = context.as_ref().map(TooltipContext::content_id);
  let anchor_id = anchor_id.or_else(|| context.as_ref().map(TooltipContext::trigger_id));
  let on_open_change =
    on_open_change.or_else(|| context.as_ref().and_then(|context| context.on_open_change));
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    on_open_change,
  );

  rsx! {
    div {
      role: "tooltip",
      id,
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tooltip_content_class_appends_user_class() {
    let actual = tooltip_content_class("max-w-48");

    assert!(actual.contains(TOOLTIP_CONTENT_BASE_CLASS));
    assert!(actual.ends_with("max-w-48"));
  }

  #[test]
  fn tooltip_primitive_config_is_reexported() {
    let config = TooltipPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert_eq!(config.side, OverlaySide::Top);
  }

  #[test]
  fn tooltip_part_ids_share_the_root_id() {
    let context = TooltipContext { base_id: "dxui-tooltip-3".to_string(), on_open_change: None };

    assert_eq!(context.trigger_id(), "dxui-tooltip-3-trigger");
    assert_eq!(context.content_id(), "dxui-tooltip-3-content");
  }
}
