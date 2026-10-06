use super::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use super::element_id::next_element_id;
use super::listbox::{ListboxMode, use_listbox};
use super::menu_marks::{
  MENU_CHECKBOX_MARK_CLASS, MENU_RADIO_MARK_CLASS, MENU_SUB_TRIGGER_CLASS, menu_mark_state_class,
};
use super::menu_sub::{MenuSubContext, use_menu_sub, use_menu_sub_content};
pub use super::overlay::{DismissBehavior, DropdownPrimitiveConfig, OverlayAlign, OverlaySide};
use super::utils::classes;
use dioxus::prelude::*;

// Runs for the bar's lifetime. Triggers are read from the DOM on every event
// so triggers added or disabled later are picked up. Sends the `MenubarMenu`
// value of the menu to open.
// Keep in sync with `MENUBAR_SCRIPT` in the crate `menubar.rs`.
pub const MENUBAR_SCRIPT: &str = r#"
const scopeId = await dioxus.recv();
const root = document.querySelector(`[data-dxui-menubar="${scopeId}"]`);
if (!root) return;
const triggerSelector = "[data-dxui-menubar-trigger]";
const enabledTriggers = () =>
  Array.from(root.querySelectorAll(triggerSelector)).filter((trigger) => !trigger.disabled);
const menuOf = (element) => element.closest("[data-dxui-menubar-menu]");
// One Tab stop: the trigger that last had focus, or the first enabled one.
const setTabStop = (current) => {
  const enabled = enabledTriggers();
  const stop = enabled.includes(current) ? current : enabled[0];
  root.querySelectorAll(triggerSelector).forEach((trigger) => {
    trigger.tabIndex = trigger === stop ? 0 : -1;
  });
};
const step = (trigger, key) => {
  const enabled = enabledTriggers();
  const index = enabled.indexOf(trigger);
  const count = enabled.length;
  if (index < 0) return null;
  if (key === "ArrowRight") return enabled[(index + 1) % count];
  if (key === "ArrowLeft") return enabled[(index - 1 + count) % count];
  if (key === "Home") return enabled[0];
  if (key === "End") return enabled[count - 1];
  return null;
};
const open = (trigger) => {
  const menu = menuOf(trigger);
  if (menu) dioxus.send(menu.dataset.value || "");
};
// In a right-to-left layout, ArrowLeft points at the next item.
const visualKey = (key) => {
  if (getComputedStyle(root).direction !== "rtl") return key;
  if (key === "ArrowLeft") return "ArrowRight";
  if (key === "ArrowRight") return "ArrowLeft";
  return key;
};
const onKeyDown = (event) => {
  if (event.defaultPrevented || !(event.target instanceof Element)) return;
  const menu = menuOf(event.target);
  const trigger = menu && root.contains(menu) ? menu.querySelector(triggerSelector) : null;
  if (!trigger) return;
  const onTrigger = event.target === trigger;
  // Inside an open menu only Left and Right leave it; the menu itself
  // handles the other keys.
  if (!onTrigger && event.key !== "ArrowLeft" && event.key !== "ArrowRight") return;
  const next = step(trigger, visualKey(event.key));
  if (!next || next === trigger) return;
  event.preventDefault();
  if (onTrigger) next.focus();
  else open(next);
};
// Pointer movement, not pointerover: Chrome also sends pointerover when the
// layout shifts under a resting cursor, such as while a menu is placed, which
// would switch back to the menu under the cursor.
const onPointerMove = (event) => {
  const trigger = event.target instanceof Element ? event.target.closest(triggerSelector) : null;
  if (!trigger || trigger.disabled || !root.contains(trigger)) return;
  const content = menuOf(trigger)?.querySelector('[role="menu"]');
  if (content && content.hidden && root.querySelector('[role="menu"]:not([hidden])')) open(trigger);
};
const onFocusIn = (event) => {
  if (event.target instanceof Element && event.target.matches(triggerSelector)) setTabStop(event.target);
};
setTabStop(null);
root.addEventListener("keydown", onKeyDown);
root.addEventListener("pointermove", onPointerMove);
root.addEventListener("focusin", onFocusIn);
await new Promise((resolve) => {
  const observer = new MutationObserver(() => {
    if (!root.isConnected) {
      observer.disconnect();
      resolve();
    }
  });
  observer.observe(document.documentElement, { subtree: true, childList: true });
});
"#;

