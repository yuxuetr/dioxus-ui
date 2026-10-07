//! Message Scroller: controlled transcript viewport parts and pure scroll intent
//! helpers for chat, logs, and message feeds.
use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{
  MessageScrollerEvent, MessageScrollerIntent, MessageScrollerMetrics,
  message_scroller_distance_to_bottom, message_scroller_is_at_bottom, message_scroller_next_intent,
  message_scroller_should_follow, message_scroller_show_unread_marker,
};

use crate::default_attribute::default_attribute;
use crate::density::{density_control_class, use_density, with_density};

const MESSAGE_SCROLLER_BASE_CLASS: &str = "relative flex min-h-0 w-full flex-col overflow-hidden";
const MESSAGE_SCROLLER_VIEWPORT_BASE_CLASS: &str = "min-h-0 flex-1 overflow-y-auto overscroll-contain focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring";
const MESSAGE_SCROLLER_CONTENT_BASE_CLASS: &str = "flex min-h-full flex-col gap-4";
const MESSAGE_SCROLLER_BOTTOM_ANCHOR_BASE_CLASS: &str = "h-px w-full shrink-0 scroll-mb-4";
const MESSAGE_SCROLLER_UNREAD_MARKER_BASE_CLASS: &str =
  "pointer-events-none absolute inset-x-0 bottom-4 z-10 justify-center";
const MESSAGE_SCROLLER_JUMP_BUTTON_BASE_CLASS: &str = "h-9 items-center justify-center rounded-md border border-border bg-background px-3 text-sm font-medium text-foreground shadow-sm transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

const fn message_scroller_intent_attribute(intent: MessageScrollerIntent) -> &'static str {
  match intent {
    MessageScrollerIntent::Follow => "follow",
    MessageScrollerIntent::Hold => "hold",
    MessageScrollerIntent::JumpToLatest => "jump-to-latest",
  }
}

const fn message_scroller_is_following_intent(intent: MessageScrollerIntent) -> bool {
  matches!(intent, MessageScrollerIntent::Follow | MessageScrollerIntent::JumpToLatest)
}

/// Classes for the outer frame, with `class` merged over them.
pub fn message_scroller_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_SCROLLER_BASE_CLASS)]), class)
}

/// Classes for the scrolling viewport and its focus ring, with `class` merged over them.
pub fn message_scroller_viewport_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_SCROLLER_VIEWPORT_BASE_CLASS)]), class)
}

/// Classes for the column that holds the messages, with `class` merged over them.
pub fn message_scroller_content_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_SCROLLER_CONTENT_BASE_CLASS)]), class)
}

/// Classes for the one-pixel anchor at the end of the content, with `class` merged over them.
pub fn message_scroller_bottom_anchor_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_SCROLLER_BOTTOM_ANCHOR_BASE_CLASS)]), class)
}

/// Classes for the unread marker over the bottom edge: hidden unless `visible`, then
/// `class` merged over them.
pub fn message_scroller_unread_marker_class(visible: bool, class: &str) -> String {
  merge_classes(
    classes([
      Some(MESSAGE_SCROLLER_UNREAD_MARKER_BASE_CLASS),
      Some(if visible { "flex" } else { "hidden" }),
    ]),
    class,
  )
}

/// Classes for the jump-to-latest button: hidden unless `visible`, then `class` merged
/// over them.
pub fn message_scroller_jump_button_class(visible: bool, class: &str) -> String {
  merge_classes(
    classes([
      Some(MESSAGE_SCROLLER_JUMP_BUTTON_BASE_CLASS),
      Some(if visible { "inline-flex" } else { "hidden" }),
    ]),
    class,
  )
}

#[component]
pub fn MessageScroller(
  #[props(default)] intent: MessageScrollerIntent,
  #[props(default)] has_unread: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = message_scroller_class(&class);
  let following = message_scroller_is_following_intent(intent).to_string();
  let unread = has_unread.to_string();

  rsx! {
    div {
      class,
      "data-intent": message_scroller_intent_attribute(intent),
      "data-following": following,
      "data-unread": unread,
      {children}
    }
  }
}

