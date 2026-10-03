use dioxus::prelude::*;
use super::utils::classes;
pub use super::utils::{OverlaySide, TooltipPrimitiveConfig};

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