pub const MENUBAR_BASE_CLASS: &str =
  "flex h-10 items-center gap-1 rounded-md border border-border bg-background p-1";
pub const MENUBAR_MENU_BASE_CLASS: &str = "relative";
pub const MENUBAR_TRIGGER_BASE_CLASS: &str = "inline-flex h-8 items-center justify-center rounded-sm px-3 text-sm font-medium text-foreground transition-colors hover:bg-accent focus:bg-accent focus:outline-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const MENUBAR_CONTENT_BASE_CLASS: &str = "z-50 min-w-32 overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md";
pub const MENUBAR_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
pub const MENUBAR_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const MENUBAR_ITEM_INSET_CLASS: &str = "pl-8";
pub const MENUBAR_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-border";
pub const MENUBAR_SHORTCUT_BASE_CLASS: &str =
  "ml-auto text-xs tracking-normal text-muted-foreground";

pub fn menubar_class(class: &str) -> String {
  classes([Some(MENUBAR_BASE_CLASS), Some(class)])
}

pub fn menubar_menu_class(class: &str) -> String {
  classes([Some(MENUBAR_MENU_BASE_CLASS), Some(class)])
}

pub fn menubar_trigger_class(open: bool, class: &str) -> String {
  let state_class = if open { "bg-accent" } else { "" };

  classes([Some(MENUBAR_TRIGGER_BASE_CLASS), Some(state_class), Some(class)])
}

pub fn menubar_content_class(class: &str) -> String {
  classes([Some(MENUBAR_CONTENT_BASE_CLASS), Some(class)])
}

pub fn menubar_label_class(class: &str) -> String {
  classes([Some(MENUBAR_LABEL_BASE_CLASS), Some(class)])
}

pub fn menubar_item_class(inset: bool, destructive: bool, class: &str) -> String {
  let variant_class = if destructive {
    "text-destructive focus:bg-destructive/10 focus:text-destructive"
  } else {
    "text-foreground focus:bg-accent"
  };
  let inset_class = if inset { MENUBAR_ITEM_INSET_CLASS } else { "" };

  classes([Some(MENUBAR_ITEM_BASE_CLASS), Some(variant_class), Some(inset_class), Some(class)])
}

pub fn menubar_separator_class(class: &str) -> String {
  classes([Some(MENUBAR_SEPARATOR_BASE_CLASS), Some(class)])
}

/// An inset item with a check mark shown while `checked`.
pub fn menubar_checkbox_item_class(checked: bool, class: &str) -> String {
  let mark =
    classes([Some(MENU_CHECKBOX_MARK_CLASS), Some(menu_mark_state_class(checked)), Some(class)]);
  menubar_item_class(true, false, &mark)
}

/// An inset item with a dot shown while `checked`.
pub fn menubar_radio_item_class(checked: bool, class: &str) -> String {
  let mark =
    classes([Some(MENU_RADIO_MARK_CLASS), Some(menu_mark_state_class(checked)), Some(class)]);
  menubar_item_class(true, false, &mark)
}

/// A sub trigger: an item with a chevron at its end.
pub fn menubar_sub_trigger_class(inset: bool, class: &str) -> String {
  menubar_item_class(inset, false, &classes([Some(MENU_SUB_TRIGGER_CLASS), Some(class)]))
}

pub fn menubar_shortcut_class(class: &str) -> String {
  classes([Some(MENUBAR_SHORTCUT_BASE_CLASS), Some(class)])
}

/// The triggers form one Tab stop: Left, Right, Home, and End move focus
/// between enabled triggers. While a menu is open, Left or Right inside it, or
/// hovering another trigger, calls `on_value_change` with the `value` of the
/// `MenubarMenu` to open.
#[component]
pub fn Menubar(
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = menubar_class(&class);
  let scope_id = use_hook(|| format!("dxui-menubar-{}", next_element_id()));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let mut eval = document::eval(MENUBAR_SCRIPT);
    // A send error means the page already finished the script; nothing to track.
    let _ = eval.send(effect_scope_id.as_str());
    spawn(async move {
      while let Ok(value) = eval.recv::<String>().await {
        if let Some(handler) = on_value_change {
          handler.call(value);
        }
      }
    });
  });

  rsx! {
    div {
      role: "menubar",
      class,
      "data-dxui-menubar": scope_id,
      ..attributes,
      {children}
    }
  }
}

