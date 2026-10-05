use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeparatorOrientation {
  Horizontal,
  Vertical,
}

impl SeparatorOrientation {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Horizontal => "h-px w-full",
      Self::Vertical => "h-full w-px",
    }
  }

  pub const fn aria_orientation(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

pub const SEPARATOR_BASE_CLASS: &str = "shrink-0 bg-border";

pub fn separator_class(orientation: SeparatorOrientation, class: &str) -> String {
  classes([Some(SEPARATOR_BASE_CLASS), Some(orientation.class()), Some(class)])
}

#[component]
pub fn Separator(
  #[props(default = SeparatorOrientation::Horizontal)] orientation: SeparatorOrientation,
  #[props(default)] class: String,
  #[props(default)] decorative: bool,
) -> Element {
  let class = separator_class(orientation, &class);
  let role = if decorative { "presentation" } else { "separator" };
  let aria_hidden = decorative.to_string();

  rsx! {
    div {
      class,
      role,
      "aria-hidden": aria_hidden,
      "aria-orientation": orientation.aria_orientation(),
    }
  }
}
