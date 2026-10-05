use dioxus::prelude::*;
use super::utils::{classes, use_roving_group};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NavigationOrientation {
  Horizontal,
  Vertical,
  #[default]
  Both,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FocusMove {
  Next,
  Previous,
  First,
  Last,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RovingFocusItem {
  pub id: String,
  pub disabled: bool,
}

impl RovingFocusItem {
  pub fn enabled(id: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      disabled: false,
    }
  }

  pub fn disabled(id: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      disabled: true,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RovingFocusState {
  pub active_id: Option<String>,
  pub orientation: NavigationOrientation,
  pub looping: bool,
}

impl RovingFocusState {
  pub fn new(orientation: NavigationOrientation) -> Self {
    Self {
      active_id: None,
      orientation,
      looping: true,
    }
  }

  pub fn with_active_id(mut self, active_id: impl Into<String>) -> Self {
    self.active_id = Some(active_id.into());
    self
  }

  pub const fn with_looping(mut self, looping: bool) -> Self {
    self.looping = looping;
    self
  }

  pub fn move_focus<'a>(
    &self,
    items: &'a [RovingFocusItem],
    focus_move: FocusMove,
  ) -> Option<&'a str> {
    match focus_move {
      FocusMove::First => first_enabled(items),
      FocusMove::Last => last_enabled(items),
      FocusMove::Next => move_by(items, self.active_id.as_deref(), 1, self.looping),
      FocusMove::Previous => move_by(items, self.active_id.as_deref(), -1, self.looping),
    }
  }
}

fn first_enabled(items: &[RovingFocusItem]) -> Option<&str> {
  items
    .iter()
    .find(|item| !item.disabled)
    .map(|item| item.id.as_str())
}

fn last_enabled(items: &[RovingFocusItem]) -> Option<&str> {
  items
    .iter()
    .rev()
    .find(|item| !item.disabled)
    .map(|item| item.id.as_str())
}

fn move_by<'a>(
  items: &'a [RovingFocusItem],
  active_id: Option<&str>,
  step: isize,
  looping: bool,
) -> Option<&'a str> {
  if items.is_empty() {
    return None;
  }

  let start = active_id
    .and_then(|id| items.iter().position(|item| item.id == id))
    .unwrap_or_else(|| if step > 0 { 0 } else { items.len().saturating_sub(1) });

  if active_id.is_none() && !items[start].disabled {
    return Some(items[start].id.as_str());
  }

  let mut index = start as isize;

  for _ in 0..items.len() {
    index += step;

    if looping {
      index = index.rem_euclid(items.len() as isize);
    } else if index < 0 || index >= items.len() as isize {
      return None;
    }

    let item = &items[index as usize];

    if !item.disabled {
      return Some(item.id.as_str());
    }
  }

  None
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToggleGroupType {
  #[default]
  Single,
  Multiple,
}

pub const TOGGLE_GROUP_BASE_CLASS: &str = "inline-flex gap-1";
pub const TOGGLE_GROUP_ITEM_BASE_CLASS: &str = "inline-flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";

pub fn toggle_group_class(orientation: NavigationOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    NavigationOrientation::Vertical => "flex-col items-start",
    NavigationOrientation::Horizontal | NavigationOrientation::Both => "flex-row items-center",
  };

  classes([
    Some(TOGGLE_GROUP_BASE_CLASS),
    Some(orientation_class),
    Some(class),
  ])
}

pub fn toggle_group_item_class(pressed: bool, class: &str) -> String {
  let pressed_class = if pressed {
    "bg-zinc-900 text-white hover:bg-zinc-800"
  } else {
    "bg-transparent hover:bg-zinc-100"
  };

  classes([
    Some(TOGGLE_GROUP_ITEM_BASE_CLASS),
    Some(pressed_class),
    Some(class),
  ])
}

pub fn toggle_group_orientation_attribute(orientation: NavigationOrientation) -> &'static str {
  match orientation {
    NavigationOrientation::Horizontal | NavigationOrientation::Both => "horizontal",
    NavigationOrientation::Vertical => "vertical",
  }
}

pub fn toggle_group_item_tabindex(pressed: bool, disabled: bool) -> i16 {
  if pressed && !disabled {
    0
  } else {
    -1
  }
}

pub fn toggle_group_focus_state(
  orientation: NavigationOrientation,
  looping: bool,
  active_value: Option<&str>,
) -> RovingFocusState {
  let state = RovingFocusState::new(orientation).with_looping(looping);

  if let Some(active_value) = active_value {
    state.with_active_id(active_value)
  } else {
    state
  }
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
  if current == Some(toggled_value) {
    None
  } else {
    Some(toggled_value.to_string())
  }
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
