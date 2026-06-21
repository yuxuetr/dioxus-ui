use crate::{DismissBehavior, FocusStrategy, OverlayAlign, OverlaySide, PortalTarget};

/// Initial controlled select primitive configuration.
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn controlled_select_uses_listbox_defaults() {
    let config = SelectPrimitiveConfig::controlled(true, Some("dark".to_string()));

    assert!(config.open);
    assert_eq!(config.value, Some("dark".to_string()));
    assert_eq!(config.focus_strategy, FocusStrategy::Container);
    assert_eq!(config.side, OverlaySide::Bottom);
    assert_eq!(config.align, OverlayAlign::Start);
    assert!(config.dismiss.escape_key);
    assert!(config.dismiss.outside_pointer);
  }
}