#[component]
pub fn MessageScrollerViewport(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = message_scroller_viewport_class(&class);
  let tabindex = default_attribute(&attributes, "tabindex", "0");

  rsx! {
    div {
      class,
      tabindex,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn MessageScrollerContent(#[props(default)] class: String, children: Element) -> Element {
  let class = message_scroller_content_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn MessageScrollerBottomAnchor(#[props(default)] class: String) -> Element {
  let class = message_scroller_bottom_anchor_class(&class);

  rsx! {
    div {
      class,
      "aria-hidden": "true",
    }
  }
}

#[component]
pub fn MessageScrollerUnreadMarker(
  #[props(default)] visible: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = message_scroller_unread_marker_class(visible, &class);

  rsx! {
    div {
      class,
      "data-visible": visible.to_string(),
      {children}
    }
  }
}

#[component]
pub fn MessageScrollerJumpButton(
  #[props(default)] visible: bool,
  #[props(default)] class: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = message_scroller_jump_button_class(
    visible,
    &with_density(density_control_class(use_density()), &class),
  );

  rsx! {
    button {
      class,
      r#type: "button",
      "data-visible": visible.to_string(),
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ssr_viewport_is_focusable_and_takes_attributes() {
    fn app() -> Element {
      rsx! { MessageScrollerViewport { "aria-label": "Conversation", "a" } }
    }
    let html = render(app);

    assert!(html.contains(r#"tabindex="0""#));
    assert!(html.contains(r#"aria-label="Conversation""#));
  }

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_jump_button_is_a_plain_button_with_passed_attributes() {
    fn app() -> Element {
      rsx! { MessageScrollerJumpButton { visible: true, "aria-label": "Jump to latest", "v" } }
    }
    let html = render(app);

    assert!(html.contains(r#"type="button""#));
    assert!(html.contains(r#"aria-label="Jump to latest""#));
  }

  #[test]
  fn message_scroller_class_appends_user_class() {
    let actual = message_scroller_class("h-96");

    assert!(actual.contains(MESSAGE_SCROLLER_BASE_CLASS));
    assert!(actual.ends_with("h-96"));
  }

  #[test]
  fn message_scroller_visibility_classes_reflect_state() {
    let hidden_marker = message_scroller_unread_marker_class(false, "bottom-6");
    let visible_button = message_scroller_jump_button_class(true, "rounded-full");

    assert_eq!(
      hidden_marker,
      "pointer-events-none absolute inset-x-0 z-10 justify-center hidden bottom-6"
    );
    assert!(hidden_marker.contains("hidden"));
    assert!(hidden_marker.ends_with("bottom-6"));
    assert_eq!(
      visible_button,
      "h-9 items-center justify-center border border-border bg-background px-3 text-sm font-medium text-foreground shadow-sm transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 inline-flex rounded-full"
    );
    assert!(!visible_button.contains("hidden"));
  }

  #[test]
  fn message_scroller_part_classes_append_user_classes() {
    assert!(message_scroller_viewport_class("px-2").ends_with("px-2"));
    assert!(message_scroller_content_class("gap-6").ends_with("gap-6"));
    assert!(message_scroller_bottom_anchor_class("scroll-mb-8").ends_with("scroll-mb-8"));
  }

  #[test]
  fn maps_message_scroller_intent_attributes() {
    assert_eq!(message_scroller_intent_attribute(MessageScrollerIntent::Follow), "follow");
    assert_eq!(message_scroller_intent_attribute(MessageScrollerIntent::Hold), "hold");
    assert_eq!(
      message_scroller_intent_attribute(MessageScrollerIntent::JumpToLatest),
      "jump-to-latest"
    );
  }
}
