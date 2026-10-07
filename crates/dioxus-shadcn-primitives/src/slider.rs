//! Value, keyboard, and ARIA state for a single-thumb slider, used by the styled `Slider` in
//! `dioxus-shadcn`.

/// A slider's value and range. Built through `new`, the value is snapped to a step and inside
/// `min..=max`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderState {
  /// Current value, snapped to `min + n * step` and clamped to the range.
  pub value: f64,
  /// Lowest value.
  pub min: f64,
  /// Highest value.
  pub max: f64,
  /// Distance an arrow key moves; always positive.
  pub step: f64,
  /// Distance Page Up and Page Down move; always positive, 10 steps by default.
  pub page_step: f64,
}

impl SliderState {
  /// A slider with `value` snapped and clamped. Swapped bounds are put in order; a non-finite
  /// `min` becomes 0 and a non-finite `max` becomes `min`. A `step` that is not a positive
  /// finite number becomes 1, and a non-finite `value` becomes `min`.
  pub fn new(value: f64, min: f64, max: f64, step: f64) -> Self {
    let (min, max) = ordered_bounds(min, max);
    let step = positive_or_default(step, 1.0);
    let page_step = step * 10.0;
    let value = snap_value(value, min, max, step);

    Self { value, min, max, step, page_step }
  }

  /// The same slider with a page step; one that is not a positive finite number falls back to
  /// 10 steps.
  pub fn with_page_step(mut self, page_step: f64) -> Self {
    self.page_step = positive_or_default(page_step, self.step * 10.0);
    self
  }

  /// The same slider at `value`, snapped and clamped.
  pub fn with_value(self, value: f64) -> Self {
    Self { value: snap_value(value, self.min, self.max, self.step), ..self }
  }

  /// How far the value sits along the range, from 0 to 100; 0 when the range is empty.
  pub fn percent(self) -> f64 {
    if self.max <= self.min {
      return 0.0;
    }

    ((self.value - self.min) / (self.max - self.min) * 100.0).clamp(0.0, 100.0)
  }

  /// The slider after a keyboard move, snapped and clamped.
  pub fn moved(self, movement: SliderKeyMove) -> Self {
    let value = match movement {
      SliderKeyMove::Decrement => self.value - self.step,
      SliderKeyMove::Increment => self.value + self.step,
      SliderKeyMove::PageDecrement => self.value - self.page_step,
      SliderKeyMove::PageIncrement => self.value + self.page_step,
      SliderKeyMove::Home => self.min,
      SliderKeyMove::End => self.max,
    };

    self.with_value(value)
  }

  /// The `aria-valuemin`, `aria-valuemax`, and `aria-valuenow` values for the thumb.
  pub fn aria_attributes(self) -> SliderAriaAttributes {
    SliderAriaAttributes {
      aria_valuemin: self.min,
      aria_valuemax: self.max,
      aria_valuenow: self.value,
    }
  }
}

/// A keyboard move on a focused slider, as in the WAI-ARIA slider pattern.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SliderKeyMove {
  /// One step down (Left or Down arrow).
  Decrement,
  /// One step up (Right or Up arrow).
  Increment,
  /// One page step down (Page Down).
  PageDecrement,
  /// One page step up (Page Up).
  PageIncrement,
  /// Jump to `min` (Home).
  Home,
  /// Jump to `max` (End).
  End,
}

/// The ARIA range values a slider thumb reports.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderAriaAttributes {
  /// `aria-valuemin`.
  pub aria_valuemin: f64,
  /// `aria-valuemax`.
  pub aria_valuemax: f64,
  /// `aria-valuenow`.
  pub aria_valuenow: f64,
}

pub(crate) fn slider_clamp(value: f64, min: f64, max: f64) -> f64 {
  let (min, max) = ordered_bounds(min, max);

  if value.is_finite() { value.clamp(min, max) } else { min }
}

fn snap_value(value: f64, min: f64, max: f64, step: f64) -> f64 {
  let clamped = slider_clamp(value, min, max);
  let steps = ((clamped - min) / step).round();
  let snapped = min + steps * step;

  slider_clamp(snapped, min, max)
}

fn ordered_bounds(min: f64, max: f64) -> (f64, f64) {
  let min = finite_or_default(min, 0.0);
  let max = finite_or_default(max, min);

  if min <= max { (min, max) } else { (max, min) }
}

fn positive_or_default(value: f64, default: f64) -> f64 {
  if value.is_finite() && value > 0.0 { value } else { default }
}

fn finite_or_default(value: f64, default: f64) -> f64 {
  if value.is_finite() { value } else { default }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn clamps_values_to_bounds() {
    assert_eq!(slider_clamp(-5.0, 0.0, 10.0), 0.0);
    assert_eq!(slider_clamp(15.0, 0.0, 10.0), 10.0);
    assert_eq!(slider_clamp(7.0, 10.0, 0.0), 7.0);
  }

  #[test]
  fn snaps_values_to_step_from_min() {
    assert_eq!(snap_value(4.2, 0.0, 10.0, 2.0), 4.0);
    assert_eq!(snap_value(4.9, 0.0, 10.0, 2.0), 4.0);
    assert_eq!(snap_value(5.1, 0.0, 10.0, 2.0), 6.0);
    assert_eq!(snap_value(9.9, 0.0, 10.0, 2.0), 10.0);
  }

  #[test]
  fn calculates_percent_for_ranges() {
    assert_eq!(SliderState::new(50.0, 0.0, 100.0, 1.0).percent(), 50.0);
    assert_eq!(SliderState::new(0.0, -10.0, 10.0, 1.0).percent(), 50.0);
    assert_eq!(SliderState::new(10.0, 10.0, 10.0, 1.0).percent(), 0.0);
  }

  #[test]
  fn keyboard_moves_use_step_and_bounds() {
    let state = SliderState::new(5.0, 0.0, 10.0, 1.0);

    assert_eq!(state.moved(SliderKeyMove::Increment).value, 6.0);
    assert_eq!(state.moved(SliderKeyMove::Decrement).value, 4.0);
    assert_eq!(state.moved(SliderKeyMove::Home).value, 0.0);
    assert_eq!(state.moved(SliderKeyMove::End).value, 10.0);
  }

  #[test]
  fn page_keyboard_moves_use_page_step() {
    let state = SliderState::new(40.0, 0.0, 100.0, 5.0).with_page_step(25.0);

    assert_eq!(state.moved(SliderKeyMove::PageIncrement).value, 65.0);
    assert_eq!(state.moved(SliderKeyMove::PageDecrement).value, 15.0);
  }

  #[test]
  fn aria_attributes_map_to_state() {
    let attributes = SliderState::new(6.0, 0.0, 10.0, 2.0).aria_attributes();

    assert_eq!(attributes.aria_valuemin, 0.0);
    assert_eq!(attributes.aria_valuemax, 10.0);
    assert_eq!(attributes.aria_valuenow, 6.0);
  }
}
