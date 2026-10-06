use super::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use super::hover_open::{HoverOpenOptions, use_hover_open};
pub use super::overlay::{DismissBehavior, OverlayAlign, OverlaySide, TooltipPrimitiveConfig};
use super::overlay_root::{OverlayRoot, use_overlay_root};
use super::root_state::use_root_context;
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

pub const TOOLTIP_CONTENT_BASE_CLASS: &str = "z-50 overflow-hidden rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground shadow-md";

pub fn tooltip_content_class(class: &str) -> String {
  merge_classes(classes([Some(TOOLTIP_CONTENT_BASE_CLASS)]), class)
}

/// What a `Tooltip` shares with its parts.
#[derive(Clone, Copy)]
struct TooltipContext(OverlayRoot);

fn use_tooltip(part: &str) -> OverlayRoot {
  use_root_context::<TooltipContext>(part, "Tooltip").0
}

/// The root of a tooltip: it owns whether it is open. It opens after
/// `delay_ms` of hover, or at once on keyboard focus, and stays open while the
/// pointer is over the trigger or the content. Pointer leave, blur, and
/// trigger presses close it. Pass `open` to control it, or `default_open` to
/// start it; `on_open_change` hears every change either way. `delay_ms` is
/// read when the root mounts.
#[component]
pub fn Tooltip(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = 700)] delay_ms: u32,
  children: Element,
) -> Element {
  let root = use_overlay_root("tooltip", open, default_open, on_open_change);
  let hover_id = use_hover_open(
    Some(root.set_open),
    HoverOpenOptions {
      open_delay_ms: delay_ms,
      // Covers the gap between the trigger and the content.
      close_delay_ms: 100,
      close_on_press: true,
      describe_trigger: true,
    },
  );
  use_context_provider(|| TooltipContext(root));

  rsx! {
    div {
      style: "display: contents",
      "data-dxui-hover-open": hover_id,
      {children}
    }
  }
}

/// A button that describes itself with the content while the tooltip is open.
#[component]
pub fn TooltipTrigger(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_tooltip("TooltipTrigger");

  rsx! {
    button {
      r#type: "button",
      id: root.trigger_id(),
      class,
      disabled,
      "data-dxui-hover-trigger": "",
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
      },
      ..attributes,
      {children}
    }
  }
}

/// The content anchors to `TooltipTrigger` and closes on Escape per
/// `dismiss`.
#[component]
pub fn TooltipContent(
  #[props(default)] class: String,
  #[props(default = OverlaySide::Top)] side: OverlaySide,
  #[props(default)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default = DismissBehavior::tooltip_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let root = use_tooltip("TooltipContent");
  let class = tooltip_content_class(&class);
  let open = root.is_open();
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement {
      anchor_id: Some(root.trigger_id()),
      anchor_point: None,
      side,
      align,
      side_offset,
    },
    dismiss,
    Some(root.set_open),
  );

  rsx! {
    div {
      role: "tooltip",
      id: root.content_id(),
      "data-dxui-hover-content": "",
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      {children}
    }
  }
}
