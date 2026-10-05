use dioxus::prelude::*;
use super::utils::classes;

pub const PROGRESS_BASE_CLASS: &str = "relative h-4 w-full overflow-hidden rounded-full bg-muted";
pub const PROGRESS_INDICATOR_BASE_CLASS: &str = "h-full w-full flex-1 bg-primary transition-transform";

pub fn progress_class(class: &str) -> String {
  classes([Some(PROGRESS_BASE_CLASS), Some(class)])
}

pub fn progress_indicator_class(class: &str) -> String {
  classes([Some(PROGRESS_INDICATOR_BASE_CLASS), Some(class)])
}

pub fn progress_percent(value: f32, max: f32) -> f32 {
  if max <= 0.0 {
    return 0.0;
  }

  (value / max * 100.0).clamp(0.0, 100.0)
}

#[component]
pub fn Progress(
  #[props(default)] value: f32,
  #[props(default = 100.0)] max: f32,
  #[props(default)] class: String,
  #[props(default)] indicator_class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
  let class = progress_class(&class);
  let indicator_class = progress_indicator_class(&indicator_class);
  let percent = progress_percent(value, max);
  let transform = format!("translateX(-{}%)", 100.0 - percent);

  rsx! {
    div {
      role: "progressbar",
      class,
      "aria-valuemin": "0",
      "aria-valuemax": max.to_string(),
      "aria-valuenow": value.clamp(0.0, max).to_string(),
      ..attributes,
      div {
        class: indicator_class,
        style: "transform: {transform};",
      }
    }
  }
}
