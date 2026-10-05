use dioxus::prelude::*;
use dioxus_shadcn_core::classes;
pub use dioxus_shadcn_primitives::{
  DismissBehavior, OverlayAlign, OverlaySide, TooltipPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::hover_open::{HoverOpenOptions, use_hover_open};

pub const TOOLTIP_CONTENT_BASE_CLASS: &str = "z-50 overflow-hidden rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground shadow-md";

pub fn tooltip_content_class(class: &str) -> String {
  classes([Some(TOOLTIP_CONTENT_BASE_CLASS), Some(class)])
}

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
  let base_id = use_hover_open(
    on_open_change,
    HoverOpenOptions {
      open_delay_ms: delay_ms,
      // Covers the gap between the trigger and the content.
      close_delay_ms: 100,
      close_on_press: true,
      describe_trigger: true,
    },
  );
  use_context_provider(|| TooltipContext { base_id: base_id.clone(), on_open_change });

  rsx! {
    div {
      style: "display: contents",
      "data-dxui-hover-open": base_id,
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
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let id = try_use_context::<TooltipContext>().map(|context| context.trigger_id());
  let is_part = id.is_some().then_some("");

  rsx! {
    button {
      r#type: "button",
      id,
      class,
      disabled,
      "data-dxui-hover-trigger": is_part,
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
  let is_part = id.is_some().then_some("");
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
      "data-dxui-hover-content": is_part,
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
      rsx! { TooltipTrigger { "aria-label": "Copy link", "c" } }
    }
    let html = render(app);

    assert!(html.contains(r#"type="button""#));
    assert!(html.contains(r#"aria-label="Copy link""#));
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

  #[test]
  fn tooltip_part_ids_share_the_root_id() {
    let context = TooltipContext { base_id: "dxui-hover-3".to_string(), on_open_change: None };

    assert_eq!(context.trigger_id(), "dxui-hover-3-trigger");
    assert_eq!(context.content_id(), "dxui-hover-3-content");
  }
}
