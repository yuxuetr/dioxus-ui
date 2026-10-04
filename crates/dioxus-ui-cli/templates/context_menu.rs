use dioxus::prelude::*;
use super::utils::{AnchoredPlacement, ListboxMode, classes, use_anchored_overlay, use_listbox};
pub use super::utils::{DismissBehavior, DropdownPrimitiveConfig, OverlayAlign, OverlaySide};

pub const CONTEXT_MENU_CONTENT_BASE_CLASS: &str = "z-50 min-w-32 overflow-hidden rounded-md border border-zinc-200 bg-white p-1 text-zinc-950 shadow-md";
pub const CONTEXT_MENU_GROUP_BASE_CLASS: &str = "p-1";
pub const CONTEXT_MENU_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-zinc-500";
pub const CONTEXT_MENU_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors data-disabled:pointer-events-none data-disabled:opacity-50";
pub const CONTEXT_MENU_ITEM_INSET_CLASS: &str = "pl-8";
pub const CONTEXT_MENU_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-zinc-200";
pub const CONTEXT_MENU_SHORTCUT_BASE_CLASS: &str = "ml-auto text-xs tracking-normal text-zinc-500";

pub fn context_menu_content_class(class: &str) -> String {
  classes([Some(CONTEXT_MENU_CONTENT_BASE_CLASS), Some(class)])
}

pub fn context_menu_group_class(class: &str) -> String {
  classes([Some(CONTEXT_MENU_GROUP_BASE_CLASS), Some(class)])
}

pub fn context_menu_label_class(class: &str) -> String {
  classes([Some(CONTEXT_MENU_LABEL_BASE_CLASS), Some(class)])
}

pub fn context_menu_item_class(inset: bool, destructive: bool, class: &str) -> String {
  let variant_class = if destructive {
    "text-red-600 focus:bg-red-50 focus:text-red-700"
  } else {
    "text-zinc-900 focus:bg-zinc-100"
  };
  let inset_class = if inset { CONTEXT_MENU_ITEM_INSET_CLASS } else { "" };

  classes([
    Some(CONTEXT_MENU_ITEM_BASE_CLASS),
    Some(variant_class),
    Some(inset_class),
    Some(class),
  ])
}

pub fn context_menu_separator_class(class: &str) -> String {
  classes([Some(CONTEXT_MENU_SEPARATOR_BASE_CLASS), Some(class)])
}

pub fn context_menu_shortcut_class(class: &str) -> String {
  classes([Some(CONTEXT_MENU_SHORTCUT_BASE_CLASS), Some(class)])
}

/// With `anchor_point` (viewport coordinates, usually the `oncontextmenu`
/// event's client coordinates) the menu's corner is placed at that point,
/// flipping and shifting to stay in the viewport. Opening focuses the first
/// enabled item; arrows wrap, Home, End, and typeahead jump, and activating an
/// item requests close and returns focus. Escape and outside interactions
/// request close per `dismiss`.
#[component]
pub fn ContextMenuContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] anchor_point: Option<(f64, f64)>,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Start)] align: OverlayAlign,
  #[props(default)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let class = context_menu_content_class(&class);
  let menu = use_listbox(open, None, ListboxMode::Menu, None, on_open_change);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id: None, anchor_point, side, align, side_offset },
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
pub fn ContextMenuGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = context_menu_group_class(&class);

  rsx! {
    div {
      role: "group",
      class,
      {children}
    }
  }
}

#[component]
pub fn ContextMenuLabel(#[props(default)] class: String, children: Element) -> Element {
  let class = context_menu_label_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn ContextMenuItem(
  #[props(default)] inset: bool,
  #[props(default)] destructive: bool,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = context_menu_item_class(inset, destructive, &class);

  rsx! {
    div {
      role: "menuitem",
      class,
      "aria-disabled": disabled.to_string(),
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
pub fn ContextMenuCheckboxItem(
  #[props(default)] checked: bool,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = context_menu_item_class(true, false, &class);

  rsx! {
    div {
      role: "menuitemcheckbox",
      class,
      "aria-checked": checked.to_string(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      onclick: move |event| {
        if let Some(handler) = onclick.filter(|_| !disabled) {
          handler.call(event);
        }
      },
      "data-state": if checked { "checked" } else { "unchecked" },
      {children}
    }
  }
}

#[component]
pub fn ContextMenuRadioGroup(
  #[props(default)] value: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  rsx! {
    div {
      role: "group",
      class,
      "data-value": value,
      {children}
    }
  }
}

#[component]
pub fn ContextMenuRadioItem(
  #[props(default)] checked: bool,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = context_menu_item_class(true, false, &class);

  rsx! {
    div {
      role: "menuitemradio",
      class,
      "aria-checked": checked.to_string(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      onclick: move |event| {
        if let Some(handler) = onclick.filter(|_| !disabled) {
          handler.call(event);
        }
      },
      "data-state": if checked { "checked" } else { "unchecked" },
      {children}
    }
  }
}

#[component]
pub fn ContextMenuSeparator(#[props(default)] class: String) -> Element {
  let class = context_menu_separator_class(&class);

  rsx! {
    div {
      role: "separator",
      class,
    }
  }
}

#[component]
pub fn ContextMenuShortcut(#[props(default)] class: String, children: Element) -> Element {
  let class = context_menu_shortcut_class(&class);

  rsx! {
    span {
      class,
      {children}
    }
  }
}
