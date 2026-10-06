use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

pub const LABEL_BASE_CLASS: &str = "text-sm font-medium leading-none text-foreground peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ssr_omits_an_empty_for() {
    fn app() -> Element {
      rsx! {
        Label { r#for: "name", "Name" }
        Label { "Wrapped" }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert_eq!(html.matches("for=").count(), 1);
    assert!(html.contains(r#"for="name""#));
  }

  #[test]
  fn label_class_appends_user_class() {
    let actual = label_class("tracking-wide");

    assert!(actual.contains(LABEL_BASE_CLASS));
    assert!(actual.ends_with("tracking-wide"));
  }
}
