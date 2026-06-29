use dioxus::prelude::*;
use dioxus_ui_core::classes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonGroupOrientation {
  #[default]
  Horizontal,
  Vertical,
}

pub const BUTTON_GROUP_BASE_CLASS: &str = "inline-flex items-stretch";
pub const BUTTON_GROUP_GAP_CLASS: &str = "gap-1";
pub const BUTTON_GROUP_ATTACHED_HORIZONTAL_CLASS: &str = "gap-0 [&>button:not(:first-child)]:rounded-l-none [&>button:not(:first-child)]:border-l-0 [&>button:not(:last-child)]:rounded-r-none";
pub const BUTTON_GROUP_ATTACHED_VERTICAL_CLASS: &str = "gap-0 [&>button:not(:first-child)]:rounded-t-none [&>button:not(:first-child)]:border-t-0 [&>button:not(:last-child)]:rounded-b-none";
pub const BUTTON_GROUP_ITEM_BASE_CLASS: &str = "inline-flex items-center justify-center rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm font-medium transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";

impl ButtonGroupOrientation {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Horizontal => "flex-row",
      Self::Vertical => "flex-col",
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

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

  classes([
    Some(BUTTON_GROUP_BASE_CLASS),
    Some(orientation.class()),
    Some(spacing_class),
    Some(class),
  ])
}

pub fn button_group_item_class(class: &str) -> String {
  classes([Some(BUTTON_GROUP_ITEM_BASE_CLASS), Some(class)])
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
      "aria-orientation": orientation.attribute(),
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
  children: Element,
) -> Element {
  let class = button_group_item_class(&class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn button_group_class_supports_attached_horizontal_layout() {
    let actual = button_group_class(ButtonGroupOrientation::Horizontal, true, "w-full");

    assert!(actual.contains(BUTTON_GROUP_BASE_CLASS));
    assert!(actual.contains("flex-row"));
    assert!(actual.contains(BUTTON_GROUP_ATTACHED_HORIZONTAL_CLASS));
    assert!(actual.ends_with("w-full"));
  }

  #[test]
  fn button_group_class_supports_vertical_gap_layout() {
    let actual = button_group_class(ButtonGroupOrientation::Vertical, false, "items-start");

    assert!(actual.contains("flex-col"));
    assert!(actual.contains(BUTTON_GROUP_GAP_CLASS));
    assert!(actual.ends_with("items-start"));
  }

  #[test]
  fn button_group_item_class_preserves_static_tailwind_tokens() {
    let actual = button_group_item_class("min-w-20");

    assert!(actual.contains(BUTTON_GROUP_ITEM_BASE_CLASS));
    assert!(actual.contains("border-zinc-200"));
    assert!(actual.ends_with("min-w-20"));
    assert!(!actual.contains("{}"));
  }
}
