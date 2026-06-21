use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const TEXTAREA_BASE_CLASS: &str = "flex min-h-24 w-full rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm transition-colors placeholder:text-zinc-500 focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

pub fn textarea_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-red-500 focus-visible:ring-red-500"
  } else {
    "focus-visible:ring-blue-600"
  };

  classes([Some(TEXTAREA_BASE_CLASS), Some(invalid_class), Some(class)])
}

#[component]
pub fn Textarea(
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
) -> Element {
  let class = textarea_class(invalid, &class);

  rsx! {
    textarea {
      class,
      value,
      placeholder,
      disabled,
      "aria-invalid": invalid.to_string(),
    }
  }
}
