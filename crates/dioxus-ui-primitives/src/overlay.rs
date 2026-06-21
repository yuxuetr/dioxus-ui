/// Strategy for focusing content when an overlay opens.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum FocusStrategy {
  #[default]
  FirstFocusable,
  Container,
  None,
}

/// Strategy for returning focus when an overlay closes.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum FocusReturn {
  #[default]
  Trigger,
  None,
}

/// Dismissal behavior shared by overlay primitives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DismissBehavior {
  pub escape_key: bool,
  pub outside_pointer: bool,
  pub focus_outside: bool,
}

impl DismissBehavior {
  pub const fn dialog_default() -> Self {
    Self {
      escape_key: true,
      outside_pointer: false,
      focus_outside: false,
    }
  }

  pub const fn popover_default() -> Self {
    Self {
      escape_key: true,
      outside_pointer: true,
      focus_outside: true,
    }
  }

  pub const fn tooltip_default() -> Self {
    Self {
      escape_key: true,
      outside_pointer: false,
      focus_outside: false,
    }
  }
}

/// Planned portal target for overlay content.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum PortalTarget {
  Body,
  Selector(String),
  #[default]
  Inline,
}

/// Preferred overlay side before a positioning engine is introduced.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlaySide {
  Top,
  Right,
  Bottom,
  Left,
  #[default]
  Inline,
}

/// Preferred overlay alignment before a positioning engine is introduced.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlayAlign {
  Start,
  #[default]
  Center,
  End,
}
