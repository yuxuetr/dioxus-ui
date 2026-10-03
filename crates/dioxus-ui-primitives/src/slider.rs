#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderState {
  pub value: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,
  pub page_step: f64,
}

impl SliderState {
  pub fn new(value: f64, min: f64, max: f64, step: f64) -> Self {
    let (min, max) = ordered_bounds(min, max);
    let step = positive_or_default(step, 1.0);
    let page_step = step * 10.0;
    let value = snap_value(value, min, max, step);

    Self { value, min, max, step, page_step }
  }

  pub fn with_page_step(mut self, page_step: f64) -> Self {
    self.page_step = positive_or_default(page_step, self.step * 10.0);
    self
  }

  pub fn with_value(self, value: f64) -> Self {
    Self { value: snap_value(value, self.min, self.max, self.step), ..self }
  }

  pub fn percent(self) -> f64 {
    if self.max <= self.min {
      return 0.0;
    }

    ((self.value - self.min) / (self.max - self.min) * 100.0).clamp(0.0, 100.0)
  }

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

  pub fn aria_attributes(self) -> SliderAriaAttributes {
    SliderAriaAttributes {
      aria_valuemin: self.min,
      aria_valuemax: self.max,
      aria_valuenow: self.value,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SliderKeyMove {
  Decrement,
  Increment,
  PageDecrement,
  PageIncrement,
  Home,
  End,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderAriaAttributes {
  pub aria_valuemin: f64,
  pub aria_valuemax: f64,
  pub aria_valuenow: f64,
}

pub fn slider_clamp(value: f64, min: f64, max: f64) -> f64 {
  let (min, max) = ordered_bounds(min, max);

  if value.is_finite() { value.clamp(min, max) } else { min }
}

pub fn slider_snap(value: f64, min: f64, max: f64, step: f64) -> f64 {
  let (min, max) = ordered_bounds(min, max);
  let step = positive_or_default(step, 1.0);

  snap_value(value, min, max, step)
}

pub fn slider_percent(value: f64, min: f64, max: f64) -> f64 {
  SliderState::new(value, min, max, 1.0).percent()
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
    assert_eq!(slider_snap(4.2, 0.0, 10.0, 2.0), 4.0);
    assert_eq!(slider_snap(4.9, 0.0, 10.0, 2.0), 4.0);
    assert_eq!(slider_snap(5.1, 0.0, 10.0, 2.0), 6.0);
    assert_eq!(slider_snap(9.9, 0.0, 10.0, 2.0), 10.0);
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
