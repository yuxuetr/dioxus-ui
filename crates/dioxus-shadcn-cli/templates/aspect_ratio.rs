use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

pub const ASPECT_RATIO_BASE_CLASS: &str = "relative w-full overflow-hidden";
pub const DEFAULT_ASPECT_RATIO: f64 = 16.0 / 9.0;

pub fn aspect_ratio_class(class: &str) -> String {
  merge_classes(classes([Some(ASPECT_RATIO_BASE_CLASS)]), class)
}

pub fn aspect_ratio_style(ratio: f64) -> String {
  let ratio = aspect_ratio_value(ratio);

  format!("aspect-ratio: {ratio};")
}

pub fn aspect_ratio_value(ratio: f64) -> f64 {
  if ratio.is_finite() && ratio > 0.0 { ratio } else { DEFAULT_ASPECT_RATIO }
}

#[component]
pub fn AspectRatio(
  #[props(default = DEFAULT_ASPECT_RATIO)] ratio: f64,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = aspect_ratio_class(&class);
  let style = aspect_ratio_style(ratio);

  rsx! {
    div {
      class,
      style,
      {children}
    }
  }
}
