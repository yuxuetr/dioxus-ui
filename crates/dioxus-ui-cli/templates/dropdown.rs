use dioxus::prelude::*;
use super::utils::{AnchoredPlacement, ListboxMode, classes, use_anchored_overlay, use_listbox};
pub use super::utils::{DismissBehavior, DropdownPrimitiveConfig, OverlayAlign, OverlaySide};

pub const DROPDOWN_CONTENT_BASE_CLASS: &str = "z-50 min-w-32 overflow-hidden rounded-md border border-zinc-200 bg-white p-1 text-zinc-950 shadow-md";
pub const DROPDOWN_GROUP_BASE_CLASS: &str = "p-1";
pub const DROPDOWN_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-zinc-500";
pub const DROPDOWN_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-zinc-100 data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
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

/// Opening focuses the first enabled item. Arrows move focus with wrapping,
/// Home, End, and typeahead jump, and activating an item requests close and
/// returns focus to where it was before opening. With `anchor_id` the menu is
/// placed next to that element. Escape and outside interactions request close
/// per `dismiss`.
#[component]
pub fn DropdownContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] anchor_id: Option<String>,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::End)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let class = dropdown_content_class(&class);
  let menu = use_listbox(open, anchor_id.clone(), ListboxMode::Menu, None, on_open_change);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    on_open_change,
  );

  rsx! {
    div {
      role: "menu",
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      "data-dxui-listbox": menu,
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

/// Enter, Space, or click on an enabled item calls `onclick`; the menu then
/// requests close.
#[component]
pub fn DropdownItem(
  #[props(default)] destructive: bool,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = dropdown_item_class(destructive, &class);

  rsx! {
    div {
      role: "menuitem",
      class,
      "data-disabled": disabled.to_string(),
      onclick: move |event| {
        if let Some(handler) = onclick.filter(|_| !disabled) {
          handler.call(event);
        }
      },
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
