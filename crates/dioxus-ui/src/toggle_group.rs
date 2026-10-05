use dioxus::prelude::*;
use dioxus_ui_core::classes;
use dioxus_ui_primitives::{FocusMove, NavigationOrientation, RovingFocusItem, RovingFocusState};

use crate::roving_group::use_roving_group;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToggleGroupType {
  #[default]
  Single,
  Multiple,
}

pub const TOGGLE_GROUP_BASE_CLASS: &str = "inline-flex gap-1";
pub const TOGGLE_GROUP_ITEM_BASE_CLASS: &str = "inline-flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

pub fn toggle_group_class(orientation: NavigationOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    NavigationOrientation::Vertical => "flex-col items-start",
    NavigationOrientation::Horizontal | NavigationOrientation::Both => "flex-row items-center",
  };

  classes([Some(TOGGLE_GROUP_BASE_CLASS), Some(orientation_class), Some(class)])
}

pub fn toggle_group_item_class(pressed: bool, class: &str) -> String {
  let pressed_class =
    if pressed { "bg-accent text-accent-foreground" } else { "bg-transparent hover:bg-accent" };

  classes([Some(TOGGLE_GROUP_ITEM_BASE_CLASS), Some(pressed_class), Some(class)])
}

pub fn toggle_group_orientation_attribute(orientation: NavigationOrientation) -> &'static str {
  match orientation {
    NavigationOrientation::Horizontal | NavigationOrientation::Both => "horizontal",
    NavigationOrientation::Vertical => "vertical",
  }
}

pub fn toggle_group_item_tabindex(pressed: bool, disabled: bool) -> i16 {
  if pressed && !disabled { 0 } else { -1 }
}

pub fn toggle_group_focus_state(
  orientation: NavigationOrientation,
  looping: bool,
  active_value: Option<&str>,
) -> RovingFocusState {
  let state = RovingFocusState::new(orientation).with_looping(looping);

  if let Some(active_value) = active_value { state.with_active_id(active_value) } else { state }
}

pub fn toggle_group_move_value<'a>(
  active_value: Option<&str>,
  items: &'a [RovingFocusItem],
  focus_move: FocusMove,
  orientation: NavigationOrientation,
  looping: bool,
) -> Option<&'a str> {
  toggle_group_focus_state(orientation, looping, active_value).move_focus(items, focus_move)
}

pub fn toggle_group_single_selection(current: Option<&str>, toggled_value: &str) -> Option<String> {
  if current == Some(toggled_value) { None } else { Some(toggled_value.to_string()) }
}

pub fn toggle_group_multiple_selection(current: &[String], toggled_value: &str) -> Vec<String> {
  let mut next = current.to_vec();

  if let Some(index) = next.iter().position(|value| value == toggled_value) {
    next.remove(index);
  } else {
    next.push(toggled_value.to_string());
  }

  next
}

/// Keeps one Tab stop on the item that last had focus, or the first pressed
/// or enabled item. Arrow keys for `orientation` move focus between enabled
/// items without pressing them, wrapping when `looping`, and Home and End jump
/// to the first and last. A click calls `on_toggle` with the item's `value`;
/// `toggle_group_single_selection` and `toggle_group_multiple_selection`
/// compute the next selection from it.
#[component]
pub fn ToggleGroup(
  #[props(default)] selection_type: ToggleGroupType,
  #[props(default)] orientation: NavigationOrientation,
  #[props(default = true)] looping: bool,
  #[props(default)] on_toggle: Option<EventHandler<String>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = toggle_group_class(orientation, &class);
  let scope_id = use_roving_group(on_toggle);
  let roving_orientation = match orientation {
    NavigationOrientation::Horizontal => "horizontal",
    NavigationOrientation::Vertical => "vertical",
    NavigationOrientation::Both => "both",
  };
  let orientation = toggle_group_orientation_attribute(orientation);
  let selection_type = match selection_type {
    ToggleGroupType::Single => "single",
    ToggleGroupType::Multiple => "multiple",
  };

  rsx! {
    div {
      role: "group",
      class,
      "aria-orientation": orientation,
      "data-type": selection_type,
      "data-looping": looping.to_string(),
      "data-dxui-roving-group": scope_id,
      "data-dxui-roving-orientation": roving_orientation,
      "data-dxui-roving-loop": looping.to_string(),
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn ToggleGroupItem(
  value: String,
  #[props(default)] pressed: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = toggle_group_item_class(pressed, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-pressed": pressed.to_string(),
      "data-value": value,
      "data-dxui-roving-item": "",
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn items() -> Vec<RovingFocusItem> {
    vec![
      RovingFocusItem::enabled("bold"),
      RovingFocusItem::disabled("italic"),
      RovingFocusItem::enabled("underline"),
    ]
  }

  #[test]
  fn toggle_group_item_class_reflects_pressed_state() {
    let actual = toggle_group_item_class(true, "min-w-10");

    assert!(actual.contains(TOGGLE_GROUP_ITEM_BASE_CLASS));
    assert!(actual.contains("bg-accent text-accent-foreground"));
    assert!(actual.ends_with("min-w-10"));
  }

  #[test]
  fn toggle_group_class_reflects_orientation() {
    let actual = toggle_group_class(NavigationOrientation::Vertical, "gap-2");

    assert!(actual.contains(TOGGLE_GROUP_BASE_CLASS));
    assert!(actual.contains("flex-col items-start"));
    assert!(actual.ends_with("gap-2"));
  }

  #[test]
  fn toggle_group_move_value_reuses_roving_focus() {
    let items = items();
    let next = toggle_group_move_value(
      Some("bold"),
      &items,
      FocusMove::Next,
      NavigationOrientation::Horizontal,
      true,
    );

    assert_eq!(next, Some("underline"));
  }

  #[test]
  fn single_selection_toggles_current_value() {
    assert_eq!(toggle_group_single_selection(None, "bold").as_deref(), Some("bold"));
    assert_eq!(toggle_group_single_selection(Some("bold"), "bold"), None);
    assert_eq!(
      toggle_group_single_selection(Some("bold"), "underline").as_deref(),
      Some("underline")
    );
  }

  #[test]
  fn multiple_selection_toggles_membership() {
    let current = vec!["bold".to_string(), "italic".to_string()];

    assert_eq!(toggle_group_multiple_selection(&current, "italic"), vec!["bold".to_string()]);
    assert_eq!(
      toggle_group_multiple_selection(&current, "underline"),
      vec!["bold".to_string(), "italic".to_string(), "underline".to_string()]
    );
  }
}
