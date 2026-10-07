//! `aria-activedescendant` state for composite widgets such as the styled
//! `Command` and `Combobox` lists, where focus stays on the container.

/// Pure state for composite widgets that keep focus on a container while an
/// item is active through `aria-activedescendant`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ActiveDescendantState {
  /// Id of the active item, or `None` when no item is active.
  pub active_id: Option<String>,
}

impl ActiveDescendantState {
  /// A state with `active_id` active, or none when `None`.
  pub fn new(active_id: Option<String>) -> Self {
    Self { active_id }
  }

  /// A state with the item `active_id` active.
  pub fn active(active_id: impl Into<String>) -> Self {
    Self { active_id: Some(active_id.into()) }
  }

  /// A copy with `active_id` as the active item.
  pub fn set_active(&self, active_id: impl Into<String>) -> Self {
    Self { active_id: Some(active_id.into()) }
  }

  /// A copy with no active item.
  pub const fn clear(&self) -> Self {
    Self { active_id: None }
  }

  /// Attributes for the focused container: focusable, and pointing at the
  /// active item when there is one.
  pub fn container_attributes(&self) -> ActiveDescendantContainerAttributes {
    ActiveDescendantContainerAttributes {
      tabindex: 0,
      aria_activedescendant: self.active_id.clone(),
    }
  }

  /// Attributes for the item `item_id`, marking whether it is the active one.
  pub fn item_attributes(&self, item_id: impl Into<String>) -> ActiveDescendantItemAttributes {
    let item_id = item_id.into();
    let active = self.active_id.as_deref() == Some(item_id.as_str());

    ActiveDescendantItemAttributes { id: item_id, active }
  }
}

/// Attributes the container element renders.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveDescendantContainerAttributes {
  /// Value for `tabindex`; always 0 so the container takes focus.
  pub tabindex: i16,
  /// Value for `aria-activedescendant`; `None` omits the attribute.
  pub aria_activedescendant: Option<String>,
}

/// Attributes an item element renders.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveDescendantItemAttributes {
  /// Value for the item's `id`, which `aria-activedescendant` refers to.
  pub id: String,
  /// Whether this item is the active one, for highlight styling.
  pub active: bool,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn active_state_sets_container_attribute() {
    let state = ActiveDescendantState::active("item-1");
    let attributes = state.container_attributes();

    assert_eq!(attributes.tabindex, 0);
    assert_eq!(attributes.aria_activedescendant.as_deref(), Some("item-1"));
  }

  #[test]
  fn clear_removes_active_descendant() {
    let state = ActiveDescendantState::active("item-1").clear();

    assert_eq!(state.active_id, None);
    assert_eq!(state.container_attributes().aria_activedescendant, None);
  }

  #[test]
  fn item_attributes_reflect_active_state() {
    let state = ActiveDescendantState::active("item-2");
    let active = state.item_attributes("item-2");
    let inactive = state.item_attributes("item-1");

    assert!(active.active);
    assert!(!inactive.active);
    assert_eq!(active.id, "item-2");
  }

  #[test]
  fn set_active_replaces_active_id() {
    let state = ActiveDescendantState::active("item-1").set_active("item-3");

    assert_eq!(state.active_id.as_deref(), Some("item-3"));
  }
}
