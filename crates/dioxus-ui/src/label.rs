use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const LABEL_BASE_CLASS: &str = "text-sm font-medium leading-none text-zinc-900 peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn label_class_appends_user_class() {
    let actual = label_class("tracking-wide");

    assert!(actual.contains(LABEL_BASE_CLASS));
    assert!(actual.ends_with("tracking-wide"));
  }
}
