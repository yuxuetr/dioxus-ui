//! Default configuration for a controlled select listbox, used by the styled
//! `Select` component.

use crate::{DismissBehavior, FocusStrategy, OverlayAlign, OverlaySide, PortalTarget};

/// Initial controlled select primitive configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectPrimitiveConfig {
  /// Whether the listbox is open.
  pub open: bool,
  /// Selected option value; `None` when nothing is selected.
  pub value: Option<String>,
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

impl SelectPrimitiveConfig {
  /// Config for a select the app controls: focus goes to the listbox, outside clicks,
  /// focus leaving, and Escape close it, and it opens below the trigger, start-aligned.
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