/// `value` identifies the menu in `Menubar`'s `on_value_change`.
#[component]
pub fn MenubarMenu(
  #[props(default)] value: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = menubar_menu_class(&class);

  rsx! {
    div {
      class,
      "data-dxui-menubar-menu": "",
      "data-value": value,
      {children}
    }
  }
}

/// Click requests `!open`; ArrowDown on a closed trigger requests `true`.
#[component]
pub fn MenubarTrigger(
  #[props(default)] id: Option<String>,
  #[props(default)] open: bool,
  #[props(default)] disabled: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = menubar_trigger_class(open, &class);

  rsx! {
    button {
      r#type: "button",
      role: "menuitem",
      id,
      class,
      disabled,
      "aria-expanded": open.to_string(),
      "aria-haspopup": "menu",
      "data-disabled": disabled.to_string(),
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-menubar-trigger": "",
      onclick: move |_| {
        if let Some(handler) = on_open_change {
          handler.call(!open);
        }
      },
      onkeydown: move |event| {
        let opens = !open && event.key() == Key::ArrowDown;
        if let Some(handler) = on_open_change.filter(|_| opens) {
          event.prevent_default();
          handler.call(true);
        }
      },
      {children}
    }
  }
}

/// Behaves like `DropdownContent`: opening focuses the first enabled item,
/// arrows, Home, End, and typeahead move focus, and activating an item
/// requests close and returns focus to the trigger named by `anchor_id`.
/// Escape and outside interactions request close per `dismiss`.
#[component]
pub fn MenubarContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] anchor_id: Option<String>,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Start)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let class = menubar_content_class(&class);
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
pub fn MenubarLabel(#[props(default)] class: String, children: Element) -> Element {
  let class = menubar_label_class(&class);

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
pub fn MenubarItem(
  #[props(default)] inset: bool,
  #[props(default)] destructive: bool,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = menubar_item_class(inset, destructive, &class);

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
pub fn MenubarCheckboxItem(
  #[props(default)] checked: bool,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = menubar_checkbox_item_class(checked, &class);

  rsx! {
    div {
      role: "menuitemcheckbox",
      class,
      "aria-checked": checked.to_string(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-state": if checked { "checked" } else { "unchecked" },
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
pub fn MenubarRadioGroup(
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
pub fn MenubarRadioItem(
  #[props(default)] checked: bool,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = menubar_radio_item_class(checked, &class);

  rsx! {
    div {
      role: "menuitemradio",
      class,
      "aria-checked": checked.to_string(),
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-state": if checked { "checked" } else { "unchecked" },
      onclick: move |event| {
        if let Some(handler) = onclick.filter(|_| !disabled) {
          handler.call(event);
        }
      },
      {children}
    }
  }
}

/// A nested menu (RFC 0067). Put `MenubarSubTrigger` and `MenubarSubContent`
/// inside and pass both the same `open`; `on_open_change` reports requests
/// to open and close it.
#[component]
pub fn MenubarSub(
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
pub fn MenubarSubTrigger(
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
  let class = menubar_sub_trigger_class(inset, &class);

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
pub fn MenubarSubContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let (context, listbox, anchored) = use_menu_sub_content(open);
  let id = context.as_ref().map(MenuSubContext::content_id);
  let labelledby = context.as_ref().map(MenuSubContext::trigger_id);
  let class = menubar_content_class(&class);

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
pub fn MenubarSeparator(#[props(default)] class: String) -> Element {
  let class = menubar_separator_class(&class);

  rsx! {
    div {
      role: "separator",
      class,
    }
  }
}

#[component]
pub fn MenubarShortcut(#[props(default)] class: String, children: Element) -> Element {
  let class = menubar_shortcut_class(&class);

  rsx! {
    span {
      class,
      {children}
    }
  }
}
