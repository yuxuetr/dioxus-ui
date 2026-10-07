//! Stick-to-bottom logic for a chat or log transcript: whether new messages should scroll
//! into view, and when to show an unread marker. Used by the styled `MessageScroller` in
//! `dioxus-shadcn`.

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

fn non_negative_finite(value: f64) -> f64 {
  if value.is_finite() { value.max(0.0) } else { 0.0 }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn computes_distance_to_bottom_with_clamping() {
    let metrics = MessageScrollerMetrics::new(700.0, 300.0, 1200.0);

    assert_eq!(message_scroller_distance_to_bottom(metrics), 200.0);
    assert_eq!(
      message_scroller_distance_to_bottom(MessageScrollerMetrics::new(1000.0, 300.0, 1200.0)),
      0.0
    );
    assert_eq!(
      message_scroller_distance_to_bottom(MessageScrollerMetrics::new(f64::NAN, -10.0, 500.0)),
      500.0
    );
  }

  #[test]
  fn reports_bottom_state_with_threshold() {
    let metrics = MessageScrollerMetrics::new(895.0, 300.0, 1200.0);

    assert!(!message_scroller_is_at_bottom(metrics, 4.0));
    assert!(message_scroller_is_at_bottom(metrics, 5.0));
    assert!(!message_scroller_is_at_bottom(metrics, f64::NAN));
  }

  #[test]
  fn follows_only_when_intent_or_metrics_allow_it() {
    let away = MessageScrollerMetrics::new(100.0, 300.0, 1200.0);
    let near_bottom = MessageScrollerMetrics::new(895.0, 300.0, 1200.0);

    assert!(message_scroller_should_follow(MessageScrollerIntent::Follow, away, 0.0));
    assert!(message_scroller_should_follow(MessageScrollerIntent::JumpToLatest, away, 0.0));
    assert!(!message_scroller_should_follow(MessageScrollerIntent::Hold, away, 5.0));
    assert!(message_scroller_should_follow(MessageScrollerIntent::Hold, near_bottom, 5.0));
  }

  #[test]
  fn shows_unread_marker_only_when_held_with_appends() {
    assert!(message_scroller_show_unread_marker(MessageScrollerIntent::Hold, 1));
    assert!(!message_scroller_show_unread_marker(MessageScrollerIntent::Hold, 0));
    assert!(!message_scroller_show_unread_marker(MessageScrollerIntent::Follow, 3));
    assert!(!message_scroller_show_unread_marker(MessageScrollerIntent::JumpToLatest, 3));
  }

  #[test]
  fn transitions_for_user_scroll_append_jump_and_reset() {
    assert_eq!(
      message_scroller_next_intent(
        MessageScrollerIntent::Follow,
        MessageScrollerEvent::UserScrolled,
        false,
      ),
      MessageScrollerIntent::Hold
    );
    assert_eq!(
      message_scroller_next_intent(
        MessageScrollerIntent::Hold,
        MessageScrollerEvent::MessageAppended,
        false,
      ),
      MessageScrollerIntent::Hold
    );
    assert_eq!(
      message_scroller_next_intent(
        MessageScrollerIntent::Hold,
        MessageScrollerEvent::MessageAppended,
        true,
      ),
      MessageScrollerIntent::Follow
    );
    assert_eq!(
      message_scroller_next_intent(
        MessageScrollerIntent::Hold,
        MessageScrollerEvent::JumpRequested,
        false,
      ),
      MessageScrollerIntent::JumpToLatest
    );
    assert_eq!(
      message_scroller_next_intent(
        MessageScrollerIntent::JumpToLatest,
        MessageScrollerEvent::Reset,
        false,
      ),
      MessageScrollerIntent::Follow
    );
  }
}
