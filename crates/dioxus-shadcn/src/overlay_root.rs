use dioxus::prelude::*;

#[cfg(any(
  feature = "alert-dialog",
  feature = "dialog",
  feature = "drawer",
  feature = "popover",
  feature = "sheet"
))]
use crate::default_attribute::default_attribute;
use crate::element_id::next_element_id;
use crate::root_state::{Controllable, use_controllable};

/// What an overlay root shares with its parts (RFC 0077): whether it is open,
/// and the ids that link its trigger and content.
#[derive(Clone, Copy)]
pub(crate) struct OverlayRoot {
  open: Controllable<bool>,
  /// Takes a change the user made; it stays the same across renders, so
  /// hooks that take an `on_open_change` can hold it.
  pub(crate) set_open: Callback<bool>,
  kind: &'static str,
  id: usize,
}

impl OverlayRoot {
  /// Whether the overlay is open, subscribing the reader to its changes.
  pub(crate) fn is_open(&self) -> bool {
    self.open.get()
  }

  pub(crate) fn trigger_id(&self) -> String {
    format!("dxui-{}-{}-trigger", self.kind, self.id)
  }

  pub(crate) fn content_id(&self) -> String {
    format!("dxui-{}-{}-content", self.kind, self.id)
  }
}

/// Called by an overlay root; `kind` names the overlay in its part ids.
pub(crate) fn use_overlay_root(
  kind: &'static str,
  open: ReadSignal<Option<bool>>,
  default_open: bool,
  on_open_change: Option<EventHandler<bool>>,
) -> OverlayRoot {
  let open = use_controllable(move || open.cloned(), move || default_open, on_open_change);
  let set_open = use_callback(move |next: bool| open.set(next));
  let id = use_hook(next_element_id);
  OverlayRoot { open, set_open, kind, id }
}

/// The trigger of an overlay that opens on press: a button that toggles the root and
/// points at the content. It has no styles of its own; the app passes them in
/// `class`, as with `asChild` in shadcn/ui.
#[cfg(any(
  feature = "alert-dialog",
  feature = "dialog",
  feature = "drawer",
  feature = "popover",
  feature = "sheet"
))]
pub(crate) fn overlay_trigger(
  root: OverlayRoot,
  haspopup: &'static str,
  class: String,
  disabled: bool,
  attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let open = root.is_open();
  let id = default_attribute(&attributes, "id", root.trigger_id());

  rsx! {
    button {
      r#type: "button",
      id,
      class,
      disabled,
      "aria-haspopup": haspopup,
      "aria-expanded": open.to_string(),
      "aria-controls": root.content_id(),
      "data-state": if open { "open" } else { "closed" },
      onclick: move |_| root.set_open.call(!open),
      ..attributes,
      {children}
    }
  }
}
