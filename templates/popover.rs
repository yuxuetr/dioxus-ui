use dioxus::prelude::*;
use dioxus_ui_core::classes;
use dioxus_ui_primitives::{OverlayAlign, OverlaySide, PopoverPrimitiveConfig};

pub const POPOVER_CONTENT_BASE_CLASS: &str = "z-50 w-72 rounded-md border border-zinc-200 bg-white p-4 text-zinc-950 shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";
pub const POPOVER_HEADER_BASE_CLASS: &str = "grid gap-1";
pub const POPOVER_TITLE_BASE_CLASS: &str = "font-medium leading-none text-zinc-950";
pub const POPOVER_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";

pub fn popover_content_class(class: &str) -> String {
  classes([Some(POPOVER_CONTENT_BASE_CLASS), Some(class)])
}

pub fn popover_header_class(class: &str) -> String {
  classes([Some(POPOVER_HEADER_BASE_CLASS), Some(class)])
}

pub fn popover_title_class(class: &str) -> String {
  classes([Some(POPOVER_TITLE_BASE_CLASS), Some(class)])
}

pub fn popover_description_class(class: &str) -> String {
  classes([Some(POPOVER_DESCRIPTION_BASE_CLASS), Some(class)])
}

#[component]
pub fn PopoverContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = popover_content_class(&class);

  rsx! {
    div {
      role: "dialog",
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[component]
pub fn PopoverHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = popover_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn PopoverTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = popover_title_class(&class);

  rsx! {
    h3 {
      class,
      {children}
    }
  }
}

#[component]
pub fn PopoverDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = popover_description_class(&class);

  rsx! {
    p {
      class,
      {children}
    }
  }
}
