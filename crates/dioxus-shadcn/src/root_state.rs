use dioxus::prelude::*;

/// State a compound component's root owns (RFC 0077): the app's value while
/// it sets one, otherwise the root's own, which starts at the default. Parts
/// read it through the root's context and change it through `set`.
pub(crate) struct Controllable<T: 'static> {
  current: Memo<T>,
  controlled: Memo<Option<T>>,
  own: Signal<T>,
  on_change: CopyValue<Option<Callback<T>>>,
}

impl<T: 'static> Clone for Controllable<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T: 'static> Copy for Controllable<T> {}

impl<T: Clone + PartialEq + 'static> Controllable<T> {
  /// The current value, subscribing the reader to its changes.
  pub(crate) fn get(&self) -> T {
    (self.current)()
  }

  /// Takes a change the user made: the root keeps it unless the app controls
  /// the value, and the app hears of it either way.
  pub(crate) fn set(&self, next: T) {
    if self.controlled.peek().is_none() {
      let mut own = self.own;
      own.set(next.clone());
    }
    if let Some(on_change) = *self.on_change.peek() {
      on_change.call(next);
    }
  }
}

/// `controlled` reads the app's value, `None` while the app leaves the state
/// to the root; `on_change` is the app's change callback, kept current across
/// renders.
pub(crate) fn use_controllable<T: Clone + PartialEq + 'static>(
  controlled: impl Fn() -> Option<T> + 'static,
  default: impl FnOnce() -> T,
  on_change: Option<Callback<T>>,
) -> Controllable<T> {
  let own = use_signal(default);
  let controlled = use_memo(controlled);
  let current = use_memo(move || controlled().unwrap_or_else(|| own.cloned()));
  let mut handler = use_hook(|| CopyValue::new(on_change));
  handler.set(on_change);
  Controllable { current, controlled, own, on_change: handler }
}

/// The context of the root a part belongs to. A part outside its root
/// panics here, naming both, instead of rendering without its state; Dioxus
/// logs the panic and renders nothing for the part.
pub(crate) fn use_root_context<T: Clone + 'static>(part: &str, root: &str) -> T {
  root_context_or_panic(try_use_context::<T>(), part, root)
}

fn root_context_or_panic<T>(context: Option<T>, part: &str, root: &str) -> T {
  context.unwrap_or_else(|| panic!("`{part}` must be inside a `{root}` (RFC 0077)"))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_missing_root_names_the_part_and_the_root() {
    let payload =
      std::panic::catch_unwind(|| root_context_or_panic::<()>(None, "SelectItem", "Select"))
        .expect_err("a missing root should panic");
    let message = payload.downcast_ref::<String>().cloned().unwrap_or_default();

    assert_eq!(message, "`SelectItem` must be inside a `Select` (RFC 0077)");
  }
}
