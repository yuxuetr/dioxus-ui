use dioxus::prelude::*;
use dioxus_ui_core::classes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SeparatorOrientation {
  #[default]
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

pub const SEPARATOR_BASE_CLASS: &str = "shrink-0 bg-zinc-200";

pub fn separator_class(orientation: SeparatorOrientation, class: &str) -> String {
  classes([Some(SEPARATOR_BASE_CLASS), Some(orientation.class()), Some(class)])
}

#[component]
pub fn Separator(
  #[props(default)] orientation: SeparatorOrientation,
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn separator_class_reflects_orientation() {
    let actual = separator_class(SeparatorOrientation::Vertical, "mx-2");

    assert!(actual.contains(SEPARATOR_BASE_CLASS));
    assert!(actual.contains("h-full w-px"));
    assert!(actual.ends_with("mx-2"));
  }
}
