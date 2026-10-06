//! Submenus for Dropdown, Context Menu, and Menubar (RFC 0067).

use super::element_id::next_element_id;
use super::overlay::{DismissBehavior, OverlayAlign, OverlaySide};
use dioxus::prelude::*;

use super::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use super::listbox::{ListboxMode, use_listbox};

/// What a submenu's trigger and content share: their ids and the app's open
/// handler.
#[derive(Clone, PartialEq)]
pub struct MenuSubContext {
  base_id: String,
  pub on_open_change: Option<EventHandler<bool>>,
}

impl MenuSubContext {
  pub fn trigger_id(&self) -> String {
    format!("{}-trigger", self.base_id)
  }

  pub fn content_id(&self) -> String {
    format!("{}-content", self.base_id)
  }
}

/// Provides the context the submenu's trigger and content read.
pub fn use_menu_sub(on_open_change: Option<EventHandler<bool>>) {
  let base_id = use_hook(|| format!("dxui-menu-sub-{}", next_element_id()));
  use_context_provider(|| MenuSubContext { base_id, on_open_change });
}

/// Runs a submenu's menu script, anchored to its trigger, and places it at
/// the trigger's inline end. Returns the context and the content's
/// `data-dxui-listbox` and `data-dxui-anchored` values.
pub fn use_menu_sub_content(open: bool) -> (Option<MenuSubContext>, String, String) {
  let context = try_use_context::<MenuSubContext>();
  let trigger_id = context.as_ref().map(MenuSubContext::trigger_id);
  let on_open_change = context.as_ref().and_then(|context| context.on_open_change);
  let listbox = use_listbox(open, trigger_id.clone(), ListboxMode::Menu, None, on_open_change);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement {
      anchor_id: trigger_id,
      anchor_point: None,
      side: OverlaySide::Right,
      align: OverlayAlign::Start,
      side_offset: -4,
    },
    DismissBehavior::popover_default(),
    on_open_change,
  );
  (context, listbox, anchored)
}
