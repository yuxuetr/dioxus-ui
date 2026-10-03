pub fn classes<I, S>(parts: I) -> String
where
  I: IntoIterator<Item = Option<S>>,
  S: AsRef<str>,
{
  let mut output = String::new();

  for part in parts.into_iter().flatten() {
    for token in part.as_ref().split_whitespace() {
      if !output.is_empty() {
        output.push(' ');
      }

      output.push_str(token);
    }
  }

  output
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UiDensity {
  Compact,
  #[default]
  Comfortable,
  Touch,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum FocusStrategy {
  #[default]
  FirstFocusable,
  Container,
  None,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum FocusReturn {
  #[default]
  Trigger,
  None,
}

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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum PortalTarget {
  Body,
  Selector(String),
  #[default]
  Inline,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlaySide {
  Top,
  Right,
  Bottom,
  Left,
  #[default]
  Inline,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlayAlign {
  Start,
  #[default]
  Center,
  End,
}

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TooltipPrimitiveConfig {
  pub open: bool,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectPrimitiveConfig {
  pub open: bool,
  pub value: Option<String>,
  pub focus_strategy: FocusStrategy,
  pub dismiss: DismissBehavior,
  pub portal_target: PortalTarget,
  pub side: OverlaySide,
  pub align: OverlayAlign,
}

impl SelectPrimitiveConfig {
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
