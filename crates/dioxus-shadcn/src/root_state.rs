use dioxus::prelude::*;

/// State a compound component's root owns (RFC 0077): the app's value while
/// it sets one, otherwise the root's own, which starts at the default. Parts
/// read it through the root's context and change it through `set`.
pub(crate) struct Controllable<T: 'static> {
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
  /// The current value, subscribing the reader to its changes. It reads the
  /// app's value and the root's own directly: a memo over the `controlled`
  /// memo would only turn dirty once `controlled` recomputed, so an event
  /// handler reading it right after the app changed its value got the old
  /// one.
  pub(crate) fn get(&self) -> T {
    self.controlled.cloned().unwrap_or_else(|| self.own.cloned())
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
  let mut handler = use_hook(|| CopyValue::new(on_change));
  handler.set(on_change);
  Controllable { controlled, own, on_change: handler }
}

/// The context of the root a part belongs to. A part outside its root
/// panics here, naming both, instead of rendering without its state; Dioxus
/// logs the panic and renders nothing for the part.
#[cfg(any(
  feature = "accordion",
  feature = "alert-dialog",
  feature = "collapsible",
  feature = "combobox",
  feature = "context-menu",
  feature = "date-picker",
  feature = "dialog",
  feature = "drawer",
  feature = "dropdown",
  feature = "fab",
  feature = "hover-card",
  feature = "menubar",
  feature = "navigation-menu",
  feature = "popover",
  feature = "select",
  feature = "sheet",
  feature = "radio-group",
  feature = "tabs",
  feature = "toggle-group",
  feature = "tooltip"
))]
pub(crate) fn use_root_context<T: Clone + 'static>(part: &str, root: &str) -> T {
  root_context_or_panic(try_use_context::<T>(), part, root)
}

#[cfg(any(
  feature = "accordion",
  feature = "alert-dialog",
  feature = "collapsible",
  feature = "combobox",
  feature = "context-menu",
  feature = "date-picker",
  feature = "dialog",
  feature = "drawer",
  feature = "dropdown",
  feature = "fab",
  feature = "hover-card",
  feature = "menubar",
  feature = "navigation-menu",
  feature = "popover",
  feature = "select",
  feature = "sheet",
  feature = "radio-group",
  feature = "tabs",
  feature = "toggle-group",
  feature = "tooltip"
))]
fn root_context_or_panic<T>(context: Option<T>, part: &str, root: &str) -> T {
  context.unwrap_or_else(|| panic!("`{part}` must be inside a `{root}` (RFC 0077)"))
}

#[cfg(test)]
mod tests {
  use super::*;

  thread_local! {
    static SEEN: std::cell::RefCell<Vec<i32>> = const { std::cell::RefCell::new(Vec::new()) };
  }

  #[test]
  fn a_controlled_value_is_current_right_after_the_app_changes_it() {
    fn app() -> Element {
      let mut value = use_signal(|| Some(1));
      let state = use_controllable(move || value.cloned(), || 0, None);
      use_hook(move || {
        let first = state.get();
        // As an event handler would: the app sets its value, and a part reads
        // the state before anything re-renders.
        value.set(Some(2));
        SEEN.with(|seen| seen.borrow_mut().extend([first, state.get()]));
      });
      rsx! {}
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();

    assert_eq!(SEEN.with(|seen| seen.borrow().clone()), [1, 2]);
  }

  #[cfg(any(
    feature = "accordion",
    feature = "alert-dialog",
    feature = "collapsible",
    feature = "combobox",
    feature = "context-menu",
    feature = "date-picker",
    feature = "dialog",
    feature = "drawer",
    feature = "dropdown",
    feature = "fab",
    feature = "hover-card",
    feature = "menubar",
    feature = "navigation-menu",
    feature = "popover",
    feature = "select",
    feature = "sheet",
    feature = "radio-group",
    feature = "tabs",
    feature = "toggle-group",
    feature = "tooltip"
  ))]
  #[test]
  fn a_missing_root_names_the_part_and_the_root() {
    let payload =
      std::panic::catch_unwind(|| root_context_or_panic::<()>(None, "SelectItem", "Select"))
        .expect_err("a missing root should panic");
    let message = payload.downcast_ref::<String>().cloned().unwrap_or_default();

    assert_eq!(message, "`SelectItem` must be inside a `Select` (RFC 0077)");
  }
}
