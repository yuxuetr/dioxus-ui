//! Default configuration for a controlled dropdown menu, used by the styled
//! `DropdownMenu`, `ContextMenu`, and `Menubar` components.

use crate::{DismissBehavior, FocusStrategy, OverlayAlign, OverlaySide, PortalTarget};

/// Initial controlled dropdown menu primitive configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DropdownPrimitiveConfig {
  /// Whether the menu is open.
  pub open: bool,
  /// Where focus moves when the content opens.
  pub focus_strategy: FocusStrategy,
  /// Which user actions close the content.
  pub dismiss: DismissBehavior,
  /// Where the content renders in the tree.
  pub portal_target: PortalTarget,
  /// Side of the trigger the content opens on.
  pub side: OverlaySide,
  /// Alignment of the content along the trigger's edge.
  pub align: OverlayAlign,
}

impl DropdownPrimitiveConfig {
  /// Config for a menu the app controls: focus goes to the menu, outside clicks, focus
  /// leaving, and Escape close it, and it opens below the trigger, end-aligned.
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
