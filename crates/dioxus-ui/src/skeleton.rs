use dioxus::prelude::*;
use dioxus_ui_core::classes;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn skeleton_class_appends_user_class() {
    let actual = skeleton_class("h-4 w-32");

    assert!(actual.contains(SKELETON_BASE_CLASS));
    assert!(actual.ends_with("h-4 w-32"));
  }
}
