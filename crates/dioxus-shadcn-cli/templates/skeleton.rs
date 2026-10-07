//! Skeleton: an animated placeholder for content that is still loading.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

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
