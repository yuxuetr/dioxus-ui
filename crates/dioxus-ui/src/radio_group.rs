use dioxus::prelude::*;
use dioxus_ui_core::classes;
use dioxus_ui_primitives::{FocusMove, NavigationOrientation, RovingFocusItem, RovingFocusState};

pub const RADIO_GROUP_BASE_CLASS: &str = "grid gap-2";
pub const RADIO_GROUP_ITEM_BASE_CLASS: &str = "inline-flex h-4 w-4 shrink-0 items-center justify-center rounded-full border border-zinc-300 bg-white transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:cursor-not-allowed disabled:opacity-50";
pub const RADIO_GROUP_INDICATOR_BASE_CLASS: &str = "h-2 w-2 rounded-full bg-white";

pub fn radio_group_class(orientation: NavigationOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    NavigationOrientation::Horizontal => "grid-flow-col auto-cols-max items-center",
    NavigationOrientation::Vertical | NavigationOrientation::Both => "grid-flow-row",
  };

  classes([Some(RADIO_GROUP_BASE_CLASS), Some(orientation_class), Some(class)])
}

pub fn radio_group_item_class(checked: bool, class: &str) -> String {
  let checked_class =
    if checked { "border-blue-600 bg-blue-600 text-white" } else { "text-transparent" };

  classes([Some(RADIO_GROUP_ITEM_BASE_CLASS), Some(checked_class), Some(class)])
}

pub fn radio_group_indicator_class(class: &str) -> String {
  classes([Some(RADIO_GROUP_INDICATOR_BASE_CLASS), Some(class)])
}

pub fn radio_group_orientation_attribute(orientation: NavigationOrientation) -> &'static str {
  match orientation {
    NavigationOrientation::Horizontal => "horizontal",
    NavigationOrientation::Vertical | NavigationOrientation::Both => "vertical",
  }
}

pub fn radio_group_item_tabindex(checked: bool, disabled: bool) -> i16 {
  if checked && !disabled { 0 } else { -1 }
}

pub fn radio_group_focus_state(
  orientation: NavigationOrientation,
  looping: bool,
  value: Option<&str>,
) -> RovingFocusState {
  let state = RovingFocusState::new(orientation).with_looping(looping);

  if let Some(value) = value { state.with_active_id(value) } else { state }
}

pub fn radio_group_move_value<'a>(
  value: Option<&str>,
  items: &'a [RovingFocusItem],
  focus_move: FocusMove,
  orientation: NavigationOrientation,
  looping: bool,
) -> Option<&'a str> {
  radio_group_focus_state(orientation, looping, value).move_focus(items, focus_move)
}

#[component]
pub fn RadioGroup(
  #[props(default)] value: Option<String>,
  #[props(default)] orientation: NavigationOrientation,
  #[props(default = true)] looping: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = radio_group_class(orientation, &class);
  let orientation = radio_group_orientation_attribute(orientation);

  rsx! {
    div {
      role: "radiogroup",
      class,
      "aria-orientation": orientation,
      "data-value": value.unwrap_or_default(),
      "data-looping": looping.to_string(),
      {children}
    }
  }
}

#[component]
pub fn RadioGroupItem(
  value: String,
  #[props(default)] checked: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
) -> Element {
  let class = radio_group_item_class(checked, &class);
  let indicator_class = radio_group_indicator_class("");
  let tabindex = radio_group_item_tabindex(checked, disabled).to_string();

  rsx! {
    button {
      r#type: "button",
      role: "radio",
      class,
      disabled,
      tabindex,
      "aria-checked": checked.to_string(),
      "data-value": value,
      span {
        class: indicator_class,
        "aria-hidden": "true",
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn items() -> Vec<RovingFocusItem> {
    vec![
      RovingFocusItem::enabled("sm"),
      RovingFocusItem::disabled("md"),
      RovingFocusItem::enabled("lg"),
    ]
  }

  #[test]
  fn radio_group_item_class_reflects_checked_state() {
    let actual = radio_group_item_class(true, "mt-1");

    assert!(actual.contains(RADIO_GROUP_ITEM_BASE_CLASS));
    assert!(actual.contains("border-blue-600 bg-blue-600 text-white"));
    assert!(actual.ends_with("mt-1"));
  }

  #[test]
  fn radio_group_class_reflects_orientation() {
    let actual = radio_group_class(NavigationOrientation::Horizontal, "gap-3");

    assert!(actual.contains(RADIO_GROUP_BASE_CLASS));
    assert!(actual.contains("grid-flow-col auto-cols-max items-center"));
    assert!(actual.ends_with("gap-3"));
  }

  #[test]
  fn radio_group_move_value_reuses_roving_focus() {
    let items = items();
    let next = radio_group_move_value(
      Some("sm"),
      &items,
      FocusMove::Next,
      NavigationOrientation::Horizontal,
      true,
    );

    assert_eq!(next, Some("lg"));
  }

  #[test]
  fn radio_group_tabindex_marks_checked_enabled_item_focusable() {
    assert_eq!(radio_group_item_tabindex(true, false), 0);
    assert_eq!(radio_group_item_tabindex(false, false), -1);
    assert_eq!(radio_group_item_tabindex(true, true), -1);
  }
}
