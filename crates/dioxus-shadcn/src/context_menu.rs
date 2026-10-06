use dioxus::prelude::*;
use dioxus_shadcn_core::classes;
pub use dioxus_shadcn_primitives::{
  DismissBehavior, DropdownPrimitiveConfig, OverlayAlign, OverlaySide,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::listbox::{ListboxMode, use_listbox};
use crate::menu_marks::{
  MENU_CHECKBOX_MARK_CLASS, MENU_RADIO_MARK_CLASS, MENU_SUB_TRIGGER_CLASS, menu_mark_state_class,
};
use crate::menu_sub::{MenuSubContext, use_menu_sub, use_menu_sub_content};

pub const CONTEXT_MENU_CONTENT_BASE_CLASS: &str = "z-50 min-w-32 overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md";
pub const CONTEXT_MENU_GROUP_BASE_CLASS: &str = "p-1";
pub const CONTEXT_MENU_LABEL_BASE_CLASS: &str =
  "px-2 py-1.5 text-xs font-medium text-muted-foreground";
pub const CONTEXT_MENU_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const CONTEXT_MENU_ITEM_INSET_CLASS: &str = "pl-8";
pub const CONTEXT_MENU_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-border";
pub const CONTEXT_MENU_SHORTCUT_BASE_CLASS: &str =
  "ml-auto text-xs tracking-normal text-muted-foreground";

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
    "text-destructive focus:bg-destructive/10 focus:text-destructive"
  } else {
    "text-foreground focus:bg-accent"
  };
  let inset_class = if inset { CONTEXT_MENU_ITEM_INSET_CLASS } else { "" };

  classes([Some(CONTEXT_MENU_ITEM_BASE_CLASS), Some(variant_class), Some(inset_class), Some(class)])
}

pub fn context_menu_separator_class(class: &str) -> String {
  classes([Some(CONTEXT_MENU_SEPARATOR_BASE_CLASS), Some(class)])
}

/// An inset item with a check mark shown while `checked`.
pub fn context_menu_checkbox_item_class(checked: bool, class: &str) -> String {
  let mark =
    classes([Some(MENU_CHECKBOX_MARK_CLASS), Some(menu_mark_state_class(checked)), Some(class)]);
  context_menu_item_class(true, false, &mark)
}

/// An inset item with a dot shown while `checked`.
pub fn context_menu_radio_item_class(checked: bool, class: &str) -> String {
  let mark =
    classes([Some(MENU_RADIO_MARK_CLASS), Some(menu_mark_state_class(checked)), Some(class)]);
  context_menu_item_class(true, false, &mark)
}

/// A sub trigger: an item with a chevron at its end.
pub fn context_menu_sub_trigger_class(inset: bool, class: &str) -> String {
  context_menu_item_class(inset, false, &classes([Some(MENU_SUB_TRIGGER_CLASS), Some(class)]))
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
  let class = context_menu_checkbox_item_class(checked, &class);

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
  let class = context_menu_radio_item_class(checked, &class);

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

/// A nested menu (RFC 0067). Put `ContextMenuSubTrigger` and `ContextMenuSubContent`
/// inside and pass both the same `open`; `on_open_change` reports requests
/// to open and close it.
#[component]
pub fn ContextMenuSub(
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  use_menu_sub(on_open_change);

  rsx! {
    div {
      role: "group",
      class,
      {children}
    }
  }
}

/// Opens its submenu on click, Enter, Space, ArrowRight (ArrowLeft in
/// right-to-left), or hover.
#[component]
pub fn ContextMenuSubTrigger(
  #[props(default)] open: bool,
  #[props(default)] inset: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let context = try_use_context::<MenuSubContext>();
  let id = context.as_ref().map(MenuSubContext::trigger_id);
  let controls = context.as_ref().map(MenuSubContext::content_id);
  let on_open_change = context.and_then(|context| context.on_open_change);
  let class = context_menu_sub_trigger_class(inset, &class);

  rsx! {
    div {
      role: "menuitem",
      id,
      class,
      "aria-haspopup": "menu",
      "aria-expanded": open.to_string(),
      "aria-controls": controls,
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-state": if open { "open" } else { "closed" },
      onclick: move |_| {
        if let Some(handler) = on_open_change.filter(|_| !disabled) {
          handler.call(true);
        }
      },
      {children}
    }
  }
}

/// The submenu, placed at its trigger's inline end. ArrowLeft (ArrowRight in
/// right-to-left) and Escape close it and return focus to the trigger;
/// choosing an item closes every level.
#[component]
pub fn ContextMenuSubContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let (context, listbox, anchored) = use_menu_sub_content(open);
  let id = context.as_ref().map(MenuSubContext::content_id);
  let labelledby = context.as_ref().map(MenuSubContext::trigger_id);
  let class = context_menu_content_class(&class);

  rsx! {
    div {
      role: "menu",
      id,
      class,
      hidden: !open,
      "aria-labelledby": labelledby,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-submenu": "",
      "data-dxui-anchored": anchored,
      "data-dxui-listbox": listbox,
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn context_menu_item_class_reflects_destructive_and_inset_state() {
    let actual = context_menu_item_class(true, true, "gap-2");

    assert!(actual.contains(CONTEXT_MENU_ITEM_BASE_CLASS));
    assert!(actual.contains(CONTEXT_MENU_ITEM_INSET_CLASS));
    assert!(actual.contains("text-destructive focus:bg-destructive/10 focus:text-destructive"));
    assert!(actual.ends_with("gap-2"));
  }

  #[test]
  fn context_menu_primitive_config_is_reexported() {
    let config = DropdownPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
