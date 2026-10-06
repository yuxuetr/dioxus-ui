//! Submenus for Dropdown, Context Menu, and Menubar (RFC 0067).

use dioxus::prelude::*;
use dioxus_shadcn_primitives::{DismissBehavior, OverlayAlign, OverlaySide};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::listbox::{ListboxMode, use_listbox};
use crate::overlay_root::{OverlayRoot, use_overlay_root};
use crate::root_state::use_root_context;

/// What a submenu's trigger and content share: whether it is open, and their
/// ids (RFC 0077).
#[derive(Clone, Copy)]
struct MenuSubContext(OverlayRoot);

/// Called by a submenu root; it owns whether the submenu is open.
pub(crate) fn use_menu_sub(
  open: ReadSignal<Option<bool>>,
  default_open: bool,
  on_open_change: Option<EventHandler<bool>>,
) {
  let root = use_overlay_root("menu-sub", open, default_open, on_open_change);
  use_context_provider(|| MenuSubContext(root));
}

/// The submenu a trigger or content `part` belongs to, named `root`.
pub(crate) fn use_menu_sub_part(part: &str, root: &str) -> OverlayRoot {
  use_root_context::<MenuSubContext>(part, root).0
}

/// Runs a submenu's menu script, anchored to its trigger, and places it at
/// the trigger's inline end. Returns the submenu and the content's
/// `data-dxui-listbox` and `data-dxui-anchored` values.
pub(crate) fn use_menu_sub_content(part: &str, root: &str) -> (OverlayRoot, String, String) {
  let sub = use_menu_sub_part(part, root);
  let open = sub.is_open();
  let listbox =
    use_listbox(open, Some(sub.trigger_id()), ListboxMode::Menu, None, Some(sub.set_open));
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement {
      anchor_id: Some(sub.trigger_id()),
      anchor_point: None,
      side: OverlaySide::Right,
      align: OverlayAlign::Start,
      side_offset: -4,
    },
    DismissBehavior::popover_default(),
    Some(sub.set_open),
  );
  (sub, listbox, anchored)
}
