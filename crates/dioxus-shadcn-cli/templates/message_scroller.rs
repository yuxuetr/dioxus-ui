//! Message Scroller: controlled transcript viewport parts and pure scroll intent
//! helpers for chat, logs, and message feeds.
use super::default_attribute::default_attribute;
use super::density::{density_control_class, use_density, with_density};
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// Scroll measurements of the transcript viewport, in CSS pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MessageScrollerMetrics {
  /// How far the viewport is scrolled down (`scrollTop`).
  pub scroll_top: f64,
  /// Visible height of the viewport (`clientHeight`).
  pub viewport_height: f64,
  /// Full height of the scrolled content (`scrollHeight`).
  pub content_height: f64,
}

impl MessageScrollerMetrics {
  /// Measurements as given. Negative or non-finite values count as 0 when read.
  pub const fn new(scroll_top: f64, viewport_height: f64, content_height: f64) -> Self {
    Self { scroll_top, viewport_height, content_height }
  }
}

/// Something that happened to the transcript, fed to `message_scroller_next_intent`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageScrollerEvent {
  /// The user scrolled the viewport.
  UserScrolled,
  /// A new message was added.
  MessageAppended,
  /// The user asked to jump to the newest message, such as with a jump button.
  JumpRequested,
  /// The viewport reached the bottom.
  ReachedBottom,
  /// The transcript was cleared or replaced.
  Reset,
}

/// Whether the transcript should keep the newest message in view.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MessageScrollerIntent {
  /// Stay pinned to the bottom as messages arrive.
  #[default]
  Follow,
  /// Keep the user's scroll position; new messages arrive out of view.
  Hold,
  /// Scroll to the newest message, then follow.
  JumpToLatest,
}

const MESSAGE_SCROLLER_BASE_CLASS: &str =
  "relative flex min-h-0 w-full flex-col overflow-hidden";
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

/// Pixels left to scroll before the bottom; never negative.
pub fn message_scroller_distance_to_bottom(metrics: MessageScrollerMetrics) -> f64 {
  let scroll_top = non_negative_finite(metrics.scroll_top);
  let viewport_height = non_negative_finite(metrics.viewport_height);
  let content_height = non_negative_finite(metrics.content_height);

  (content_height - viewport_height - scroll_top).max(0.0)
}

/// Whether the viewport is within `threshold` CSS pixels of the bottom. A negative or
/// non-finite threshold counts as 0.
pub fn message_scroller_is_at_bottom(metrics: MessageScrollerMetrics, threshold: f64) -> bool {
  message_scroller_distance_to_bottom(metrics) <= non_negative_finite(threshold)
}

/// Whether to scroll to the bottom now: when following or jumping, or when already within
/// `threshold` CSS pixels of it.
pub fn message_scroller_should_follow(
  intent: MessageScrollerIntent,
  metrics: MessageScrollerMetrics,
  threshold: f64,
) -> bool {
  matches!(intent, MessageScrollerIntent::Follow | MessageScrollerIntent::JumpToLatest)
    || message_scroller_is_at_bottom(metrics, threshold)
}

/// Whether to show the unread marker: the user is holding their position and
/// `appended_count` messages have arrived since.
pub fn message_scroller_show_unread_marker(
  intent: MessageScrollerIntent,
  appended_count: usize,
) -> bool {
  matches!(intent, MessageScrollerIntent::Hold) && appended_count > 0
}

/// The intent after `event`. Scrolling follows when it ends `at_bottom` and holds otherwise;
/// a new message keeps following if the viewport was following, jumping, or at the bottom,
/// and holds otherwise; a jump request jumps; reaching the bottom or a reset follows.
pub fn message_scroller_next_intent(
  intent: MessageScrollerIntent,
  event: MessageScrollerEvent,
  at_bottom: bool,
) -> MessageScrollerIntent {
  match event {
    MessageScrollerEvent::UserScrolled => {
      if at_bottom {
        MessageScrollerIntent::Follow
      } else {
        MessageScrollerIntent::Hold
      }
    }
    MessageScrollerEvent::MessageAppended => {
      if at_bottom
        || matches!(intent, MessageScrollerIntent::Follow | MessageScrollerIntent::JumpToLatest)
      {
        MessageScrollerIntent::Follow
      } else {
        MessageScrollerIntent::Hold
      }
    }
    MessageScrollerEvent::JumpRequested => MessageScrollerIntent::JumpToLatest,
    MessageScrollerEvent::ReachedBottom | MessageScrollerEvent::Reset => {
      MessageScrollerIntent::Follow
    }
  }
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
  merge_classes(classes([Some(MESSAGE_SCROLLER_UNREAD_MARKER_BASE_CLASS), Some(if visible { "flex" } else { "hidden" })]), class)
}

/// Classes for the jump-to-latest button: hidden unless `visible`, then `class` merged
/// over them.
pub fn message_scroller_jump_button_class(visible: bool, class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_SCROLLER_JUMP_BUTTON_BASE_CLASS), Some(if visible { "inline-flex" } else { "hidden" })]), class)
}

fn non_negative_finite(value: f64) -> f64 {
  if value.is_finite() { value.max(0.0) } else { 0.0 }
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
  let class = message_scroller_jump_button_class(visible, &with_density(density_control_class(use_density()), &class));

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
