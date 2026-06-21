use dioxus::prelude::*;

#[component]
pub fn Input(
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
) -> Element {
  let base = "flex h-10 w-full rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm transition-colors placeholder:text-zinc-500 focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
  let invalid_class = if invalid {
    "border-red-500 focus-visible:ring-red-500"
  } else {
    "focus-visible:ring-blue-600"
  };
  let class = format!("{base} {invalid_class} {class}");

  rsx! {
    input {
      class,
      value,
      placeholder,
      disabled,
      "aria-invalid": "{invalid}",
    }
  }
}
