use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

pub const TEXTAREA_BASE_CLASS: &str = "flex min-h-24 w-full rounded-md border bg-background px-3 py-2 text-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

pub fn textarea_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(TEXTAREA_BASE_CLASS), Some(invalid_class)]), class)
}

/// A controlled textarea. Each `input` event calls `on_value_change` with the new
/// text; the app passes it back as `value`. Other attributes, such as `id`,
/// `name`, and `aria-describedby`, are passed to the textarea.
#[component]
pub fn Textarea(
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(extends = GlobalAttributes, extends = textarea)] attributes: Vec<Attribute>,
) -> Element {
  let class = textarea_class(invalid, &class);

  rsx! {
    textarea {
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
