use dioxus::prelude::*;
use dioxus_ui_core::classes;
use dioxus_ui_primitives::{SliderAriaAttributes, SliderState};

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn slider_classes_append_user_classes() {
    assert!(slider_root_class("mt-2").contains(SLIDER_ROOT_BASE_CLASS));
    assert!(slider_track_class("h-3").ends_with("h-3"));
    assert!(slider_range_class("bg-zinc-900").ends_with("bg-zinc-900"));
    assert!(slider_thumb_class("h-6").ends_with("h-6"));
  }

  #[test]
  fn slider_percent_uses_primitive_state() {
    assert_eq!(slider_percent(5.0, 0.0, 10.0, 1.0), 50.0);
    assert_eq!(slider_percent(15.0, 0.0, 10.0, 1.0), 100.0);
  }

  #[test]
  fn slider_styles_clamp_percent() {
    assert_eq!(slider_range_style(125.0), "left: 0%; width: 100%;");
    assert_eq!(
      slider_thumb_style(-10.0),
      "left: 0%; transform: translateX(-50%);"
    );
  }

  #[test]
  fn slider_aria_attributes_reflect_snapped_value() {
    let attributes = slider_aria_attributes(4.9, 0.0, 10.0, 2.0);

    assert_eq!(attributes.aria_valuemin, 0.0);
    assert_eq!(attributes.aria_valuemax, 10.0);
    assert_eq!(attributes.aria_valuenow, 4.0);
  }
}
