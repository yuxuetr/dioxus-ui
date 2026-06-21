use crate::{DismissBehavior, FocusReturn, FocusStrategy, PortalTarget};

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn controlled_dialog_uses_safe_defaults() {
    let config = DialogPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert_eq!(config.focus_strategy, FocusStrategy::FirstFocusable);
    assert_eq!(config.focus_return, FocusReturn::Trigger);
    assert_eq!(config.portal_target, PortalTarget::Inline);
    assert!(config.dismiss.escape_key);
    assert!(!config.dismiss.outside_pointer);
    assert!(!config.dismiss.focus_outside);
  }
}
