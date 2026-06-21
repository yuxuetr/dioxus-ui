use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const SWITCH_BASE_CLASS: &str = "inline-flex h-6 w-11 shrink-0 items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:cursor-not-allowed disabled:opacity-50";
pub const SWITCH_THUMB_BASE_CLASS: &str = "pointer-events-none block h-5 w-5 rounded-full bg-white shadow transition-transform";

pub fn switch_class(checked: bool, class: &str) -> String {
  let checked_class = if checked {
    "bg-blue-600"
  } else {
    "bg-zinc-200"
  };

  classes([Some(SWITCH_BASE_CLASS), Some(checked_class), Some(class)])
}

pub fn switch_thumb_class(checked: bool) -> String {
  let checked_class = if checked {
    "translate-x-5"
  } else {
    "translate-x-0"
  };

  classes([Some(SWITCH_THUMB_BASE_CLASS), Some(checked_class)])
}

#[component]
pub fn Switch(
  #[props(default)] checked: bool,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
) -> Element {
  let class = switch_class(checked, &class);
  let thumb_class = switch_thumb_class(checked);

  rsx! {
    button {
      r#type: "button",
      role: "switch",
      class,
      disabled,
      "aria-checked": checked.to_string(),
      span {
        class: thumb_class,
      }
    }
  }
}
