//! Menubar: a horizontal bar of menus for application commands. It reuses the dropdown
//! primitive's defaults and shares item semantics with Context Menu.
use super::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use super::element_id::next_element_id;
use super::listbox::{ListboxMode, use_listbox};
use super::menu_marks::{
  MENU_CHECKBOX_MARK_CLASS, MENU_RADIO_MARK_CLASS, MENU_SUB_TRIGGER_CLASS, menu_mark_state_class,
};
use super::menu_radio::{use_menu_radio_group, use_menu_radio_item};
use super::menu_sub::{use_menu_sub, use_menu_sub_content, use_menu_sub_part};
pub use super::overlay::{DismissBehavior, DropdownPrimitiveConfig, OverlayAlign, OverlaySide};
use super::root_state::{Controllable, use_controllable, use_root_context};
use super::utils::{classes, merge_classes};
use super::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;

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

const MENUBAR_BASE_CLASS: &str =
  "flex h-10 items-center gap-1 rounded-md border border-border bg-background p-1";
const MENUBAR_MENU_BASE_CLASS: &str = "relative";
const MENUBAR_TRIGGER_BASE_CLASS: &str = "inline-flex h-8 items-center justify-center rounded-sm px-3 text-sm font-medium text-foreground transition-colors hover:bg-accent focus:bg-accent focus:outline-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
const MENUBAR_CONTENT_BASE_CLASS: &str = "z-50 min-w-32 overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md";
const MENUBAR_LABEL_BASE_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
const MENUBAR_ITEM_BASE_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
const MENUBAR_ITEM_INSET_CLASS: &str = "pl-8";
const MENUBAR_SEPARATOR_BASE_CLASS: &str = "-mx-1 my-1 h-px bg-border";
const MENUBAR_SHORTCUT_BASE_CLASS: &str =
  "ml-auto text-xs tracking-normal text-muted-foreground";

/// Classes for the bar that holds the menus, with `class` merged over them.
pub fn menubar_class(class: &str) -> String {
  merge_classes(classes([Some(MENUBAR_BASE_CLASS)]), class)
}

fn menubar_menu_class(class: &str) -> String {
  merge_classes(classes([Some(MENUBAR_MENU_BASE_CLASS)]), class)
}

/// Classes for the trigger: base classes, plus the accent background while `open`, with `class`
/// merged over them.
pub fn menubar_trigger_class(open: bool, class: &str) -> String {
  let state_class = if open { "bg-accent" } else { "" };

  merge_classes(classes([Some(MENUBAR_TRIGGER_BASE_CLASS), Some(state_class)]), class)
}

fn menubar_content_class(class: &str) -> String {
  merge_classes(classes([Some(MENUBAR_CONTENT_BASE_CLASS)]), class)
}

fn menubar_label_class(class: &str) -> String {
  merge_classes(classes([Some(MENUBAR_LABEL_BASE_CLASS)]), class)
}

/// Classes for the item: base classes, destructive or plain colors, and extra left padding when
/// `inset`, with `class` merged over them.
pub fn menubar_item_class(inset: bool, destructive: bool, class: &str) -> String {
  let variant_class = if destructive {
    "text-destructive focus:bg-destructive/10 focus:text-destructive"
  } else {
    "text-foreground focus:bg-accent"
  };
  let inset_class = if inset { MENUBAR_ITEM_INSET_CLASS } else { "" };

  merge_classes(
    classes([Some(MENUBAR_ITEM_BASE_CLASS), Some(variant_class), Some(inset_class)]),
    class,
  )
}

fn menubar_separator_class(class: &str) -> String {
  merge_classes(classes([Some(MENUBAR_SEPARATOR_BASE_CLASS)]), class)
}

/// An inset item with a check mark shown while `checked`.
pub fn menubar_checkbox_item_class(checked: bool, class: &str) -> String {
  let mark = merge_classes(
    classes([Some(MENU_CHECKBOX_MARK_CLASS), Some(menu_mark_state_class(checked))]),
    class,
  );
  menubar_item_class(true, false, &mark)
}

/// An inset item with a dot shown while `checked`.
pub fn menubar_radio_item_class(checked: bool, class: &str) -> String {
  let mark = merge_classes(
    classes([Some(MENU_RADIO_MARK_CLASS), Some(menu_mark_state_class(checked))]),
    class,
  );
  menubar_item_class(true, false, &mark)
}

/// A sub trigger: an item with a chevron at its end.
pub fn menubar_sub_trigger_class(inset: bool, class: &str) -> String {
  menubar_item_class(inset, false, &merge_classes(classes([Some(MENU_SUB_TRIGGER_CLASS)]), class))
}

fn menubar_shortcut_class(class: &str) -> String {
  merge_classes(classes([Some(MENUBAR_SHORTCUT_BASE_CLASS)]), class)
}

/// What a `Menubar` shares with its menus: the `value` of the open menu, or
/// the empty string while none is open.
#[derive(Clone, Copy)]
struct MenubarContext(Controllable<String>);

/// What a `MenubarMenu` shares with its trigger and content.
#[derive(Clone)]
struct MenubarMenuContext {
  bar: Controllable<String>,
  value: String,
  id: usize,
  set_open: Callback<bool>,
}

impl MenubarMenuContext {
  fn is_open(&self) -> bool {
    self.bar.get() == self.value
  }

  fn trigger_id(&self) -> String {
    format!("dxui-menubar-menu-{}-trigger", self.id)
  }

  fn content_id(&self) -> String {
    format!("dxui-menubar-menu-{}-content", self.id)
  }
}

