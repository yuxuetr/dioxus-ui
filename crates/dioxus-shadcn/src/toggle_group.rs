//! Toggle group: a set of pressed buttons with single or multiple selection and
//! roving arrow-key focus.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
use dioxus_shadcn_primitives::NavigationOrientation;

use crate::choice::{Choice, use_choice};
use crate::density::{density_control_class, use_density, with_density};
use crate::root_state::use_root_context;
use crate::roving_group::use_roving_group;

/// How many items can be pressed at once.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToggleGroupType {
  /// At most one; pressing an item releases the others.
  #[default]
  Single,
  /// Any number, each toggled on its own.
  Multiple,
}

const TOGGLE_GROUP_BASE_CLASS: &str = "inline-flex gap-1";
const TOGGLE_GROUP_ITEM_BASE_CLASS: &str = "inline-flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

/// Classes for the group: base classes, a column when vertical or a row otherwise,
/// then `class` merged over them.
pub fn toggle_group_class(orientation: NavigationOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    NavigationOrientation::Vertical => "flex-col items-start",
    NavigationOrientation::Horizontal | NavigationOrientation::Both => "flex-row items-center",
  };

  merge_classes(classes([Some(TOGGLE_GROUP_BASE_CLASS), Some(orientation_class)]), class)
}

/// Classes for an item: base classes, the pressed or unpressed colors, then `class`
/// merged over them.
pub fn toggle_group_item_class(pressed: bool, class: &str) -> String {
  let pressed_class =
    if pressed { "bg-accent text-accent-foreground" } else { "bg-transparent hover:bg-accent" };

  merge_classes(classes([Some(TOGGLE_GROUP_ITEM_BASE_CLASS), Some(pressed_class)]), class)
}

fn toggle_group_orientation_attribute(orientation: NavigationOrientation) -> &'static str {
  match orientation {
    NavigationOrientation::Horizontal | NavigationOrientation::Both => "horizontal",
    NavigationOrientation::Vertical => "vertical",
  }
}

#[derive(Clone, Copy)]
struct ToggleGroupContext {
  pressed: Choice,
}

/// Keeps one Tab stop on the item that last had focus, or the first pressed
/// or enabled item. Arrow keys for `orientation` move focus between enabled
/// items without pressing them, wrapping when `looping`, and Home and End jump
/// to the first and last. The group owns which items are pressed (RFC 0077):
/// one, named by `value` (controlled) or `default_value`, the empty string
/// while none is; with `ToggleGroupType::Multiple`, any number, named by
/// `values` or `default_values`. A click presses an item, or releases it when
/// pressed, and the change callback hears it either way.
#[component]
pub fn ToggleGroup(
  #[props(default)] selection_type: ToggleGroupType,
  #[props(default)] orientation: NavigationOrientation,
  #[props(default = true)] looping: bool,
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: String,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] values: ReadSignal<Option<Vec<String>>>,
  #[props(default)] default_values: Vec<String>,
  #[props(default)] on_values_change: Option<EventHandler<Vec<String>>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = toggle_group_class(orientation, &class);
  let pressed = use_choice(
    selection_type == ToggleGroupType::Multiple,
    value,
    Some(default_value),
    on_value_change,
    values,
    default_values,
    on_values_change,
  );
  use_context_provider(|| ToggleGroupContext { pressed });
  let toggle = use_callback(move |value: String| pressed.toggle(value));
  let scope_id = use_roving_group(Some(toggle));
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
      "data-orientation": orientation,
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
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let context = use_root_context::<ToggleGroupContext>("ToggleGroupItem", "ToggleGroup");
  let pressed = context.pressed.chosen().contains(&value);
  let class =
    toggle_group_item_class(pressed, &with_density(density_control_class(use_density()), &class));

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

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  fn styles() -> Element {
    rsx! {
      ToggleGroupItem { value: "bold", "Bold" }
      ToggleGroupItem { value: "italic", "Italic" }
    }
  }

  #[test]
  fn ssr_presses_the_defaults_unless_controlled() {
    fn app() -> Element {
      rsx! {
        ToggleGroup { default_value: "italic", {styles()} }
        ToggleGroup { value: String::new(), default_value: "italic", {styles()} }
        ToggleGroup {
          selection_type: ToggleGroupType::Multiple,
          default_values: vec!["bold".to_string(), "italic".to_string()],
          {styles()}
        }
      }
    }
    let html = render(app);

    assert_eq!(html.matches(r#"aria-pressed="true""#).count(), 3, "{html}");
    assert_eq!(html.matches(r#"aria-pressed="false""#).count(), 3);
  }

  #[test]
  fn an_item_outside_its_group_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        ToggleGroupItem { value: "bold", "Bold" }
        p { "after" }
      }
    }

    assert_eq!(render(app), "<p>before</p><p>after</p>");
  }

  #[test]
  fn ssr_group_has_no_aria_orientation() {
    fn app() -> Element {
      rsx! { ToggleGroup { orientation: NavigationOrientation::Vertical, "aria-label": "Text style" } }
    }
    let html = render(app);

    assert!(!html.contains("aria-orientation"));
    assert!(html.contains(r#"data-orientation="vertical""#));
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

    assert_eq!(actual, "inline-flex flex-col items-start gap-2");
    assert!(actual.contains("flex-col items-start"));
    assert!(actual.ends_with("gap-2"));
  }
}
