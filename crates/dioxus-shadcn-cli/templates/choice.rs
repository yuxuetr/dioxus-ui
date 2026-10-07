//! The value, or values with `multiple`, a Select, Combobox, Accordion, or
//! Toggle Group root owns (RFC 0077). A single choice holds the empty string
//! while nothing is chosen, as in Radix.

use super::root_state::{Controllable, use_controllable};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct Choice {
  pub(crate) multiple: bool,
  single: Controllable<Option<String>>,
  many: Controllable<Vec<String>>,
  /// Takes the value of the option the user chose: it becomes the value, or
  /// with `multiple` toggles in the values.
  pub(crate) choose: Callback<String>,
}

impl Choice {
  /// The chosen values, subscribing the reader to their changes.
  pub(crate) fn chosen(&self) -> Vec<String> {
    if self.multiple {
      self.many.get()
    } else {
      self.single.get().into_iter().filter(|value| !value.is_empty()).collect()
    }
  }

  /// As `choose`, except that pressing the one chosen value of a single
  /// choice clears it, so its root hears the empty string.
  pub(crate) fn toggle(&self, value: String) {
    let clears = !self.multiple && self.single.get().as_deref() == Some(value.as_str());
    self.choose.call(if clears { String::new() } else { value });
  }
}

/// Called by the root; the props are the root's, as Select documents them.
pub(crate) fn use_choice(
  multiple: bool,
  value: ReadSignal<Option<String>>,
  default_value: Option<String>,
  on_value_change: Option<EventHandler<String>>,
  values: ReadSignal<Option<Vec<String>>>,
  default_values: Vec<String>,
  on_values_change: Option<EventHandler<Vec<String>>>,
) -> Choice {
  let single_change = use_callback(move |next: Option<String>| {
    if let (Some(handler), Some(next)) = (on_value_change, next) {
      handler.call(next);
    }
  });
  let single =
    use_controllable(move || value().map(Some), move || default_value, Some(single_change));
  let many = use_controllable(move || values.cloned(), move || default_values, on_values_change);
  let choose = use_callback(move |chosen: String| {
    if multiple {
      let mut next = many.get();
      match next.iter().position(|value| *value == chosen) {
        Some(index) => {
          next.remove(index);
        }
        None => next.push(chosen),
      }
      many.set(next);
    } else {
      single.set(Some(chosen));
    }
  });
  Choice { multiple, single, many, choose }
}
