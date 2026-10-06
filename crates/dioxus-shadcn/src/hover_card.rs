use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{
  DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::hover_open::{HoverOpenOptions, use_hover_open};

pub const HOVER_CARD_CONTENT_BASE_CLASS: &str = "z-50 w-80 rounded-md border border-border bg-popover p-4 text-popover-foreground shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
pub const HOVER_CARD_HEADER_BASE_CLASS: &str = "grid gap-1";
pub const HOVER_CARD_TITLE_BASE_CLASS: &str = "font-medium leading-none text-foreground";
pub const HOVER_CARD_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";

pub fn hover_card_content_class(class: &str) -> String {
  merge_classes(classes([Some(HOVER_CARD_CONTENT_BASE_CLASS)]), class)
}

pub fn hover_card_header_class(class: &str) -> String {
  merge_classes(classes([Some(HOVER_CARD_HEADER_BASE_CLASS)]), class)
}

pub fn hover_card_title_class(class: &str) -> String {
  merge_classes(classes([Some(HOVER_CARD_TITLE_BASE_CLASS)]), class)
}

pub fn hover_card_description_class(class: &str) -> String {
  merge_classes(classes([Some(HOVER_CARD_DESCRIPTION_BASE_CLASS)]), class)
}

pub fn hover_card_side_attribute(side: OverlaySide) -> &'static str {
  match side {
    OverlaySide::Top => "top",
    OverlaySide::Right => "right",
    OverlaySide::Bottom => "bottom",
    OverlaySide::Left => "left",
    OverlaySide::Inline => "inline",
  }
}

pub fn hover_card_align_attribute(align: OverlayAlign) -> &'static str {
  match align {
    OverlayAlign::Start => "start",
    OverlayAlign::Center => "center",
    OverlayAlign::End => "end",
  }
}

#[derive(Clone, PartialEq)]
struct HoverCardContext {
  base_id: String,
  on_open_change: Option<EventHandler<bool>>,
}

impl HoverCardContext {
  fn trigger_id(&self) -> String {
    format!("{}-trigger", self.base_id)
  }
}

/// Opens the card after `open_delay_ms` of hover, or at once on keyboard
/// focus, and keeps it open while the pointer or focus is on the trigger or
/// the card. It closes `close_delay_ms` after the pointer leaves both, or when
/// focus leaves both. Trigger presses keep it open. Requests reach the app
/// through `on_open_change`; the delays are read when the root mounts.
#[component]
pub fn HoverCard(
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = 700)] open_delay_ms: u32,
  #[props(default = 300)] close_delay_ms: u32,
  children: Element,
) -> Element {
  let base_id = use_hover_open(
    on_open_change,
    HoverOpenOptions {
      open_delay_ms,
      close_delay_ms,
      close_on_press: false,
      describe_trigger: false,
    },
  );
  use_context_provider(|| HoverCardContext { base_id: base_id.clone(), on_open_change });

  rsx! {
    div {
      style: "display: contents",
      "data-dxui-hover-open": base_id,
      {children}
    }
  }
}

/// A link that opens the card. Use it inside `HoverCard`.
#[component]
pub fn HoverCardTrigger(
  href: String,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let id = try_use_context::<HoverCardContext>().map(|context| context.trigger_id());
  let is_part = id.is_some().then_some("");

  rsx! {
    a {
      href,
      id,
      class,
      "data-dxui-hover-trigger": is_part,
      ..attributes,
      {children}
    }
  }
}

/// Inside `HoverCard`, the content anchors to `HoverCardTrigger` and uses the
/// root's `on_open_change` for dismissal unless `anchor_id` or
/// `on_open_change` is set.
#[component]
pub fn HoverCardContent(
  #[props(default)] open: bool,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default)] align: OverlayAlign,
  #[props(default)] class: String,
  #[props(default)] anchor_id: Option<String>,
  #[props(default = 4)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let class = hover_card_content_class(&class);
  let context = try_use_context::<HoverCardContext>();
  let is_part = context.is_some().then_some("");
  let anchor_id = anchor_id.or_else(|| context.as_ref().map(HoverCardContext::trigger_id));
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
      class,
      hidden: !open,
      "data-align": hover_card_align_attribute(align),
      "data-side": hover_card_side_attribute(side),
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      "data-dxui-hover-content": is_part,
      {children}
    }
  }
}

#[component]
pub fn HoverCardHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = hover_card_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn HoverCardTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = hover_card_title_class(&class);

  rsx! {
    h3 {
      class,
      {children}
    }
  }
}

#[component]
pub fn HoverCardDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = hover_card_description_class(&class);

  rsx! {
    p {
      class,
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
      rsx! { HoverCardTrigger { href: "https://dioxuslabs.com", target: "_blank", rel: "noreferrer", "@dioxus" } }
    }
    let html = render(app);

    assert!(html.contains(r#"target="_blank""#));
    assert!(html.contains(r#"rel="noreferrer""#));
  }

  #[test]
  fn ssr_content_renders_no_role() {
    fn app() -> Element {
      rsx! { HoverCardContent { open: true, "Dioxus" } }
    }
    let html = render(app);

    assert!(html.contains(r#"data-state="open""#));
    assert!(!html.contains("role="));
  }

  #[test]
  fn hover_card_content_class_appends_user_class() {
    let actual = hover_card_content_class("w-96");

    assert_eq!(
      actual,
      "z-50 rounded-md border border-border bg-popover p-4 text-popover-foreground shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring w-96"
    );
    assert!(actual.ends_with("w-96"));
  }

  #[test]
  fn hover_card_attributes_match_placement_values() {
    assert_eq!(hover_card_side_attribute(OverlaySide::Top), "top");
    assert_eq!(hover_card_side_attribute(OverlaySide::Right), "right");
    assert_eq!(hover_card_side_attribute(OverlaySide::Bottom), "bottom");
    assert_eq!(hover_card_side_attribute(OverlaySide::Left), "left");
    assert_eq!(hover_card_align_attribute(OverlayAlign::Start), "start");
    assert_eq!(hover_card_align_attribute(OverlayAlign::Center), "center");
    assert_eq!(hover_card_align_attribute(OverlayAlign::End), "end");
  }

  #[test]
  fn hover_card_trigger_id_uses_the_root_id() {
    let context = HoverCardContext { base_id: "dxui-hover-2".to_string(), on_open_change: None };

    assert_eq!(context.trigger_id(), "dxui-hover-2-trigger");
  }

  #[test]
  fn hover_card_uses_popover_primitive_defaults() {
    let config = PopoverPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert_eq!(config.side, OverlaySide::Bottom);
    assert_eq!(config.align, OverlayAlign::Center);
    assert!(config.dismiss.escape_key);
    assert!(config.dismiss.outside_pointer);
    assert!(config.dismiss.focus_outside);
  }
}
