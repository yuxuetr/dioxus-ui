use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::modal_focus::use_modal_focus_scope;

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

/// Click requests `!open` through `on_open_change`. Pass `id` as the content's
/// `anchor_id`.
#[component]
pub fn DatePickerTrigger(
  #[props(default)] id: Option<String>,
  #[props(default)] open: bool,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = date_picker_trigger_class(invalid, &class);

  rsx! {
    button {
      r#type: "button",
      id,
      class,
      disabled,
      onclick: move |_| {
        if let Some(handler) = on_open_change {
          handler.call(!open);
        }
      },
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

/// With `anchor_id` (the trigger's `id`) the content is placed next to the
/// trigger, flipping and shifting to stay in the viewport. Opening moves focus
/// to the element marked `data-dxui-autofocus` (a keyboard-managed Calendar's
/// focused day) or the first focusable element, Tab wraps inside, and closing
/// returns focus to the trigger. Escape and outside interactions request close
/// per `dismiss`.
#[component]
pub fn DatePickerContent(
  #[props(default)] open: bool,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Center)] align: OverlayAlign,
  #[props(default)] anchor_id: Option<String>,
  #[props(default = 4)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = date_picker_content_class(&class);
  let focus_scope = use_modal_focus_scope(open);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, side, align, side_offset },
    dismiss,
    on_open_change,
  );

  rsx! {
    div {
      role: "dialog",
      class,
      tabindex: "-1",
      hidden: !open,
      "data-dxui-anchored": anchored,
      "data-dxui-focus-scope": focus_scope,
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
