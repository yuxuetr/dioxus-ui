use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const INPUT_BASE_CLASS: &str = "flex h-10 w-full rounded-md border bg-white px-3 py-2 text-sm transition-colors placeholder:text-zinc-500 focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

pub fn input_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-red-500 focus-visible:ring-red-500"
  } else {
    "border-zinc-200 focus-visible:ring-blue-600"
  };

  classes([Some(INPUT_BASE_CLASS), Some(invalid_class), Some(class)])
}

/// A controlled input. Each `input` event calls `on_value_change` with the new
/// text; the app passes it back as `value`. Other attributes, such as `id`,
/// `name`, and `aria-describedby`, are passed to the input.
#[component]
pub fn Input(
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let class = input_class(invalid, &class);

  rsx! {
    input {
      class,
      value,
      placeholder,
      disabled,
      "aria-invalid": invalid.to_string(),
      oninput: move |event: FormEvent| {
        if let Some(handler) = on_value_change {
          handler.call(event.value());
        }
      },
      ..attributes,
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
