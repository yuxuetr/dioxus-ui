use dioxus::prelude::*;
use super::utils::classes;

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

pub const RADIO_GROUP_BASE_CLASS: &str = "grid gap-2";
pub const RADIO_GROUP_ITEM_BASE_CLASS: &str = "inline-flex h-4 w-4 shrink-0 items-center justify-center rounded-full border border-zinc-300 bg-white transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:cursor-not-allowed disabled:opacity-50";
pub const RADIO_GROUP_INDICATOR_BASE_CLASS: &str = "h-2 w-2 rounded-full bg-white";

pub fn radio_group_class(orientation: NavigationOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    NavigationOrientation::Horizontal => "grid-flow-col auto-cols-max items-center",
    NavigationOrientation::Vertical | NavigationOrientation::Both => "grid-flow-row",
  };

  classes([
    Some(RADIO_GROUP_BASE_CLASS),
    Some(orientation_class),
    Some(class),
  ])
}

pub fn radio_group_item_class(checked: bool, class: &str) -> String {
  let checked_class = if checked {
    "border-blue-600 bg-blue-600 text-white"
  } else {
    "text-transparent"
  };

  classes([
    Some(RADIO_GROUP_ITEM_BASE_CLASS),
    Some(checked_class),
    Some(class),
  ])
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
  if checked && !disabled {
    0
  } else {
    -1
  }
}

pub fn radio_group_focus_state(
  orientation: NavigationOrientation,
  looping: bool,
  value: Option<&str>,
) -> RovingFocusState {
  let state = RovingFocusState::new(orientation).with_looping(looping);

  if let Some(value) = value {
    state.with_active_id(value)
  } else {
    state
  }
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
