use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  message_scroller_distance_to_bottom, message_scroller_is_at_bottom,
  message_scroller_next_intent, message_scroller_should_follow,
  message_scroller_show_unread_marker, MessageScrollerEvent, MessageScrollerIntent,
  MessageScrollerMetrics,
};

pub const MESSAGE_SCROLLER_BASE_CLASS: &str = "relative flex min-h-0 w-full flex-col overflow-hidden";
pub const MESSAGE_SCROLLER_VIEWPORT_BASE_CLASS: &str = "min-h-0 flex-1 overflow-y-auto overscroll-contain";
pub const MESSAGE_SCROLLER_CONTENT_BASE_CLASS: &str = "flex min-h-full flex-col gap-4";
pub const MESSAGE_SCROLLER_BOTTOM_ANCHOR_BASE_CLASS: &str = "h-px w-full shrink-0 scroll-mb-4";
pub const MESSAGE_SCROLLER_UNREAD_MARKER_BASE_CLASS: &str = "pointer-events-none absolute inset-x-0 bottom-4 z-10 flex justify-center";
pub const MESSAGE_SCROLLER_JUMP_BUTTON_BASE_CLASS: &str = "inline-flex h-9 items-center justify-center rounded-md border border-zinc-200 bg-white px-3 text-sm font-medium text-zinc-950 shadow-sm transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";

pub const fn message_scroller_intent_attribute(intent: MessageScrollerIntent) -> &'static str {
  match intent {
    MessageScrollerIntent::Follow => "follow",
    MessageScrollerIntent::Hold => "hold",
    MessageScrollerIntent::JumpToLatest => "jump-to-latest",
  }
}

pub const fn message_scroller_is_following_intent(intent: MessageScrollerIntent) -> bool {
  matches!(
    intent,
    MessageScrollerIntent::Follow | MessageScrollerIntent::JumpToLatest
  )
}

pub fn message_scroller_class(_intent: MessageScrollerIntent, class: &str) -> String {
  classes([Some(MESSAGE_SCROLLER_BASE_CLASS), Some(class)])
}

pub fn message_scroller_viewport_class(class: &str) -> String {
  classes([Some(MESSAGE_SCROLLER_VIEWPORT_BASE_CLASS), Some(class)])
}

pub fn message_scroller_content_class(class: &str) -> String {
  classes([Some(MESSAGE_SCROLLER_CONTENT_BASE_CLASS), Some(class)])
}

pub fn message_scroller_bottom_anchor_class(class: &str) -> String {
  classes([Some(MESSAGE_SCROLLER_BOTTOM_ANCHOR_BASE_CLASS), Some(class)])
}

pub fn message_scroller_unread_marker_class(visible: bool, class: &str) -> String {
  classes([
    Some(MESSAGE_SCROLLER_UNREAD_MARKER_BASE_CLASS),
    (!visible).then_some("hidden"),
    Some(class),
  ])
}

pub fn message_scroller_jump_button_class(visible: bool, class: &str) -> String {
  classes([
    Some(MESSAGE_SCROLLER_JUMP_BUTTON_BASE_CLASS),
    (!visible).then_some("hidden"),
    Some(class),
  ])
}

#[component]
pub fn MessageScroller(
  #[props(default)] intent: MessageScrollerIntent,
  #[props(default)] has_unread: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = message_scroller_class(intent, &class);
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
pub fn MessageScrollerViewport(#[props(default)] class: String, children: Element) -> Element {
  let class = message_scroller_viewport_class(&class);

  rsx! {
    div {
      class,
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
  children: Element,
) -> Element {
  let class = message_scroller_jump_button_class(visible, &class);

  rsx! {
    button {
      class,
      "data-visible": visible.to_string(),
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn message_scroller_class_reflects_intent() {
    let held = message_scroller_class(MessageScrollerIntent::Hold, "h-96");
    let following = message_scroller_class(MessageScrollerIntent::JumpToLatest, "");

    assert!(held.contains(MESSAGE_SCROLLER_BASE_CLASS));
    assert!(held.ends_with("h-96"));
    assert!(following.contains(MESSAGE_SCROLLER_BASE_CLASS));
  }

  #[test]
  fn message_scroller_visibility_classes_reflect_state() {
    let hidden_marker = message_scroller_unread_marker_class(false, "bottom-6");
    let visible_button = message_scroller_jump_button_class(true, "rounded-full");

    assert!(hidden_marker.contains(MESSAGE_SCROLLER_UNREAD_MARKER_BASE_CLASS));
    assert!(hidden_marker.contains("hidden"));
    assert!(hidden_marker.ends_with("bottom-6"));
    assert!(visible_button.contains(MESSAGE_SCROLLER_JUMP_BUTTON_BASE_CLASS));
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
    assert_eq!(
      message_scroller_intent_attribute(MessageScrollerIntent::Follow),
      "follow"
    );
    assert_eq!(
      message_scroller_intent_attribute(MessageScrollerIntent::Hold),
      "hold"
    );
    assert_eq!(
      message_scroller_intent_attribute(MessageScrollerIntent::JumpToLatest),
      "jump-to-latest"
    );
  }
}
