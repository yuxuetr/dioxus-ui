use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

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
