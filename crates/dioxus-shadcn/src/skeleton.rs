//! Skeleton: an animated placeholder for content that is still loading.
use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

const SKELETON_BASE_CLASS: &str = "animate-pulse rounded-md bg-accent";

/// Classes for the pulsing placeholder, with `class` merged over them; set its size with `class`.
pub fn skeleton_class(class: &str) -> String {
  merge_classes(classes([Some(SKELETON_BASE_CLASS)]), class)
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
