//! Default configuration for a controlled popover, used by the styled `Popover`,
//! `HoverCard`, `Combobox`, `DatePicker`, and `NavigationMenu` components.

use crate::{DismissBehavior, FocusReturn, FocusStrategy, OverlayAlign, OverlaySide, PortalTarget};

/// Initial controlled popover primitive configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PopoverPrimitiveConfig {
  /// Whether the popover is open.
  pub open: bool,
  /// Where focus moves when the content opens.
  pub focus_strategy: FocusStrategy,
  /// Where focus goes when the content closes.
  pub focus_return: FocusReturn,
  /// Which user actions close the content.
  pub dismiss: DismissBehavior,
  /// Where the content renders in the tree.
  pub portal_target: PortalTarget,
  /// Side of the trigger the content opens on.
  pub side: OverlaySide,
  /// Alignment of the content along the trigger's edge.
  pub align: OverlayAlign,
}

impl PopoverPrimitiveConfig {
  /// Config for a popover the app controls: focus stays put on open and returns to the
  /// trigger on close, outside clicks, focus leaving, and Escape close it, and it opens
  /// below the trigger, centered.
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
