//! Radio groups for Dropdown, Context Menu, and Menubar (RFC 0077).

use dioxus::prelude::*;

use crate::root_state::{Controllable, use_controllable, use_root_context};

/// The value a menu radio group owns, shared with its items.
#[derive(Clone, Copy)]
pub(crate) struct MenuRadioGroup(Controllable<Option<String>>);

impl MenuRadioGroup {
  /// The checked item's value, subscribing the reader to its changes.
  pub(crate) fn value(&self) -> Option<String> {
    self.0.get()
  }

  pub(crate) fn choose(&self, value: String) {
    self.0.set(Some(value));
  }
}

/// Called by a radio group root; returns the group it provides to its items.
pub(crate) fn use_menu_radio_group(
  value: ReadSignal<Option<String>>,
  default_value: Option<String>,
  on_value_change: Option<EventHandler<String>>,
) -> MenuRadioGroup {
  let change = use_callback(move |next: Option<String>| {
    if let (Some(handler), Some(next)) = (on_value_change, next) {
      handler.call(next);
    }
  });
  let group = MenuRadioGroup(use_controllable(
    move || value().map(Some),
    move || default_value,
    Some(change),
  ));
  use_context_provider(|| group)
}

/// The group a radio item `part` belongs to, named `root`.
pub(crate) fn use_menu_radio_item(part: &str, root: &str) -> MenuRadioGroup {
  use_root_context::<MenuRadioGroup>(part, root)
}
