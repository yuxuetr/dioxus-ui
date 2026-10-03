use dioxus::prelude::*;
use super::utils::classes;
pub use super::utils::DropdownPrimitiveConfig;

pub const DROPDOWN_CONTENT_BASE_CLASS: &str = "z-50 min-w-32 overflow-hidden rounded-md border border-zinc-200 bg-white p-1 text-zinc-950 shadow-md";
pub const DROPDOWN_GROUP_BASE_CLASS: &str = "p-1";
pub const DROPDOWN_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-zinc-500";
pub const DROPDOWN_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-zinc-100 data-disabled:pointer-events-none data-disabled:opacity-50";
pub const DROPDOWN_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-zinc-200";

pub fn dropdown_content_class(class: &str) -> String {
  classes([Some(DROPDOWN_CONTENT_BASE_CLASS), Some(class)])
}

pub fn dropdown_group_class(class: &str) -> String {
  classes([Some(DROPDOWN_GROUP_BASE_CLASS), Some(class)])
}

pub fn dropdown_label_class(class: &str) -> String {
  classes([Some(DROPDOWN_LABEL_BASE_CLASS), Some(class)])
}

pub fn dropdown_item_class(destructive: bool, class: &str) -> String {
  let variant_class = if destructive {
    "text-red-600 focus:bg-red-50 focus:text-red-700"
  } else {
    "text-zinc-900 focus:bg-zinc-100"
  };

  classes([Some(DROPDOWN_ITEM_BASE_CLASS), Some(variant_class), Some(class)])
}

pub fn dropdown_separator_class(class: &str) -> String {
  classes([Some(DROPDOWN_SEPARATOR_BASE_CLASS), Some(class)])
}

#[component]
pub fn DropdownContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = dropdown_content_class(&class);

  rsx! {
    div {
      role: "menu",
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[component]
pub fn DropdownGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = dropdown_group_class(&class);

  rsx! {
    div {
      role: "group",
      class,
      {children}
    }
  }
}

#[component]
pub fn DropdownLabel(#[props(default)] class: String, children: Element) -> Element {
  let class = dropdown_label_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn DropdownItem(
  #[props(default)] destructive: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = dropdown_item_class(destructive, &class);

  rsx! {
    div {
      role: "menuitem",
      class,
      "data-disabled": disabled.to_string(),
      {children}
    }
  }
}

#[component]
pub fn DropdownSeparator(#[props(default)] class: String) -> Element {
  let class = dropdown_separator_class(&class);

  rsx! {
    div {
      role: "separator",
      class,
    }
  }
}
