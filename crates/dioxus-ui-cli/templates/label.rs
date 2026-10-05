use dioxus::prelude::*;
use super::utils::classes;

pub const LABEL_BASE_CLASS: &str = "text-sm font-medium leading-none text-foreground peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

pub fn label_class(class: &str) -> String {
  classes([Some(LABEL_BASE_CLASS), Some(class)])
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

  rsx! {
    label {
      class,
      r#for,
      ..attributes,
      {children}
    }
  }
}
