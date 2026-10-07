use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
use dioxus_shadcn_primitives::{
  FocusMove, NavigationOrientation, RovingFocusItem, RovingFocusState,
};

use crate::root_state::{Controllable, use_controllable, use_root_context};
use crate::roving_group::use_roving_group;

pub const RADIO_GROUP_BASE_CLASS: &str = "grid gap-2";
pub const RADIO_GROUP_ITEM_BASE_CLASS: &str = "inline-flex h-4 w-4 shrink-0 items-center justify-center rounded-full border transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
pub const RADIO_GROUP_INDICATOR_BASE_CLASS: &str = "h-2 w-2 rounded-full bg-background";

pub fn radio_group_class(orientation: NavigationOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    NavigationOrientation::Horizontal => "grid-flow-col auto-cols-max items-center",
    NavigationOrientation::Vertical | NavigationOrientation::Both => "grid-flow-row",
  };

  merge_classes(classes([Some(RADIO_GROUP_BASE_CLASS), Some(orientation_class)]), class)
}

pub fn radio_group_item_class(checked: bool, class: &str) -> String {
  let checked_class = if checked {
    "border-primary bg-primary text-primary-foreground"
  } else {
    "border-input bg-background text-transparent"
  };

  merge_classes(classes([Some(RADIO_GROUP_ITEM_BASE_CLASS), Some(checked_class)]), class)
}

pub fn radio_group_indicator_class(class: &str) -> String {
  merge_classes(classes([Some(RADIO_GROUP_INDICATOR_BASE_CLASS)]), class)
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

#[derive(Clone, Copy)]
struct RadioGroupContext {
  value: Controllable<Option<String>>,
}

/// The root of a radio group: it owns the checked value (RFC 0077). Pass
/// `value` to control it, or `default_value` to start it; `on_value_change`
/// hears every change the user makes either way. Keeps one Tab stop on the
/// checked item, or the first enabled item when none is checked. Arrow keys
/// for `orientation` move between enabled items, wrapping when `looping`, and
/// Home and End jump to the first and last. A click, or an arrow, Home, or
/// End key that moves focus to another item, checks that item.
#[component]
pub fn RadioGroup(
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: Option<String>,
  #[props(default)] orientation: NavigationOrientation,
  #[props(default = true)] looping: bool,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = radio_group_class(orientation, &class);
  let change = use_callback(move |next: Option<String>| {
    if let (Some(handler), Some(next)) = (on_value_change, next) {
      handler.call(next);
    }
  });
  let value = use_controllable(move || value().map(Some), move || default_value, Some(change));
  use_context_provider(|| RadioGroupContext { value });
  let check = use_callback(move |next: String| value.set(Some(next)));
  let scope_id = use_roving_group(Some(check));
  let roving_orientation = match orientation {
    NavigationOrientation::Horizontal => "horizontal",
    NavigationOrientation::Vertical => "vertical",
    NavigationOrientation::Both => "both",
  };
  let orientation = radio_group_orientation_attribute(orientation);

  rsx! {
    div {
      role: "radiogroup",
      class,
      "aria-orientation": orientation,
      "data-value": value.get().unwrap_or_default(),
      "data-looping": looping.to_string(),
      "data-dxui-roving-group": scope_id,
      "data-dxui-roving-orientation": roving_orientation,
      "data-dxui-roving-loop": looping.to_string(),
      "data-dxui-roving-activation": "focus",
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn RadioGroupItem(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
  let context = use_root_context::<RadioGroupContext>("RadioGroupItem", "RadioGroup");
  let checked = context.value.get().as_deref() == Some(value.as_str());
  let class = radio_group_item_class(checked, &class);
  let indicator_class = radio_group_indicator_class("");

  rsx! {
    button {
      r#type: "button",
      role: "radio",
      class,
      disabled,
      "aria-checked": checked.to_string(),
      "data-value": value,
      "data-dxui-roving-item": "",
      ..attributes,
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

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  fn sizes() -> Element {
    rsx! {
      RadioGroupItem { value: "sm", "aria-label": "Small" }
      RadioGroupItem { value: "lg", "aria-label": "Large" }
    }
  }

  #[test]
  fn ssr_checks_the_default_unless_controlled() {
    fn app() -> Element {
      rsx! {
        RadioGroup { default_value: "lg", {sizes()} }
        RadioGroup { value: "sm", default_value: "lg", {sizes()} }
      }
    }
    let html = render(app);

    assert_eq!(html.matches(r#"aria-checked="true""#).count(), 2, "{html}");
    let checked: Vec<_> = html
      .split(r#"aria-checked="true" data-value=""#)
      .skip(1)
      .filter_map(|rest| rest.split('"').next())
      .collect();
    assert_eq!(checked, ["lg", "sm"]);
  }

  #[test]
  fn an_item_outside_its_group_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        RadioGroupItem { value: "sm" }
        p { "after" }
      }
    }

    assert_eq!(render(app), "<p>before</p><p>after</p>");
  }

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
    assert!(actual.contains("border-primary bg-primary text-primary-foreground"));
    assert!(actual.ends_with("mt-1"));
  }

  #[test]
  fn radio_group_class_reflects_orientation() {
    let actual = radio_group_class(NavigationOrientation::Horizontal, "gap-3");

    assert_eq!(actual, "grid grid-flow-col auto-cols-max items-center gap-3");
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
