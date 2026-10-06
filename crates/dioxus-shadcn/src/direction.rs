use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextDirection {
  #[default]
  Ltr,
  Rtl,
}

impl TextDirection {
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Ltr => "ltr",
      Self::Rtl => "rtl",
    }
  }
}

pub const DIRECTION_BASE_CLASS: &str = "contents";

pub fn direction_class(class: &str) -> String {
  merge_classes(classes([Some(DIRECTION_BASE_CLASS)]), class)
}

#[component]
pub fn Direction(
  #[props(default)] dir: TextDirection,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = direction_class(&class);
  let dir = dir.attribute();

  rsx! {
    div {
      class,
      "dir": dir,
      "data-direction": dir,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn text_direction_attribute_maps_to_native_dir_values() {
    assert_eq!(TextDirection::Ltr.attribute(), "ltr");
    assert_eq!(TextDirection::Rtl.attribute(), "rtl");
  }

  #[test]
  fn direction_class_appends_user_class() {
    let actual = direction_class("block");

    assert_eq!(actual, "block");
    assert!(actual.ends_with("block"));
  }
}
