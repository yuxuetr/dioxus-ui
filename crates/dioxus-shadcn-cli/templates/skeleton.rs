use super::utils::classes;
use dioxus::prelude::*;

pub const SKELETON_BASE_CLASS: &str = "animate-pulse rounded-md bg-accent";

pub fn skeleton_class(class: &str) -> String {
  classes([Some(SKELETON_BASE_CLASS), Some(class)])
}

#[component]
pub fn Skeleton(#[props(default)] class: String) -> Element {
  let class = skeleton_class(&class);

  rsx! {
    div {
      class,
      "aria-hidden": "true",
    }
  }
}
