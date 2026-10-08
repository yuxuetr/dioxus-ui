//! Layer: the order in which overlays opened, so one Escape closes only the
//! overlay opened last.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use dioxus::core::provide_root_context;
use dioxus::prelude::*;

/// The open overlays of one virtual DOM, oldest first.
#[derive(Clone, Default)]
struct OpenLayers {
  next: Rc<Cell<usize>>,
  open: Rc<RefCell<Vec<usize>>>,
}

/// One overlay's place among the open ones.
#[derive(Clone)]
pub(crate) struct Layer {
  layers: OpenLayers,
  id: usize,
}

impl Layer {
  /// Whether this overlay is the one opened last of those still open. A
  /// dialog's Escape handler acts only then, so an Escape in a popover or a
  /// dialog opened inside it closes that one and leaves the dialog open.
  pub(crate) fn is_top(&self) -> bool {
    self.layers.open.borrow().last() == Some(&self.id)
  }
}

/// Counts the overlay among the open ones while `open` is true.
pub(crate) fn use_layer(open: bool) -> Layer {
  let layer = use_hook(|| {
    let layers = try_consume_context::<OpenLayers>()
      .unwrap_or_else(|| provide_root_context(OpenLayers::default()));
    let id = layers.next.get();
    layers.next.set(id + 1);
    Layer { layers, id }
  });
  let registered = layer.clone();
  use_effect(use_reactive((&open,), move |(open,)| {
    let mut layers = registered.layers.open.borrow_mut();
    layers.retain(|id| *id != registered.id);
    if open {
      layers.push(registered.id);
    }
  }));
  let dropped = layer.clone();
  use_drop(move || dropped.layers.open.borrow_mut().retain(|id| *id != dropped.id));
  layer
}