fn use_menubar_menu(part: &str) -> MenubarMenuContext {
  use_root_context::<MenubarMenuContext>(part, "MenubarMenu")
}

/// The root of a menubar: it owns which menu is open, by the menu's `value`,
/// or the empty string while none is. Pass `value` to control it, or
/// `default_value` to start it; `on_value_change` hears every change the user
/// makes either way. The triggers form one Tab stop: Left, Right, Home, and
/// End move focus between enabled triggers. While a menu is open, Left or
/// Right inside it, or hovering another trigger, opens that menu instead.
#[component]
pub fn Menubar(
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: String,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = menubar_class(&class);
  let bar = use_controllable(move || value.cloned(), move || default_value, on_value_change);
  use_context_provider(|| MenubarContext(bar));
  let scope_id = use_hook(|| format!("dxui-menubar-{}", next_element_id()));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let mut eval = document::eval(MENUBAR_SCRIPT);
    // A send error means the page already finished the script; nothing to track.
    let _ = eval.send(effect_scope_id.as_str());
    spawn(async move {
      while let Ok(value) = eval.recv::<String>().await {
        bar.set(value);
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

/// One menu of the bar. `value` names it in the `Menubar`'s value; without
/// one it gets a generated name.
#[component]
pub fn MenubarMenu(
  #[props(default)] value: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let bar = use_root_context::<MenubarContext>("MenubarMenu", "Menubar").0;
  let class = menubar_menu_class(&class);
  let id = use_hook(next_element_id);
  let value = if value.is_empty() { format!("menu-{id}") } else { value };
  let own = value.clone();
  // Closing clears the bar only while this menu is the open one, so a menu
  // closing as another opens leaves the other open.
  let set_open = use_callback(move |next: bool| {
    if next {
      bar.set(own.clone());
    } else if bar.get() == own {
      bar.set(String::new());
    }
  });
  use_context_provider(|| MenubarMenuContext { bar, value: value.clone(), id, set_open });

  rsx! {
    div {
      class,
      "data-dxui-menubar-menu": "",
      "data-value": value,
      {children}
    }
  }
}

/// Click toggles its menu; ArrowDown on a closed trigger opens it.
#[component]
pub fn MenubarTrigger(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let menu = use_menubar_menu("MenubarTrigger");
  let open = menu.is_open();
  let set_open = menu.set_open;
  let class = menubar_trigger_class(open, &with_density(density_control_class(use_density()), &class));

  rsx! {
    button {
      r#type: "button",
      role: "menuitem",
      id: menu.trigger_id(),
      class,
      disabled,
      "aria-expanded": open.to_string(),
      "aria-haspopup": "menu",
      // A menu may have no content, such as a disabled one, so the trigger
      // names the content only while it is open.
      "aria-controls": open.then(|| menu.content_id()),
      "data-disabled": disabled.to_string(),
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-menubar-trigger": "",
      onclick: move |_| set_open.call(!open),
      onkeydown: move |event| {
        if !open && event.key() == Key::ArrowDown {
          event.prevent_default();
          set_open.call(true);
        }
      },
      {children}
    }
  }
}

/// Behaves like `DropdownContent`: opening focuses the first enabled item,
/// arrows, Home, End, and typeahead move focus, and activating an item closes
/// the menu and returns focus to its trigger. Escape and outside interactions
/// close it per `dismiss`.
#[component]
pub fn MenubarContent(
  #[props(default)] class: String,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Start)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let menu = use_menubar_menu("MenubarContent");
  let class = menubar_content_class(&class);
  let open = menu.is_open();
  let anchor_id = Some(menu.trigger_id());
  let listbox = use_listbox(open, anchor_id.clone(), ListboxMode::Menu, None, Some(menu.set_open));
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    Some(menu.set_open),
  );

  rsx! {
    div {
      role: "menu",
      id: menu.content_id(),
      class,
      hidden: !open,
      "aria-labelledby": menu.trigger_id(),
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      "data-dxui-listbox": listbox,
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
  let class = menubar_item_class(inset, destructive, &with_density(density_control_class(use_density()), &class));

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
  let class = menubar_checkbox_item_class(checked, &with_density(density_control_class(use_density()), &class));

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

/// Groups radio items and owns the checked item's value. Pass `value` to
/// control it, or `default_value` to start it; `on_value_change` hears every
/// change the user makes either way.
#[component]
pub fn MenubarRadioGroup(
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
pub fn MenubarRadioItem(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let group = use_menu_radio_item("MenubarRadioItem", "MenubarRadioGroup");
  let checked = group.value().as_ref() == Some(&value);
  let class = menubar_radio_item_class(checked, &with_density(density_control_class(use_density()), &class));

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
/// `MenubarSubTrigger` and `MenubarSubContent` inside. Pass `open` to
/// control it, or `default_open` to start it; `on_open_change` hears every
/// change either way.
#[component]
pub fn MenubarSub(
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
pub fn MenubarSubTrigger(
  #[props(default)] inset: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let sub = use_menu_sub_part("MenubarSubTrigger", "MenubarSub");
  let open = sub.is_open();
  let class = menubar_sub_trigger_class(inset, &with_density(density_control_class(use_density()), &class));

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
pub fn MenubarSubContent(#[props(default)] class: String, children: Element) -> Element {
  let (sub, listbox, anchored) = use_menu_sub_content("MenubarSubContent", "MenubarSub");
  let open = sub.is_open();
  let class = menubar_content_class(&class);

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
