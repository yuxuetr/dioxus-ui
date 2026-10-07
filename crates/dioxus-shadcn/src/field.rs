//! Field: the layout around a form control, its label, description, and error message.
//! Control IDs and validation stay app-owned.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

const FIELD_BASE_CLASS: &str = "grid gap-2 data-[disabled=true]:opacity-50";
const FIELD_INVALID_CLASS: &str = "data-[invalid=true]:text-destructive";
const FIELD_LABEL_BASE_CLASS: &str = "text-sm font-medium leading-none text-foreground";
const FIELD_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
const FIELD_ERROR_BASE_CLASS: &str = "text-sm font-medium text-destructive";
const FIELD_GROUP_BASE_CLASS: &str = "grid gap-4";

/// Classes for the field: base classes, plus destructive text while `invalid`, with `class`
/// merged over them.
pub fn field_class(invalid: bool, class: &str) -> String {
  merge_classes(classes([Some(FIELD_BASE_CLASS), invalid.then_some(FIELD_INVALID_CLASS)]), class)
}

/// Classes for the field's label.
pub fn field_label_class(class: &str) -> String {
  merge_classes(classes([Some(FIELD_LABEL_BASE_CLASS)]), class)
}

/// Classes for the help text under the control.
pub fn field_description_class(class: &str) -> String {
  merge_classes(classes([Some(FIELD_DESCRIPTION_BASE_CLASS)]), class)
}

/// Classes for the error message.
pub fn field_error_class(class: &str) -> String {
  merge_classes(classes([Some(FIELD_ERROR_BASE_CLASS)]), class)
}

/// Classes for the group that spaces several fields apart.
pub fn field_group_class(class: &str) -> String {
  merge_classes(classes([Some(FIELD_GROUP_BASE_CLASS)]), class)
}

#[component]
pub fn Field(
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = field_class(invalid, &class);

  rsx! {
    div {
      class,
      "data-disabled": disabled.to_string(),
      "data-invalid": invalid.to_string(),
      {children}
    }
  }
}

#[component]
pub fn FieldLabel(
  #[props(default)] r#for: String,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = label)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = field_label_class(&class);
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

#[component]
pub fn FieldDescription(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = field_description_class(&class);

  rsx! {
    p {
      class,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn FieldError(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = field_error_class(&class);

  rsx! {
    p {
      class,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn FieldGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = field_group_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ssr_description_and_error_take_ids_for_aria_describedby() {
    fn app() -> Element {
      rsx! {
        FieldDescription { id: "email-hint", "We never share it." }
        FieldError { id: "email-error", role: "alert", "Enter your email address." }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains(r#"id="email-hint""#));
    assert!(html.contains(r#"id="email-error""#));
    assert!(html.contains(r#"role="alert""#));
  }

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_label_renders_for_and_passed_attributes() {
    fn app() -> Element {
      rsx! {
        FieldLabel { r#for: "email", id: "email-label", "Email" }
        FieldLabel { "Wrapped" }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"for="email""#));
    assert!(html.contains(r#"id="email-label""#));
    // Without `for`, the label must not point at an empty id.
    assert_eq!(html.matches("for=").count(), 1);
  }

  #[test]
  fn field_class_reflects_invalid_state() {
    let actual = field_class(true, "gap-3");

    assert_eq!(
      actual,
      "grid data-[disabled=true]:opacity-50 data-[invalid=true]:text-destructive gap-3"
    );
    assert!(actual.contains(FIELD_INVALID_CLASS));
    assert!(actual.ends_with("gap-3"));
  }

  #[test]
  fn field_group_class_appends_user_class() {
    let actual = field_group_class("max-w-sm");

    assert!(actual.contains(FIELD_GROUP_BASE_CLASS));
    assert!(actual.ends_with("max-w-sm"));
  }
}
