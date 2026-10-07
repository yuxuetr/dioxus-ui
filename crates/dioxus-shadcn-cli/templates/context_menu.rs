use super::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use super::listbox::{ListboxMode, use_listbox};
use super::menu_marks::{
  MENU_CHECKBOX_MARK_CLASS, MENU_RADIO_MARK_CLASS, MENU_SUB_TRIGGER_CLASS, menu_mark_state_class,
};
use super::menu_radio::{use_menu_radio_group, use_menu_radio_item};
use super::menu_sub::{use_menu_sub, use_menu_sub_content, use_menu_sub_part};
pub use super::overlay::{DismissBehavior, DropdownPrimitiveConfig, OverlayAlign, OverlaySide};
use super::overlay_root::{OverlayRoot, use_overlay_root};
use super::root_state::use_root_context;
use super::utils::{classes, merge_classes};
use super::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;

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
  merge_classes(classes([Some(CONTEXT_MENU_CONTENT_BASE_CLASS)]), class)
}

pub fn context_menu_group_class(class: &str) -> String {
  merge_classes(classes([Some(CONTEXT_MENU_GROUP_BASE_CLASS)]), class)
}

pub fn context_menu_label_class(class: &str) -> String {
  merge_classes(classes([Some(CONTEXT_MENU_LABEL_BASE_CLASS)]), class)
}

pub fn context_menu_item_class(inset: bool, destructive: bool, class: &str) -> String {
  let variant_class = if destructive {
    "text-destructive focus:bg-destructive/10 focus:text-destructive"
  } else {
    "text-foreground focus:bg-accent"
  };
  let inset_class = if inset { CONTEXT_MENU_ITEM_INSET_CLASS } else { "" };

  merge_classes(
    classes([Some(CONTEXT_MENU_ITEM_BASE_CLASS), Some(variant_class), Some(inset_class)]),
    class,
  )
}

pub fn context_menu_separator_class(class: &str) -> String {
  merge_classes(classes([Some(CONTEXT_MENU_SEPARATOR_BASE_CLASS)]), class)
}

/// An inset item with a check mark shown while `checked`.
pub fn context_menu_checkbox_item_class(checked: bool, class: &str) -> String {
  let mark = merge_classes(
    classes([Some(MENU_CHECKBOX_MARK_CLASS), Some(menu_mark_state_class(checked))]),
    class,
  );
  context_menu_item_class(true, false, &mark)
}

/// An inset item with a dot shown while `checked`.
pub fn context_menu_radio_item_class(checked: bool, class: &str) -> String {
  let mark = merge_classes(
    classes([Some(MENU_RADIO_MARK_CLASS), Some(menu_mark_state_class(checked))]),
    class,
  );
  context_menu_item_class(true, false, &mark)
}

/// A sub trigger: an item with a chevron at its end.
pub fn context_menu_sub_trigger_class(inset: bool, class: &str) -> String {
  context_menu_item_class(
    inset,
    false,
    &merge_classes(classes([Some(MENU_SUB_TRIGGER_CLASS)]), class),
  )
}

pub fn context_menu_shortcut_class(class: &str) -> String {
  merge_classes(classes([Some(CONTEXT_MENU_SHORTCUT_BASE_CLASS)]), class)
}

/// What a `ContextMenu` shares with its parts: whether it is open, and the
/// viewport point the menu opens at.
#[derive(Clone, Copy)]
struct ContextMenuContext {
  root: OverlayRoot,
  point: Signal<(f64, f64)>,
}

fn use_context_menu(part: &str) -> ContextMenuContext {
  use_root_context::<ContextMenuContext>(part, "ContextMenu")
}

/// The root of a context menu: it owns whether the menu is open. Pass `open`
/// to control it, or `default_open` to start it; `on_open_change` hears every
/// change the user makes either way.
#[component]
pub fn ContextMenu(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let root = use_overlay_root("context-menu", open, default_open, on_open_change);
  let point = use_signal(|| (0.0, 0.0));
  use_context_provider(|| ContextMenuContext { root, point });

  rsx! { {children} }
}

