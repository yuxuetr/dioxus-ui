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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MessageScrollerIntent {
  #[default]
  Follow,
  Hold,
  JumpToLatest,
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

fn non_negative_finite(value: f64) -> f64 {
  if value.is_finite() {
    value.max(0.0)
  } else {
    0.0
  }
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

    assert!(message_scroller_should_follow(
      MessageScrollerIntent::Follow,
      away,
      0.0
    ));
    assert!(message_scroller_should_follow(
      MessageScrollerIntent::JumpToLatest,
      away,
      0.0
    ));
    assert!(!message_scroller_should_follow(
      MessageScrollerIntent::Hold,
      away,
      5.0
    ));
    assert!(message_scroller_should_follow(
      MessageScrollerIntent::Hold,
      near_bottom,
      5.0
    ));
  }

  #[test]
  fn shows_unread_marker_only_when_held_with_appends() {
    assert!(message_scroller_show_unread_marker(
      MessageScrollerIntent::Hold,
      1
    ));
    assert!(!message_scroller_show_unread_marker(
      MessageScrollerIntent::Hold,
      0
    ));
    assert!(!message_scroller_show_unread_marker(
      MessageScrollerIntent::Follow,
      3
    ));
    assert!(!message_scroller_show_unread_marker(
      MessageScrollerIntent::JumpToLatest,
      3
    ));
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
