/// Pure state for composite widgets that keep focus on a container while an
/// item is active through `aria-activedescendant`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ActiveDescendantState {
  pub active_id: Option<String>,
}

impl ActiveDescendantState {
  pub fn new(active_id: Option<String>) -> Self {
    Self { active_id }
  }

  pub fn active(active_id: impl Into<String>) -> Self {
    Self { active_id: Some(active_id.into()) }
  }

  pub fn set_active(&self, active_id: impl Into<String>) -> Self {
    Self { active_id: Some(active_id.into()) }
  }

  pub const fn clear(&self) -> Self {
    Self { active_id: None }
  }

  pub fn container_attributes(&self) -> ActiveDescendantContainerAttributes {
    ActiveDescendantContainerAttributes {
      tabindex: 0,
      aria_activedescendant: self.active_id.clone(),
    }
  }

  pub fn item_attributes(&self, item_id: impl Into<String>) -> ActiveDescendantItemAttributes {
    let item_id = item_id.into();
    let active = self.active_id.as_deref() == Some(item_id.as_str());

    ActiveDescendantItemAttributes { id: item_id, active }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveDescendantContainerAttributes {
  pub tabindex: i16,
  pub aria_activedescendant: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveDescendantItemAttributes {
  pub id: String,
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
