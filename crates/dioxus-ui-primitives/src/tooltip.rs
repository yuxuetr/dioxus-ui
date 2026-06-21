use crate::{DismissBehavior, FocusStrategy, OverlaySide, PortalTarget};

/// Initial controlled tooltip primitive configuration.
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn controlled_tooltip_uses_non_modal_defaults() {
    let config = TooltipPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert_eq!(config.delay_ms, 700);
    assert_eq!(config.focus_strategy, FocusStrategy::None);
    assert_eq!(config.side, OverlaySide::Top);
    assert!(config.dismiss.escape_key);
    assert!(!config.dismiss.outside_pointer);
    assert!(!config.dismiss.focus_outside);
  }
}