/// The area that opens the menu at the pointer on a right click, or at the
/// area on the keyboard's context menu key.
#[component]
pub fn ContextMenuTrigger(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let context = use_context_menu("ContextMenuTrigger");
  let mut point = context.point;

  rsx! {
    div {
      class,
      "data-state": if context.root.is_open() { "open" } else { "closed" },
      oncontextmenu: move |event| {
        event.prevent_default();
        let position = event.client_coordinates();
        point.set((position.x, position.y));
        context.root.set_open.call(true);
      },
      ..attributes,
      {children}
    }
  }
}

/// The menu's corner is placed at the point the trigger was opened at,
/// flipping and shifting to stay in the viewport. Opening focuses the first
/// enabled item; arrows wrap, Home, End, and typeahead jump, and activating an
/// item closes the menu and returns focus. Escape and outside interactions
/// close it per `dismiss`.
#[component]
pub fn ContextMenuContent(
  #[props(default)] class: String,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Start)] align: OverlayAlign,
  #[props(default)] side_offset: i32,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let context = use_context_menu("ContextMenuContent");
  let root = context.root;
  let class = context_menu_content_class(&class);
  let open = root.is_open();
  let menu = use_listbox(open, None, ListboxMode::Menu, None, Some(root.set_open));
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement {
      anchor_id: None,
      anchor_point: Some((context.point)()),
      side,
      align,
      side_offset,
    },
    dismiss,
    Some(root.set_open),
  );

  rsx! {
    div {
      role: "menu",
      id: root.content_id(),
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
  let class = context_menu_item_class(inset, destructive, &with_density(density_control_class(use_density()), &class));

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
  let class = context_menu_checkbox_item_class(checked, &with_density(density_control_class(use_density()), &class));

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

/// Groups radio items and owns the checked item's value. Pass `value` to
/// control it, or `default_value` to start it; `on_value_change` hears every
/// change the user makes either way.
#[component]
pub fn ContextMenuRadioGroup(
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: Option<String>,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let group = use_menu_radio_group(value, default_value, on_value_change);

  rsx! {
    div {
      role: "group",
      class,
      "data-value": group.value(),
      {children}
    }
  }
}

/// A radio item: activation checks it in its group and calls `onclick`.
/// Shows a dot while checked.
#[component]
pub fn ContextMenuRadioItem(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let group = use_menu_radio_item("ContextMenuRadioItem", "ContextMenuRadioGroup");
  let checked = group.value().as_ref() == Some(&value);
  let class = context_menu_radio_item_class(checked, &with_density(density_control_class(use_density()), &class));

  rsx! {
    div {
      role: "menuitemradio",
      class,
      "aria-checked": checked.to_string(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-state": if checked { "checked" } else { "unchecked" },
      onclick: move |event| {
        if !disabled {
          group.choose(value.clone());
          if let Some(handler) = onclick {
            handler.call(event);
          }
        }
      },
      {children}
    }
  }
}

/// A nested menu (RFC 0067) that owns whether it is open. Put
/// `ContextMenuSubTrigger` and `ContextMenuSubContent` inside. Pass `open` to
/// control it, or `default_open` to start it; `on_open_change` hears every
/// change either way.
#[component]
pub fn ContextMenuSub(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  use_menu_sub(open, default_open, on_open_change);

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
  #[props(default)] inset: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let sub = use_menu_sub_part("ContextMenuSubTrigger", "ContextMenuSub");
  let open = sub.is_open();
  let class = context_menu_sub_trigger_class(inset, &with_density(density_control_class(use_density()), &class));

  rsx! {
    div {
      role: "menuitem",
      id: sub.trigger_id(),
      class,
      "aria-haspopup": "menu",
      "aria-expanded": open.to_string(),
      "aria-controls": sub.content_id(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-state": if open { "open" } else { "closed" },
      onclick: move |_| {
        if !disabled {
          sub.set_open.call(true);
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
pub fn ContextMenuSubContent(#[props(default)] class: String, children: Element) -> Element {
  let (sub, listbox, anchored) = use_menu_sub_content("ContextMenuSubContent", "ContextMenuSub");
  let open = sub.is_open();
  let class = context_menu_content_class(&class);

  rsx! {
    div {
      role: "menu",
      id: sub.content_id(),
      class,
      hidden: !open,
      "aria-labelledby": sub.trigger_id(),
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
