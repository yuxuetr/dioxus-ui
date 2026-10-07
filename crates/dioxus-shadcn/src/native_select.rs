use crate::density::{use_density, with_density};
use dioxus::prelude::*;
use dioxus_shadcn_core::{UiDensity, classes, merge_classes};

pub const NATIVE_SELECT_BASE_CLASS: &str = "h-10 w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
pub const NATIVE_SELECT_GROUP_BASE_CLASS: &str = "text-sm font-medium text-foreground";
pub const NATIVE_SELECT_OPTION_BASE_CLASS: &str = "text-sm text-foreground";

pub fn native_select_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(NATIVE_SELECT_BASE_CLASS), Some(invalid_class)]), class)
}

pub fn native_select_group_class(class: &str) -> String {
  merge_classes(classes([Some(NATIVE_SELECT_GROUP_BASE_CLASS)]), class)
}

pub fn native_select_option_class(class: &str) -> String {
  merge_classes(classes([Some(NATIVE_SELECT_OPTION_BASE_CLASS)]), class)
}

/// A change calls `on_value_change` with the chosen option's `value`; the app
/// marks that option `selected`. Other attributes, such as `id`, `name`, and
/// `required`, are passed to the select.
#[component]
pub fn NativeSelect(
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(extends = GlobalAttributes, extends = select)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  // WebKit keeps a native select's height over a `min-height`.
  let touch_class = match use_density() {
    UiDensity::Touch => "h-11",
    UiDensity::Compact | UiDensity::Comfortable => "",
  };
  let class = native_select_class(invalid, &with_density(touch_class, &class));

  rsx! {
    select {
      class,
      disabled,
      "aria-invalid": invalid.to_string(),
      onchange: move |event: FormEvent| {
        if let Some(handler) = on_value_change {
          handler.call(event.value());
        }
      },
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn NativeSelectGroup(
  #[props(default)] label: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = native_select_group_class(&class);

  rsx! {
    optgroup {
      class,
      label,
      {children}
    }
  }
}

#[component]
pub fn NativeSelectOption(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] selected: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = native_select_option_class(&class);

  rsx! {
    option {
      class,
      value,
      disabled,
      selected,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_select_class_reflects_invalid_state() {
    let actual = native_select_class(true, "w-48");

    assert_eq!(
      actual,
      "h-10 rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50 border-destructive focus-visible:ring-destructive w-48"
    );
    assert!(actual.contains("border-destructive focus-visible:ring-destructive"));
    assert!(actual.ends_with("w-48"));
  }

  #[test]
  fn native_select_option_class_extends_user_class() {
    let actual = native_select_option_class("font-medium");

    assert!(actual.contains(NATIVE_SELECT_OPTION_BASE_CLASS));
    assert!(actual.ends_with("font-medium"));
  }
}
