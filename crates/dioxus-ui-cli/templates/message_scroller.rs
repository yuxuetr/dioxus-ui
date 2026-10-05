use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MessageScrollerMetrics {
  pub scroll_top: f64,
  pub viewport_height: f64,
  pub content_height: f64,
}

impl MessageScrollerMetrics {
  pub const fn new(scroll_top: f64, viewport_height: f64, content_height: f64) -> Self {
    Self {
      scroll_top,
      viewport_height,
      content_height,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageScrollerEvent {
  UserScrolled,
  MessageAppended,
  JumpRequested,
  ReachedBottom,
  Reset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageScrollerIntent {
  Follow,
  Hold,
  JumpToLatest,
}

pub const MESSAGE_SCROLLER_BASE_CLASS: &str = "relative flex min-h-0 w-full flex-col overflow-hidden";
pub const MESSAGE_SCROLLER_VIEWPORT_BASE_CLASS: &str = "min-h-0 flex-1 overflow-y-auto overscroll-contain";
pub const MESSAGE_SCROLLER_CONTENT_BASE_CLASS: &str = "flex min-h-full flex-col gap-4";
pub const MESSAGE_SCROLLER_BOTTOM_ANCHOR_BASE_CLASS: &str = "h-px w-full shrink-0 scroll-mb-4";
pub const MESSAGE_SCROLLER_UNREAD_MARKER_BASE_CLASS: &str = "pointer-events-none absolute inset-x-0 bottom-4 z-10 flex justify-center";
pub const MESSAGE_SCROLLER_JUMP_BUTTON_BASE_CLASS: &str = "inline-flex h-9 items-center justify-center rounded-md border border-border bg-background px-3 text-sm font-medium text-foreground shadow-sm transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

impl Default for MessageScrollerIntent {
  fn default() -> Self {
    Self::Follow
  }
}

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

pub fn message_scroller_distance_to_bottom(metrics: MessageScrollerMetrics) -> f64 {
  let scroll_top = non_negative_finite(metrics.scroll_top);
  let viewport_height = non_negative_finite(metrics.viewport_height);
  let content_height = non_negative_finite(metrics.content_height);

  (content_height - viewport_height - scroll_top).max(0.0)
}

pub fn message_scroller_is_at_bottom(metrics: MessageScrollerMetrics, threshold: f64) -> bool {
  message_scroller_distance_to_bottom(metrics) <= non_negative_finite(threshold)
}

pub fn message_scroller_should_follow(
  intent: MessageScrollerIntent,
  metrics: MessageScrollerMetrics,
  threshold: f64,
) -> bool {
  matches!(
    intent,
    MessageScrollerIntent::Follow | MessageScrollerIntent::JumpToLatest
  ) || message_scroller_is_at_bottom(metrics, threshold)
}

pub fn message_scroller_show_unread_marker(
  intent: MessageScrollerIntent,
  appended_count: usize,
) -> bool {
  matches!(intent, MessageScrollerIntent::Hold) && appended_count > 0
}

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
      if at_bottom || matches!(
        intent,
        MessageScrollerIntent::Follow | MessageScrollerIntent::JumpToLatest
      ) {
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

fn non_negative_finite(value: f64) -> f64 {
  if value.is_finite() {
    value.max(0.0)
  } else {
    0.0
  }
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
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = message_scroller_jump_button_class(visible, &class);

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
