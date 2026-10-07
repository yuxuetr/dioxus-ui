//! Textarea: a styled multi-line text field.

use crate::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

const TEXTAREA_BASE_CLASS: &str = "flex min-h-24 w-full rounded-md border bg-background px-3 py-2 text-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

/// Classes for the textarea: base classes, the destructive border and ring when
/// `invalid` or the plain ones otherwise, then `class` merged over them.
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
  let class = textarea_class(invalid, &with_density(density_control_class(use_density()), &class));

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn textarea_class_adds_invalid_state() {
    let actual = textarea_class(true, "resize-none");

    assert!(actual.contains(TEXTAREA_BASE_CLASS));
    assert!(actual.contains("border-destructive focus-visible:ring-destructive"));
    assert!(actual.ends_with("resize-none"));
  }
}
