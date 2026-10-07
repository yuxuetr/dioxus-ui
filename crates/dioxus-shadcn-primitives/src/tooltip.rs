//! Default configuration for a controlled tooltip, used by the styled `Tooltip`
//! component.

use crate::{DismissBehavior, FocusStrategy, OverlaySide, PortalTarget};

/// Initial controlled tooltip primitive configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TooltipPrimitiveConfig {
  /// Whether the tooltip is shown.
  pub open: bool,
  /// Hover delay before the tooltip shows, in milliseconds.
  pub delay_ms: u16,
  /// Where focus moves when the content opens.
  pub focus_strategy: FocusStrategy,
  /// Which user actions close the content.
  pub dismiss: DismissBehavior,
  /// Where the content renders in the tree.
  pub portal_target: PortalTarget,
  /// Side of the trigger the content opens on.
  pub side: OverlaySide,
}

impl TooltipPrimitiveConfig {
  /// Config for a tooltip the app controls: a 700 ms delay, no focus movement, only
  /// Escape dismisses it, and it shows above the trigger.
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
