use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{
  DismissBehavior, OverlayAlign, OverlaySide, TooltipPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::hover_open::{HoverOpenOptions, use_hover_open};
use crate::overlay_root::{OverlayRoot, use_overlay_root};
use crate::root_state::use_root_context;

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

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_trigger_renders_passed_attributes() {
    fn app() -> Element {
      rsx! {
        Tooltip {
          TooltipTrigger { "aria-label": "Copy link", "c" }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"type="button" id="dxui-tooltip-0-trigger""#), "{html}");
    assert!(html.contains(r#"aria-label="Copy link""#));
  }

  #[test]
  fn ssr_tooltip_content_follows_the_root() {
    fn app() -> Element {
      rsx! {
        Tooltip { default_open: true,
          TooltipTrigger { "c" }
          TooltipContent { "Copy link" }
        }
        Tooltip {
          TooltipTrigger { "d" }
          TooltipContent { "Delete" }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"role="tooltip" id="dxui-tooltip-0-content""#), "{html}");
    assert_eq!(html.matches(r#"data-state="open""#).count(), 1, "{html}");
    assert_eq!(html.matches(r#"data-state="closed""#).count(), 1, "{html}");
  }

  #[test]
  fn ssr_tooltip_part_outside_its_root_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        TooltipTrigger { "c" }
      }
    }
    let html = render(app);

    assert!(html.contains("before"));
    assert!(!html.contains("<button"), "{html}");
  }

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
}
