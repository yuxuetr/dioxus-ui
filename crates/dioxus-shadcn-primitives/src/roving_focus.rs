//! Roving focus: which item of a composite widget holds the single tab stop, and
//! where arrow keys move it. The styled `RadioGroup` builds on it.

/// Axis used for arrow-key navigation in composite widgets.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NavigationOrientation {
  /// Left and Right arrows move focus.
  Horizontal,
  /// Up and Down arrows move focus.
  Vertical,
  /// All four arrow keys move focus.
  #[default]
  Both,
}

/// Direction for moving active focus within an ordered collection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FocusMove {
  /// The next enabled item after the active one.
  Next,
  /// The previous enabled item before the active one.
  Previous,
  /// The first enabled item.
  First,
  /// The last enabled item.
  Last,
}

/// Pure state for roving focus.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RovingFocusState {
  /// Id of the item holding focus; `None` before any item is active.
  pub active_id: Option<String>,
  /// Arrow keys the widget responds to. The caller maps keys to
  /// moves; `move_focus` does not read it.
  pub orientation: NavigationOrientation,
  /// Whether moving past either end wraps to the other end.
  pub looping: bool,
}

impl RovingFocusState {
  /// State with no active item that loops at the ends.
  pub fn new(orientation: NavigationOrientation) -> Self {
    Self { active_id: None, orientation, looping: true }
  }

  /// Sets the active item.
  pub fn with_active_id(mut self, active_id: impl Into<String>) -> Self {
    self.active_id = Some(active_id.into());
    self
  }

  /// Sets whether moves wrap at the ends.
  pub const fn with_looping(mut self, looping: bool) -> Self {
    self.looping = looping;
    self
  }

  /// The id of the item `focus_move` lands on, skipping disabled items.
  /// With no active item, `Next` starts from the first item and `Previous` from the last.
  /// `None` when `items` has no enabled item, or when looping is off and the move
  /// would go past an end.
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

  /// A copy of the state after `focus_move`. Keeps the current active item when the
  /// move lands nowhere.
  pub fn moved(&self, items: &[RovingFocusItem], focus_move: FocusMove) -> Self {
    let active_id = self
      .move_focus(items, focus_move)
      .map(ToString::to_string)
      .or_else(|| self.active_id.clone());

    Self { active_id, orientation: self.orientation, looping: self.looping }
  }
}

impl Default for RovingFocusState {
  fn default() -> Self {
    Self::new(NavigationOrientation::default())
  }
}

/// Item metadata used by roving focus calculations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RovingFocusItem {
  /// Id matched against `RovingFocusState::active_id`.
  pub id: String,
  /// Whether moves skip this item.
  pub disabled: bool,
}

impl RovingFocusItem {
  /// An item that can take focus.
  pub fn enabled(id: impl Into<String>) -> Self {
    Self { id: id.into(), disabled: false }
  }

  /// An item that moves skip.
  pub fn disabled(id: impl Into<String>) -> Self {
    Self { id: id.into(), disabled: true }
  }
}

fn first_enabled(items: &[RovingFocusItem]) -> Option<&str> {
  items.iter().find(|item| !item.disabled).map(|item| item.id.as_str())
}

fn last_enabled(items: &[RovingFocusItem]) -> Option<&str> {
  items.iter().rev().find(|item| !item.disabled).map(|item| item.id.as_str())
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

#[cfg(test)]
mod tests {
  use super::*;

  fn items() -> Vec<RovingFocusItem> {
    vec![
      RovingFocusItem::enabled("one"),
      RovingFocusItem::disabled("two"),
      RovingFocusItem::enabled("three"),
    ]
  }

  #[test]
  fn moves_to_next_enabled_item() {
    let state = RovingFocusState::new(NavigationOrientation::Both).with_active_id("one");

    assert_eq!(state.move_focus(&items(), FocusMove::Next), Some("three"));
  }

  #[test]
  fn moves_to_previous_enabled_item() {
    let state = RovingFocusState::new(NavigationOrientation::Both).with_active_id("three");

    assert_eq!(state.move_focus(&items(), FocusMove::Previous), Some("one"));
  }

  #[test]
  fn moves_to_first_and_last_enabled_items() {
    let state = RovingFocusState::default();

    assert_eq!(state.move_focus(&items(), FocusMove::First), Some("one"));
    assert_eq!(state.move_focus(&items(), FocusMove::Last), Some("three"));
  }

  #[test]
  fn loops_at_boundaries_when_enabled() {
    let state = RovingFocusState::default().with_active_id("three");

    assert_eq!(state.move_focus(&items(), FocusMove::Next), Some("one"));
  }

  #[test]
  fn stops_at_boundaries_when_looping_disabled() {
    let state = RovingFocusState::default().with_active_id("three").with_looping(false);

    assert_eq!(state.move_focus(&items(), FocusMove::Next), None);
  }

  #[test]
  fn starts_from_first_enabled_item_when_active_missing() {
    let state = RovingFocusState::default();

    assert_eq!(state.move_focus(&items(), FocusMove::Next), Some("one"));
  }

  #[test]
  fn preserves_active_id_when_no_move_is_available() {
    let state = RovingFocusState::default().with_active_id("three").with_looping(false);
    let next = state.moved(&items(), FocusMove::Next);

    assert_eq!(next.active_id.as_deref(), Some("three"));
  }
}
