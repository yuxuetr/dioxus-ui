use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{OverlayAlign, OverlaySide, PopoverPrimitiveConfig};

pub const HOVER_CARD_CONTENT_BASE_CLASS: &str = "z-50 w-80 rounded-md border border-zinc-200 bg-white p-4 text-zinc-950 shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";
pub const HOVER_CARD_HEADER_BASE_CLASS: &str = "grid gap-1";
pub const HOVER_CARD_TITLE_BASE_CLASS: &str = "font-medium leading-none text-zinc-950";
pub const HOVER_CARD_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";

pub fn hover_card_content_class(class: &str) -> String {
  classes([Some(HOVER_CARD_CONTENT_BASE_CLASS), Some(class)])
}

pub fn hover_card_header_class(class: &str) -> String {
  classes([Some(HOVER_CARD_HEADER_BASE_CLASS), Some(class)])
}

pub fn hover_card_title_class(class: &str) -> String {
  classes([Some(HOVER_CARD_TITLE_BASE_CLASS), Some(class)])
}

pub fn hover_card_description_class(class: &str) -> String {
  classes([Some(HOVER_CARD_DESCRIPTION_BASE_CLASS), Some(class)])
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

#[component]
pub fn HoverCardContent(
  #[props(default)] open: bool,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default)] align: OverlayAlign,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = hover_card_content_class(&class);

  rsx! {
    div {
      role: "dialog",
      class,
      hidden: !open,
      "data-align": hover_card_align_attribute(align),
      "data-side": hover_card_side_attribute(side),
      "data-state": if open { "open" } else { "closed" },
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

  #[test]
  fn hover_card_content_class_appends_user_class() {
    let actual = hover_card_content_class("w-96");

    assert!(actual.contains(HOVER_CARD_CONTENT_BASE_CLASS));
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
