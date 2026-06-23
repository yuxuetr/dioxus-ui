use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::DropdownPrimitiveConfig;

pub const MENUBAR_BASE_CLASS: &str = "flex h-10 items-center gap-1 rounded-md border border-zinc-200 bg-white p-1";
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

  classes([
    Some(MENUBAR_ITEM_BASE_CLASS),
    Some(variant_class),
    Some(inset_class),
    Some(class),
  ])
}

pub fn menubar_separator_class(class: &str) -> String {
  classes([Some(MENUBAR_SEPARATOR_BASE_CLASS), Some(class)])
}

pub fn menubar_shortcut_class(class: &str) -> String {
  classes([Some(MENUBAR_SHORTCUT_BASE_CLASS), Some(class)])
}

#[component]
pub fn Menubar(#[props(default)] class: String, children: Element) -> Element {
  let class = menubar_class(&class);

  rsx! {
    div {
      role: "menubar",
      class,
      {children}
    }
  }
}

#[component]
pub fn MenubarMenu(#[props(default)] class: String, children: Element) -> Element {
  let class = menubar_menu_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn MenubarTrigger(
  #[props(default)] open: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = menubar_trigger_class(open, &class);

  rsx! {
    button {
      r#type: "button",
      role: "menuitem",
      class,
      disabled,
      "aria-expanded": open.to_string(),
      "data-disabled": disabled.to_string(),
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[component]
pub fn MenubarContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = menubar_content_class(&class);

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
pub fn MenubarLabel(#[props(default)] class: String, children: Element) -> Element {
  let class = menubar_label_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn MenubarItem(
  #[props(default)] inset: bool,
  #[props(default)] destructive: bool,
  #[props(default)] disabled: bool,
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
      {children}
    }
  }
}

#[component]
pub fn MenubarCheckboxItem(
  #[props(default)] checked: bool,
  #[props(default)] disabled: bool,
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
