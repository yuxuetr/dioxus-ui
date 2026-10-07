//! Shared overlay settings: where focus goes on open and close, what dismisses the overlay,
//! where it renders, and which side of its trigger it prefers. Dialog, popover, tooltip, and
//! the other overlay primitives configure themselves with these.

/// Strategy for focusing content when an overlay opens.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FocusStrategy {
  /// Focus the first focusable element inside the content.
  #[default]
  FirstFocusable,
  /// Focus the content container itself.
  Container,
  /// Leave focus where it is.
  None,
}

/// Strategy for returning focus when an overlay closes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FocusReturn {
  /// Move focus back to the element that opened the overlay.
  #[default]
  Trigger,
  /// Leave focus where it is.
  None,
}

/// Dismissal behavior shared by overlay primitives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DismissBehavior {
  /// Pressing Escape closes the overlay.
  pub escape_key: bool,
  /// A pointer press outside the content closes the overlay.
  pub outside_pointer: bool,
  /// Focus moving outside the content closes the overlay.
  pub focus_outside: bool,
}

impl DismissBehavior {
  /// Dialogs close on Escape only; outside presses and focus loss keep them open.
  pub const fn dialog_default() -> Self {
    Self { escape_key: true, outside_pointer: false, focus_outside: false }
  }

  /// Popovers close on Escape, an outside press, or focus leaving them.
  pub const fn popover_default() -> Self {
    Self { escape_key: true, outside_pointer: true, focus_outside: true }
  }

  /// Tooltips close on Escape only.
  pub const fn tooltip_default() -> Self {
    Self { escape_key: true, outside_pointer: false, focus_outside: false }
  }
}

/// Planned portal target for overlay content.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum PortalTarget {
  /// Render at the end of the document body.
  Body,
  /// Render inside the element matching this CSS selector.
  Selector(String),
  /// Render in place, next to the trigger.
  #[default]
  Inline,
}

/// Preferred overlay side before a positioning engine is introduced.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlaySide {
  /// Above the anchor.
  Top,
  /// Right of the anchor.
  Right,
  /// Below the anchor.
  Bottom,
  /// Left of the anchor.
  Left,
  /// At the anchor's top-left corner, rendered in place; never flipped or shifted.
  #[default]
  Inline,
}

/// Preferred overlay alignment before a positioning engine is introduced.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlayAlign {
  /// Line up with the anchor's leading edge (left or top).
  Start,
  /// Center on the anchor.
  #[default]
  Center,
  /// Line up with the anchor's trailing edge (right or bottom).
  End,
}
