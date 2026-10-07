//! Overlay types and primitive configs, copied from the
//! `dioxus-shadcn-primitives` crate.

/// Strategy for focusing content when an overlay opens.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FocusStrategy {
  /// Focus the first focusable element inside the content.
  #[default]
  FirstFocusable,
  /// Focus the content container itself.
  Container,
  None,
}

/// Strategy for returning focus when an overlay closes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FocusReturn {
  /// Move focus back to the element that opened the overlay.
  #[default]
  Trigger,
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
  pub const fn dialog_default() -> Self {
    Self { escape_key: true, outside_pointer: false, focus_outside: false }
  }

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
  #[default]
  Inline,
}

/// Preferred overlay alignment before a positioning engine is introduced.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlayAlign {
  Start,
  /// Center on the anchor.
  #[default]
  Center,
  End,
}

/// Initial controlled dialog primitive configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DialogPrimitiveConfig {
  pub open: bool,
  pub focus_strategy: FocusStrategy,
  pub focus_return: FocusReturn,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
}

impl DialogPrimitiveConfig {
  pub fn controlled(open: bool) -> Self {
    Self {
      open,
      focus_strategy: FocusStrategy::default(),
      focus_return: FocusReturn::default(),
      dismiss: DismissBehavior::dialog_default(),
      portal_target: PortalTarget::default(),
    }
  }
}

/// Initial controlled popover primitive configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PopoverPrimitiveConfig {
  pub open: bool,
  pub focus_strategy: FocusStrategy,
  pub focus_return: FocusReturn,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
  pub side: OverlaySide,
  pub align: OverlayAlign,
}

impl PopoverPrimitiveConfig {
  pub fn controlled(open: bool) -> Self {
    Self {
      open,
      focus_strategy: FocusStrategy::None,
      focus_return: FocusReturn::Trigger,
      dismiss: DismissBehavior::popover_default(),
      portal_target: PortalTarget::default(),
      side: OverlaySide::Bottom,
      align: OverlayAlign::Center,
    }
  }
}

/// Initial controlled tooltip primitive configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TooltipPrimitiveConfig {
  pub open: bool,
  /// Hover delay before the tooltip shows, in milliseconds.
  pub delay_ms: u16,
  pub focus_strategy: FocusStrategy,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
  pub side: OverlaySide,
}

impl TooltipPrimitiveConfig {
  pub fn controlled(open: bool) -> Self {
    Self {
      open,
      delay_ms: 700,
      focus_strategy: FocusStrategy::None,
      dismiss: DismissBehavior::tooltip_default(),
      portal_target: PortalTarget::default(),
      side: OverlaySide::Top,
    }
  }
}

/// Initial controlled select primitive configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectPrimitiveConfig {
  pub open: bool,
  /// Selected option value; `None` when nothing is selected.
  pub value: Option<String>,
  pub focus_strategy: FocusStrategy,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
  pub side: OverlaySide,
  pub align: OverlayAlign,
}

impl SelectPrimitiveConfig {
  /// Config for a select the app controls: focus goes to the listbox, outside clicks,
  /// focus leaving, and Escape close it, and it opens below the trigger, start-aligned.
  pub fn controlled(open: bool, value: Option<String>) -> Self {
    Self {
      open,
      value,
      focus_strategy: FocusStrategy::Container,
      dismiss: DismissBehavior::popover_default(),
      portal_target: PortalTarget::default(),
      side: OverlaySide::Bottom,
      align: OverlayAlign::Start,
    }
  }
}

/// Initial controlled dropdown menu primitive configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DropdownPrimitiveConfig {
  pub open: bool,
  pub focus_strategy: FocusStrategy,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
  pub side: OverlaySide,
  pub align: OverlayAlign,
}

impl DropdownPrimitiveConfig {
  pub fn controlled(open: bool) -> Self {
    Self {
      open,
      focus_strategy: FocusStrategy::Container,
      dismiss: DismissBehavior::popover_default(),
      portal_target: PortalTarget::default(),
      side: OverlaySide::Bottom,
      align: OverlayAlign::End,
    }
  }
}
