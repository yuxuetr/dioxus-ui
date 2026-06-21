use crate::{
  DismissBehavior, FocusReturn, FocusStrategy, OverlayAlign, OverlaySide, PortalTarget,
};

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn controlled_popover_uses_dismissible_defaults() {
    let config = PopoverPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert_eq!(config.focus_strategy, FocusStrategy::None);
    assert_eq!(config.focus_return, FocusReturn::Trigger);
    assert_eq!(config.side, OverlaySide::Bottom);
    assert!(config.dismiss.escape_key);
    assert!(config.dismiss.outside_pointer);
    assert!(config.dismiss.focus_outside);
  }
}
