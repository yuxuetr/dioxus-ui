use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderState {
  pub value: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,
}

impl SliderState {
  pub fn new(value: f64, min: f64, max: f64, step: f64) -> Self {
    let (min, max) = ordered_bounds(min, max);
    let step = positive_or_default(step, 1.0);
    let value = snap_value(value, min, max, step);

    Self {
      value,
      min,
      max,
      step,
    }
  }

  pub fn percent(self) -> f64 {
    if self.max <= self.min {
      return 0.0;
    }

    ((self.value - self.min) / (self.max - self.min) * 100.0).clamp(0.0, 100.0)
  }

  pub fn aria_attributes(self) -> SliderAriaAttributes {
    SliderAriaAttributes {
      aria_valuemin: self.min,
      aria_valuemax: self.max,
      aria_valuenow: self.value,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderAriaAttributes {
  pub aria_valuemin: f64,
  pub aria_valuemax: f64,
  pub aria_valuenow: f64,
}

pub const SLIDER_ROOT_BASE_CLASS: &str =
  "relative flex w-full touch-none select-none items-center disabled:opacity-50";
pub const SLIDER_TRACK_BASE_CLASS: &str =
  "relative h-2 w-full grow overflow-hidden rounded-full bg-zinc-100";
pub const SLIDER_RANGE_BASE_CLASS: &str = "absolute h-full rounded-full bg-blue-600";
pub const SLIDER_THUMB_BASE_CLASS: &str = "block h-5 w-5 rounded-full border-2 border-blue-600 bg-white shadow transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";

pub fn slider_root_class(class: &str) -> String {
  classes([Some(SLIDER_ROOT_BASE_CLASS), Some(class)])
}

pub fn slider_track_class(class: &str) -> String {
  classes([Some(SLIDER_TRACK_BASE_CLASS), Some(class)])
}

pub fn slider_range_class(class: &str) -> String {
  classes([Some(SLIDER_RANGE_BASE_CLASS), Some(class)])
}

pub fn slider_thumb_class(class: &str) -> String {
  classes([Some(SLIDER_THUMB_BASE_CLASS), Some(class)])
}

pub fn slider_state(value: f64, min: f64, max: f64, step: f64) -> SliderState {
  SliderState::new(value, min, max, step)
}

pub fn slider_percent(value: f64, min: f64, max: f64, step: f64) -> f64 {
  slider_state(value, min, max, step).percent()
}

pub fn slider_range_style(percent: f64) -> String {
  let percent = percent.clamp(0.0, 100.0);

  format!("left: 0%; width: {percent}%;")
}

pub fn slider_thumb_style(percent: f64) -> String {
  let percent = percent.clamp(0.0, 100.0);

  format!("left: {percent}%; transform: translateX(-50%);")
}

pub fn slider_aria_attributes(
  value: f64,
  min: f64,
  max: f64,
  step: f64,
) -> SliderAriaAttributes {
  slider_state(value, min, max, step).aria_attributes()
}

fn snap_value(value: f64, min: f64, max: f64, step: f64) -> f64 {
  let clamped = slider_clamp(value, min, max);
  let steps = ((clamped - min) / step).round();
  let snapped = min + steps * step;

  slider_clamp(snapped, min, max)
}

fn slider_clamp(value: f64, min: f64, max: f64) -> f64 {
  let (min, max) = ordered_bounds(min, max);

  if value.is_finite() {
    value.clamp(min, max)
  } else {
    min
  }
}

fn ordered_bounds(min: f64, max: f64) -> (f64, f64) {
  let min = finite_or_default(min, 0.0);
  let max = finite_or_default(max, min);

  if min <= max {
    (min, max)
  } else {
    (max, min)
  }
}

fn positive_or_default(value: f64, default: f64) -> f64 {
  if value.is_finite() && value > 0.0 {
    value
  } else {
    default
  }
}

fn finite_or_default(value: f64, default: f64) -> f64 {
  if value.is_finite() {
    value
  } else {
    default
  }
}

#[component]
pub fn Slider(
  #[props(default)] value: f64,
  #[props(default = 0.0)] min: f64,
  #[props(default = 100.0)] max: f64,
  #[props(default = 1.0)] step: f64,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] track_class: String,
  #[props(default)] range_class: String,
  #[props(default)] thumb_class: String,
) -> Element {
  let state = slider_state(value, min, max, step);
  let percent = state.percent();
  let aria = state.aria_attributes();
  let root_class = slider_root_class(&class);
  let track_class = slider_track_class(&track_class);
  let range_class = slider_range_class(&range_class);
  let thumb_class = slider_thumb_class(&thumb_class);
  let range_style = slider_range_style(percent);
  let thumb_style = slider_thumb_style(percent);

  rsx! {
    div {
      role: "slider",
      class: root_class,
      tabindex: if disabled { "-1" } else { "0" },
      "aria-disabled": disabled.to_string(),
      "aria-orientation": "horizontal",
      "aria-valuemin": aria.aria_valuemin.to_string(),
      "aria-valuemax": aria.aria_valuemax.to_string(),
      "aria-valuenow": aria.aria_valuenow.to_string(),
      "data-value": state.value.to_string(),
      "data-disabled": disabled.to_string(),
      div {
        class: track_class,
        div {
          class: range_class,
          style: range_style,
        }
      }
      span {
        class: thumb_class,
        style: thumb_style,
      }
    }
  }
}
