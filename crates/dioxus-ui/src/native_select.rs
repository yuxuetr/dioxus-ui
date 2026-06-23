use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const NATIVE_SELECT_BASE_CLASS: &str = "h-10 w-full rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm text-zinc-950 transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
pub const NATIVE_SELECT_GROUP_BASE_CLASS: &str = "text-sm font-medium text-zinc-900";
pub const NATIVE_SELECT_OPTION_BASE_CLASS: &str = "text-sm text-zinc-950";

pub fn native_select_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-red-500 focus-visible:ring-red-500"
  } else {
    "focus-visible:ring-blue-600"
  };

  classes([Some(NATIVE_SELECT_BASE_CLASS), Some(invalid_class), Some(class)])
}

pub fn native_select_group_class(class: &str) -> String {
  classes([Some(NATIVE_SELECT_GROUP_BASE_CLASS), Some(class)])
}

pub fn native_select_option_class(class: &str) -> String {
  classes([Some(NATIVE_SELECT_OPTION_BASE_CLASS), Some(class)])
}

#[component]
pub fn NativeSelect(
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = native_select_class(invalid, &class);

  rsx! {
    select {
      class,
      disabled,
      "aria-invalid": invalid.to_string(),
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

    assert!(actual.contains(NATIVE_SELECT_BASE_CLASS));
    assert!(actual.contains("border-red-500 focus-visible:ring-red-500"));
    assert!(actual.ends_with("w-48"));
  }

  #[test]
  fn native_select_option_class_extends_user_class() {
    let actual = native_select_option_class("font-medium");

    assert!(actual.contains(NATIVE_SELECT_OPTION_BASE_CLASS));
    assert!(actual.ends_with("font-medium"));
  }
}
