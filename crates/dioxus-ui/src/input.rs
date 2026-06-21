use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const INPUT_BASE_CLASS: &str = "flex h-10 w-full rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm transition-colors placeholder:text-zinc-500 focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

pub fn input_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-red-500 focus-visible:ring-red-500"
  } else {
    "focus-visible:ring-blue-600"
  };

  classes([Some(INPUT_BASE_CLASS), Some(invalid_class), Some(class)])
}

#[component]
pub fn Input(
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
) -> Element {
  let class = input_class(invalid, &class);

  rsx! {
    input {
      class,
      value,
      placeholder,
      disabled,
      "aria-invalid": invalid.to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn input_class_adds_invalid_state() {
    let actual = input_class(true, "w-64");

    assert!(actual.contains(INPUT_BASE_CLASS));
    assert!(actual.contains("border-red-500 focus-visible:ring-red-500"));
    assert!(actual.ends_with("w-64"));
  }
}
