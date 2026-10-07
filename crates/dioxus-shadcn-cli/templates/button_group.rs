//! Button Group: a grouped layout for related command buttons, spaced apart or attached.
use super::density::{density_control_class, use_density, with_density};
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// The direction the buttons run.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonGroupOrientation {
  /// Side by side in a row.
  #[default]
  Horizontal,
  /// Stacked in a column.
  Vertical,
}

const BUTTON_GROUP_BASE_CLASS: &str = "inline-flex items-stretch";
const BUTTON_GROUP_GAP_CLASS: &str = "gap-1";
const BUTTON_GROUP_ATTACHED_HORIZONTAL_CLASS: &str = "gap-0 [&>button:not(:first-child)]:rounded-l-none [&>button:not(:first-child)]:border-l-0 [&>button:not(:last-child)]:rounded-r-none";
const BUTTON_GROUP_ATTACHED_VERTICAL_CLASS: &str = "gap-0 [&>button:not(:first-child)]:rounded-t-none [&>button:not(:first-child)]:border-t-0 [&>button:not(:last-child)]:rounded-b-none";
const BUTTON_GROUP_ITEM_BASE_CLASS: &str = "inline-flex items-center justify-center rounded-md border border-input bg-background px-3 py-2 text-sm font-medium transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

impl ButtonGroupOrientation {
  /// The orientation's flex direction class.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Horizontal => "flex-row",
      Self::Vertical => "flex-col",
    }
  }

  /// The value for the group's `data-orientation` attribute.
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

/// Classes for the group: base classes and the orientation's direction; `attached` joins the
/// buttons' borders instead of spacing them, then `class` is merged over them.
pub fn button_group_class(
  orientation: ButtonGroupOrientation,
  attached: bool,
  class: &str,
) -> String {
  let spacing_class = if attached {
    match orientation {
      ButtonGroupOrientation::Horizontal => BUTTON_GROUP_ATTACHED_HORIZONTAL_CLASS,
      ButtonGroupOrientation::Vertical => BUTTON_GROUP_ATTACHED_VERTICAL_CLASS,
    }
  } else {
    BUTTON_GROUP_GAP_CLASS
  };

  merge_classes(classes([Some(BUTTON_GROUP_BASE_CLASS), Some(orientation.class()), Some(spacing_class)]), class)
}

/// Classes for the bordered button inside the group.
pub fn button_group_item_class(class: &str) -> String {
  merge_classes(classes([Some(BUTTON_GROUP_ITEM_BASE_CLASS)]), class)
}

#[component]
pub fn ButtonGroup(
  #[props(default)] orientation: ButtonGroupOrientation,
  #[props(default = true)] attached: bool,
  #[props(default)] aria_label: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = button_group_class(orientation, attached, &class);

  rsx! {
    div {
      role: "group",
      class,
      "aria-label": aria_label,
      "data-orientation": orientation.attribute(),
      "data-attached": attached.to_string(),
      {children}
    }
  }
}

#[component]
pub fn ButtonGroupItem(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = button_group_item_class(&with_density(density_control_class(use_density()), &class));

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
      },
      ..attributes,
      {children}
    }
  }
}
