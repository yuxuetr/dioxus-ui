use crate::{DismissBehavior, FocusStrategy, OverlayAlign, OverlaySide, PortalTarget};

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn controlled_dropdown_uses_menu_defaults() {
    let config = DropdownPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert_eq!(config.focus_strategy, FocusStrategy::Container);
    assert_eq!(config.side, OverlaySide::Bottom);
    assert_eq!(config.align, OverlayAlign::End);
    assert!(config.dismiss.escape_key);
    assert!(config.dismiss.outside_pointer);
  }
}
