//! Label: consistent text styling for the label of a form control.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

const LABEL_BASE_CLASS: &str = "text-sm font-medium leading-none text-foreground peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

/// Classes for the label, dimmed when its peer control is disabled, with `class` merged over them.
pub fn label_class(class: &str) -> String {
  merge_classes(classes([Some(LABEL_BASE_CLASS)]), class)
}

/// Other attributes, such as `id` for an `aria-labelledby` reference, are
/// passed to the label.
#[component]
pub fn Label(
  #[props(default)] r#for: String,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = label)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = label_class(&class);
  // An empty `for` would point at no control, unlinking a wrapped input.
  let r#for = (!r#for.is_empty()).then_some(r#for);

  rsx! {
    label {
      class,
      r#for,
      ..attributes,
      {children}
    }
  }
}
