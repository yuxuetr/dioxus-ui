use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{OverlaySide, TooltipPrimitiveConfig};

pub const TOOLTIP_CONTENT_BASE_CLASS: &str =
  "z-50 overflow-hidden rounded-md bg-zinc-950 px-3 py-1.5 text-xs text-white shadow-md";

pub fn tooltip_content_class(class: &str) -> String {
  classes([Some(TOOLTIP_CONTENT_BASE_CLASS), Some(class)])
}

#[component]
pub fn TooltipContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = tooltip_content_class(&class);

  rsx! {
    div {
      role: "tooltip",
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
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
