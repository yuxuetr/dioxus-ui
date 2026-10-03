use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::DropdownPrimitiveConfig;

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

  classes([Some(CONTEXT_MENU_ITEM_BASE_CLASS), Some(variant_class), Some(inset_class), Some(class)])
}

pub fn context_menu_separator_class(class: &str) -> String {
  classes([Some(CONTEXT_MENU_SEPARATOR_BASE_CLASS), Some(class)])
}

pub fn context_menu_shortcut_class(class: &str) -> String {
  classes([Some(CONTEXT_MENU_SHORTCUT_BASE_CLASS), Some(class)])
}

#[component]
pub fn ContextMenuContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = context_menu_content_class(&class);

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
      {children}
    }
  }
}

#[component]
pub fn ContextMenuCheckboxItem(
  #[props(default)] checked: bool,
  #[props(default)] disabled: bool,
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn context_menu_item_class_reflects_destructive_and_inset_state() {
    let actual = context_menu_item_class(true, true, "gap-2");

    assert!(actual.contains(CONTEXT_MENU_ITEM_BASE_CLASS));
    assert!(actual.contains(CONTEXT_MENU_ITEM_INSET_CLASS));
    assert!(actual.contains("text-red-600 focus:bg-red-50 focus:text-red-700"));
    assert!(actual.ends_with("gap-2"));
  }

  #[test]
  fn context_menu_primitive_config_is_reexported() {
    let config = DropdownPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
