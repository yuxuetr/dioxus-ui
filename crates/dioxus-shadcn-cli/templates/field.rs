use super::utils::classes;
use dioxus::prelude::*;

pub const FIELD_BASE_CLASS: &str = "grid gap-2 data-[disabled=true]:opacity-50";
pub const FIELD_INVALID_CLASS: &str = "data-[invalid=true]:text-destructive";
pub const FIELD_LABEL_BASE_CLASS: &str = "text-sm font-medium leading-none text-foreground";
pub const FIELD_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
pub const FIELD_ERROR_BASE_CLASS: &str = "text-sm font-medium text-destructive";
pub const FIELD_GROUP_BASE_CLASS: &str = "grid gap-4";

pub fn field_class(invalid: bool, class: &str) -> String {
  classes([Some(FIELD_BASE_CLASS), invalid.then_some(FIELD_INVALID_CLASS), Some(class)])
}

pub fn field_label_class(class: &str) -> String {
  classes([Some(FIELD_LABEL_BASE_CLASS), Some(class)])
}

pub fn field_description_class(class: &str) -> String {
  classes([Some(FIELD_DESCRIPTION_BASE_CLASS), Some(class)])
}

pub fn field_error_class(class: &str) -> String {
  classes([Some(FIELD_ERROR_BASE_CLASS), Some(class)])
}

pub fn field_group_class(class: &str) -> String {
  classes([Some(FIELD_GROUP_BASE_CLASS), Some(class)])
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
