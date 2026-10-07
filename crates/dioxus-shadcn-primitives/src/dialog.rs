//! Default configuration for a controlled modal dialog, used by the styled `Dialog`,
//! `AlertDialog`, and `Drawer` components.

use crate::{DismissBehavior, FocusReturn, FocusStrategy, PortalTarget};

/// Initial controlled dialog primitive configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DialogPrimitiveConfig {
  /// Whether the dialog is open.
  pub open: bool,
  /// Where focus moves when the content opens.
  pub focus_strategy: FocusStrategy,
  /// Where focus goes when the content closes.
  pub focus_return: FocusReturn,
  /// Which user actions close the content.
  pub dismiss: DismissBehavior,
  /// Where the content renders in the tree.
  pub portal_target: PortalTarget,
}

impl DialogPrimitiveConfig {
  /// Config for a dialog the app controls: focus moves to the first focusable element and
  /// returns to the trigger on close, only Escape dismisses it, and it renders inline.
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
