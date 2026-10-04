use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  DismissBehavior, DropdownPrimitiveConfig, OverlayAlign, OverlaySide,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::listbox::{ListboxMode, use_listbox};

static NEXT_MENUBAR_ID: AtomicUsize = AtomicUsize::new(0);

// Runs for the bar's lifetime. Triggers are read from the DOM on every event
// so triggers added or disabled later are picked up. Sends the `MenubarMenu`
// value of the menu to open.
// Keep in sync with `MENUBAR_SCRIPT` in the CLI `menubar.rs` template.
pub(crate) const MENUBAR_SCRIPT: &str = r#"
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
const onKeyDown = (event) => {
  if (event.defaultPrevented || !(event.target instanceof Element)) return;
  const menu = menuOf(event.target);
  const trigger = menu && root.contains(menu) ? menu.querySelector(triggerSelector) : null;
  if (!trigger) return;
  const onTrigger = event.target === trigger;
  // Inside an open menu only Left and Right leave it; the menu itself
  // handles the other keys.
  if (!onTrigger && event.key !== "ArrowLeft" && event.key !== "ArrowRight") return;
  const next = step(trigger, event.key);
  if (!next || next === trigger) return;
  event.preventDefault();
  if (onTrigger) next.focus();
  else open(next);
};
const onPointerOver = (event) => {
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
root.addEventListener("pointerover", onPointerOver);
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
  "flex h-10 items-center gap-1 rounded-md border border-zinc-200 bg-white p-1";
pub const MENUBAR_MENU_BASE_CLASS: &str = "relative";
pub const MENUBAR_TRIGGER_BASE_CLASS: &str = "inline-flex h-8 items-center justify-center rounded-sm px-3 text-sm font-medium text-zinc-900 transition-colors hover:bg-zinc-100 focus:bg-zinc-100 focus:outline-none data-disabled:pointer-events-none data-disabled:opacity-50";
pub const MENUBAR_CONTENT_BASE_CLASS: &str = "z-50 min-w-32 overflow-hidden rounded-md border border-zinc-200 bg-white p-1 text-zinc-950 shadow-md";
pub const MENUBAR_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-zinc-500";
pub const MENUBAR_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors data-disabled:pointer-events-none data-disabled:opacity-50";
pub const MENUBAR_ITEM_INSET_CLASS: &str = "pl-8";
pub const MENUBAR_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-zinc-200";
pub const MENUBAR_SHORTCUT_BASE_CLASS: &str = "ml-auto text-xs tracking-normal text-zinc-500";

pub fn menubar_class(class: &str) -> String {
  classes([Some(MENUBAR_BASE_CLASS), Some(class)])
}

pub fn menubar_menu_class(class: &str) -> String {
  classes([Some(MENUBAR_MENU_BASE_CLASS), Some(class)])
}

pub fn menubar_trigger_class(open: bool, class: &str) -> String {
  let state_class = if open { "bg-zinc-100" } else { "" };

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
    "text-red-600 focus:bg-red-50 focus:text-red-700"
  } else {
    "text-zinc-900 focus:bg-zinc-100"
  };
  let inset_class = if inset { MENUBAR_ITEM_INSET_CLASS } else { "" };

  classes([Some(MENUBAR_ITEM_BASE_CLASS), Some(variant_class), Some(inset_class), Some(class)])
}

pub fn menubar_separator_class(class: &str) -> String {
  classes([Some(MENUBAR_SEPARATOR_BASE_CLASS), Some(class)])
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
  children: Element,
) -> Element {
  let class = menubar_class(&class);
  let scope_id =
    use_hook(|| format!("dxui-menubar-{}", NEXT_MENUBAR_ID.fetch_add(1, Ordering::Relaxed)));
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
  let class = menubar_item_class(true, false, &class);

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
  let class = menubar_item_class(true, false, &class);

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn menubar_trigger_class_reflects_open_state() {
    let actual = menubar_trigger_class(true, "min-w-20");

    assert!(actual.contains(MENUBAR_TRIGGER_BASE_CLASS));
    assert!(actual.contains("bg-zinc-100"));
    assert!(actual.ends_with("min-w-20"));
  }

  #[test]
  fn menubar_item_class_reflects_destructive_and_inset_state() {
    let actual = menubar_item_class(true, true, "gap-2");

    assert!(actual.contains(MENUBAR_ITEM_BASE_CLASS));
    assert!(actual.contains(MENUBAR_ITEM_INSET_CLASS));
    assert!(actual.contains("text-red-600 focus:bg-red-50 focus:text-red-700"));
    assert!(actual.ends_with("gap-2"));
  }

  #[test]
  fn menubar_primitive_config_is_reexported() {
    let config = DropdownPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
