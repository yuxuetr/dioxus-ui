use dioxus::prelude::*;
use super::utils::classes;

pub const CHECKBOX_BASE_CLASS: &str = "inline-flex h-4 w-4 shrink-0 items-center justify-center rounded border border-zinc-300 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:cursor-not-allowed disabled:opacity-50";

pub fn checkbox_class(checked: bool, class: &str) -> String {
  let checked_class = if checked {
    "border-blue-600 bg-blue-600 text-white"
  } else {
    "bg-white text-transparent"
  };

  classes([Some(CHECKBOX_BASE_CLASS), Some(checked_class), Some(class)])
}

#[component]
pub fn Checkbox(
  #[props(default)] checked: bool,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
) -> Element {
  let class = checkbox_class(checked, &class);

  rsx! {
    input {
      r#type: "checkbox",
      class,
      checked,
      disabled,
    }
  }
}
