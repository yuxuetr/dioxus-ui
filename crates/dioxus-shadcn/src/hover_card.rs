use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{
  DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::hover_open::{HoverOpenOptions, use_hover_open};
use crate::overlay_root::{OverlayRoot, use_overlay_root};
use crate::root_state::use_root_context;

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

/// What a `HoverCard` shares with its parts.
#[derive(Clone, Copy)]
struct HoverCardContext(OverlayRoot);

fn use_hover_card(part: &str) -> OverlayRoot {
  use_root_context::<HoverCardContext>(part, "HoverCard").0
}

/// The root of a hover card: it owns whether the card is open. It opens after
/// `open_delay_ms` of hover, or at once on keyboard focus, and stays open
/// while the pointer or focus is on the trigger or the card. It closes
/// `close_delay_ms` after the pointer leaves both, or when focus leaves both.
/// Trigger presses keep it open. Pass `open` to control it, or `default_open`
/// to start it; `on_open_change` hears every change either way. The delays
/// are read when the root mounts.
#[component]
pub fn HoverCard(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = 700)] open_delay_ms: u32,
  #[props(default = 300)] close_delay_ms: u32,
  children: Element,
) -> Element {
  let root = use_overlay_root("hover-card", open, default_open, on_open_change);
  let hover_id = use_hover_open(
    Some(root.set_open),
    HoverOpenOptions {
      open_delay_ms,
      close_delay_ms,
      close_on_press: false,
      describe_trigger: false,
    },
  );
  use_context_provider(|| HoverCardContext(root));

  rsx! {
    div {
      style: "display: contents",
      "data-dxui-hover-open": hover_id,
      {children}
    }
  }
}

/// A link that opens the card.
#[component]
pub fn HoverCardTrigger(
  href: String,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_hover_card("HoverCardTrigger");

  rsx! {
    a {
      href,
      id: root.trigger_id(),
      class,
      "data-dxui-hover-trigger": "",
      ..attributes,
      {children}
    }
  }
}

/// The card anchors to `HoverCardTrigger` and closes per `dismiss`.
#[component]
pub fn HoverCardContent(
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default)] align: OverlayAlign,
  #[props(default)] class: String,
  #[props(default = 4)] side_offset: i32,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let root = use_hover_card("HoverCardContent");
  let class = hover_card_content_class(&class);
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
      id: root.content_id(),
      class,
      hidden: !open,
      "data-align": hover_card_align_attribute(align),
      "data-side": hover_card_side_attribute(side),
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      "data-dxui-hover-content": "",
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
      rsx! {
        HoverCard {
          HoverCardTrigger { href: "https://dioxuslabs.com", target: "_blank", rel: "noreferrer", "@dioxus" }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"id="dxui-hover-card-0-trigger""#), "{html}");
    assert!(html.contains(r#"target="_blank""#));
    assert!(html.contains(r#"rel="noreferrer""#));
  }

  #[test]
  fn ssr_content_renders_no_role() {
    fn app() -> Element {
      rsx! {
        HoverCard { open: true,
          HoverCardContent { "Dioxus" }
        }
      }
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
  fn ssr_hover_card_part_outside_its_root_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        HoverCardContent { "Dioxus" }
      }
    }
    let html = render(app);

    assert!(html.contains("before"));
    assert!(!html.contains("Dioxus"), "{html}");
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
