use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{OverlayAlign, OverlaySide, PopoverPrimitiveConfig};

pub const DATE_PICKER_TRIGGER_BASE_CLASS: &str = "flex h-10 w-full items-center justify-between rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm text-zinc-950 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:cursor-not-allowed disabled:opacity-50";
pub const DATE_PICKER_VALUE_BASE_CLASS: &str = "truncate text-left data-placeholder:text-zinc-500";
pub const DATE_PICKER_CONTENT_BASE_CLASS: &str = "z-50 w-auto rounded-md border border-zinc-200 bg-white p-0 text-zinc-950 shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";

pub fn date_picker_trigger_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-red-500 focus-visible:ring-red-500"
  } else {
    "focus-visible:ring-blue-600"
  };

  classes([Some(DATE_PICKER_TRIGGER_BASE_CLASS), Some(invalid_class), Some(class)])
}

pub fn date_picker_value_class(class: &str) -> String {
  classes([Some(DATE_PICKER_VALUE_BASE_CLASS), Some(class)])
}

pub fn date_picker_content_class(class: &str) -> String {
  classes([Some(DATE_PICKER_CONTENT_BASE_CLASS), Some(class)])
}

pub fn date_picker_side_attribute(side: OverlaySide) -> &'static str {
  match side {
    OverlaySide::Top => "top",
    OverlaySide::Right => "right",
    OverlaySide::Bottom => "bottom",
    OverlaySide::Left => "left",
    OverlaySide::Inline => "inline",
  }
}

pub fn date_picker_align_attribute(align: OverlayAlign) -> &'static str {
  match align {
    OverlayAlign::Start => "start",
    OverlayAlign::Center => "center",
    OverlayAlign::End => "end",
  }
}

#[component]
pub fn DatePickerTrigger(
  #[props(default)] open: bool,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = date_picker_trigger_class(invalid, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-expanded": open.to_string(),
      "aria-haspopup": "dialog",
      "aria-invalid": invalid.to_string(),
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[component]
pub fn DatePickerValue(
  #[props(default)] placeholder: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = date_picker_value_class(&class);
  let has_placeholder = !placeholder.is_empty();

  rsx! {
    span {
      class,
      "data-placeholder": has_placeholder.to_string(),
      "aria-label": placeholder,
      {children}
    }
  }
}

#[component]
pub fn DatePickerContent(
  #[props(default)] open: bool,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Center)] align: OverlayAlign,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = date_picker_content_class(&class);

  rsx! {
    div {
      role: "dialog",
      class,
      hidden: !open,
      "data-align": date_picker_align_attribute(align),
      "data-side": date_picker_side_attribute(side),
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn date_picker_trigger_class_reflects_invalid_state() {
    let actual = date_picker_trigger_class(true, "w-56");

    assert!(actual.contains(DATE_PICKER_TRIGGER_BASE_CLASS));
    assert!(actual.contains("border-red-500 focus-visible:ring-red-500"));
    assert!(actual.ends_with("w-56"));
  }

  #[test]
  fn date_picker_side_and_align_attributes_map_values() {
    assert_eq!(date_picker_side_attribute(OverlaySide::Left), "left");
    assert_eq!(date_picker_align_attribute(OverlayAlign::End), "end");
  }

  #[test]
  fn date_picker_primitive_config_is_reexported() {
    let config = PopoverPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
