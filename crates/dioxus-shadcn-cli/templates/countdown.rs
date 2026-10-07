//! Countdown: the time left, such as until a sale ends, as `D:HH:MM:SS` with
//! fixed-width digits.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// Remaining seconds split into days, hours, minutes, and seconds.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CountdownParts {
  /// Whole days.
  pub days: u64,
  /// Hours past the whole days, 0 to 23.
  pub hours: u64,
  /// Minutes past the whole hours, 0 to 59.
  pub minutes: u64,
  /// Seconds past the whole minutes, 0 to 59.
  pub seconds: u64,
}

/// Splits `remaining` seconds into days, hours, minutes, and seconds.
pub fn countdown_parts(remaining: u64) -> CountdownParts {
  CountdownParts {
    days: remaining / 86_400,
    hours: remaining % 86_400 / 3_600,
    minutes: remaining % 3_600 / 60,
    seconds: remaining % 60,
  }
}

/// The segments as text: `HH:MM:SS`, with a leading day count once the
/// remaining time reaches a day.
fn countdown_segments(remaining: u64) -> Vec<String> {
  let parts = countdown_parts(remaining);
  let mut segments = Vec::with_capacity(4);
  if parts.days > 0 {
    segments.push(parts.days.to_string());
  }
  segments.push(format!("{:02}", parts.hours));
  segments.push(format!("{:02}", parts.minutes));
  segments.push(format!("{:02}", parts.seconds));
  segments
}

const COUNTDOWN_BASE_CLASS: &str = "inline-flex items-baseline font-semibold tabular-nums";
const COUNTDOWN_SEPARATOR_CLASS: &str = "px-0.5 text-muted-foreground";

/// Classes for the timer: bold tabular digits, with `class` merged over them.
pub fn countdown_class(class: &str) -> String {
  merge_classes(classes([Some(COUNTDOWN_BASE_CLASS)]), class)
}

/// The time left, in `remaining` seconds, as `D:HH:MM:SS`. The app owns the
/// clock and passes the new value each second. `role="timer"` is not a live
/// region, so screen readers do not announce every tick; name the timer with
/// `aria-label`.
#[component]
pub fn Countdown(
  remaining: u64,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
  let class = countdown_class(&class);
  let segments = countdown_segments(remaining);
  let last = segments.len() - 1;

  rsx! {
    span { class, role: "timer", ..attributes,
      for (index, segment) in segments.into_iter().enumerate() {
        span { key: "{index}", "{segment}" }
        if index < last {
          span { key: "{index}-separator", class: COUNTDOWN_SEPARATOR_CLASS, ":" }
        }
      }
    }
  }
}
