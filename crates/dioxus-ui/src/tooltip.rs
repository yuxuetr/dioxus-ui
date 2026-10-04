use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  DismissBehavior, OverlayAlign, OverlaySide, TooltipPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};

pub const TOOLTIP_CONTENT_BASE_CLASS: &str =
  "z-50 overflow-hidden rounded-md bg-zinc-950 px-3 py-1.5 text-xs text-white shadow-md";

pub fn tooltip_content_class(class: &str) -> String {
  classes([Some(TOOLTIP_CONTENT_BASE_CLASS), Some(class)])
}

#[component]
pub fn TooltipContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] anchor_id: Option<String>,
  #[props(default = OverlaySide::Top)] side: OverlaySide,
  #[props(default)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::tooltip_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let class = tooltip_content_class(&class);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, side, align, side_offset },
    dismiss,
    on_open_change,
  );

  rsx! {
    div {
      role: "tooltip",
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tooltip_content_class_appends_user_class() {
    let actual = tooltip_content_class("max-w-48");

    assert!(actual.contains(TOOLTIP_CONTENT_BASE_CLASS));
    assert!(actual.ends_with("max-w-48"));
  }

  #[test]
  fn tooltip_primitive_config_is_reexported() {
    let config = TooltipPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert_eq!(config.side, OverlaySide::Top);
  }
}
