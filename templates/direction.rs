use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDirection {
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
  classes([Some(DIRECTION_BASE_CLASS), Some(class)])
}

#[component]
pub fn Direction(
  #[props(default = TextDirection::Ltr)] dir: TextDirection,
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
