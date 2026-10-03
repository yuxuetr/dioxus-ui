use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::PopoverPrimitiveConfig;

pub const NAVIGATION_MENU_BASE_CLASS: &str =
  "relative z-10 flex max-w-max flex-1 items-center justify-center";
pub const NAVIGATION_MENU_LIST_BASE_CLASS: &str =
  "group flex flex-1 list-none items-center justify-center gap-1";
pub const NAVIGATION_MENU_ITEM_BASE_CLASS: &str = "relative";
pub const NAVIGATION_MENU_TRIGGER_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md bg-white px-4 py-2 text-sm font-medium text-zinc-900 transition-colors hover:bg-zinc-100 focus:bg-zinc-100 focus:outline-none disabled:pointer-events-none disabled:opacity-50";
pub const NAVIGATION_MENU_CONTENT_BASE_CLASS: &str = "left-0 top-0 w-full rounded-md border border-zinc-200 bg-white p-4 text-zinc-950 shadow-md md:absolute md:w-auto";
pub const NAVIGATION_MENU_LINK_BASE_CLASS: &str = "block select-none rounded-md p-3 text-sm leading-none text-zinc-900 no-underline outline-none transition-colors hover:bg-zinc-100 focus:bg-zinc-100 data-active:bg-zinc-100 data-disabled:pointer-events-none data-disabled:opacity-50";
pub const NAVIGATION_MENU_VIEWPORT_BASE_CLASS: &str = "absolute left-0 top-full flex h-[var(--navigation-menu-viewport-height)] w-full justify-center overflow-hidden rounded-md border border-zinc-200 bg-white text-zinc-950 shadow md:w-[var(--navigation-menu-viewport-width)]";
pub const NAVIGATION_MENU_INDICATOR_BASE_CLASS: &str =
  "top-full z-10 flex h-2 items-end justify-center overflow-hidden";

pub fn navigation_menu_class(class: &str) -> String {
  classes([Some(NAVIGATION_MENU_BASE_CLASS), Some(class)])
}

pub fn navigation_menu_list_class(class: &str) -> String {
  classes([Some(NAVIGATION_MENU_LIST_BASE_CLASS), Some(class)])
}

pub fn navigation_menu_item_class(class: &str) -> String {
  classes([Some(NAVIGATION_MENU_ITEM_BASE_CLASS), Some(class)])
}

pub fn navigation_menu_trigger_class(open: bool, class: &str) -> String {
  let state_class = if open { "bg-zinc-100" } else { "" };

  classes([Some(NAVIGATION_MENU_TRIGGER_BASE_CLASS), Some(state_class), Some(class)])
}

pub fn navigation_menu_content_class(class: &str) -> String {
  classes([Some(NAVIGATION_MENU_CONTENT_BASE_CLASS), Some(class)])
}

pub fn navigation_menu_link_class(active: bool, class: &str) -> String {
  let state_class = if active { "bg-zinc-100" } else { "" };

  classes([Some(NAVIGATION_MENU_LINK_BASE_CLASS), Some(state_class), Some(class)])
}

pub fn navigation_menu_viewport_class(class: &str) -> String {
  classes([Some(NAVIGATION_MENU_VIEWPORT_BASE_CLASS), Some(class)])
}

pub fn navigation_menu_indicator_class(open: bool, class: &str) -> String {
  let state_class = if open { "opacity-100" } else { "opacity-0" };

  classes([Some(NAVIGATION_MENU_INDICATOR_BASE_CLASS), Some(state_class), Some(class)])
}

#[component]
pub fn NavigationMenu(#[props(default)] class: String, children: Element) -> Element {
  let class = navigation_menu_class(&class);

  rsx! {
    nav {
      class,
      {children}
    }
  }
}

#[component]
pub fn NavigationMenuList(#[props(default)] class: String, children: Element) -> Element {
  let class = navigation_menu_list_class(&class);

  rsx! {
    ul {
      class,
      {children}
    }
  }
}

#[component]
pub fn NavigationMenuItem(#[props(default)] class: String, children: Element) -> Element {
  let class = navigation_menu_item_class(&class);

  rsx! {
    li {
      class,
      {children}
    }
  }
}

#[component]
pub fn NavigationMenuTrigger(
  #[props(default)] open: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = navigation_menu_trigger_class(open, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-expanded": open.to_string(),
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[component]
pub fn NavigationMenuContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = navigation_menu_content_class(&class);

  rsx! {
    div {
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[component]
pub fn NavigationMenuLink(
  #[props(default = String::from("#"))] href: String,
  #[props(default)] active: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = navigation_menu_link_class(active, &class);

  rsx! {
    a {
      href,
      class,
      "aria-current": if active { "page" } else { "false" },
      "aria-disabled": disabled.to_string(),
      "data-active": active.to_string(),
      "data-disabled": disabled.to_string(),
      {children}
    }
  }
}

#[component]
pub fn NavigationMenuViewport(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = navigation_menu_viewport_class(&class);

  rsx! {
    div {
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[component]
pub fn NavigationMenuIndicator(
  #[props(default)] open: bool,
  #[props(default)] class: String,
) -> Element {
  let class = navigation_menu_indicator_class(open, &class);

  rsx! {
    div {
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn navigation_menu_trigger_class_reflects_open_state() {
    let actual = navigation_menu_trigger_class(true, "min-w-24");

    assert!(actual.contains(NAVIGATION_MENU_TRIGGER_BASE_CLASS));
    assert!(actual.contains("bg-zinc-100"));
    assert!(actual.ends_with("min-w-24"));
  }

  #[test]
  fn navigation_menu_link_class_reflects_active_state() {
    let actual = navigation_menu_link_class(true, "font-semibold");

    assert!(actual.contains(NAVIGATION_MENU_LINK_BASE_CLASS));
    assert!(actual.contains("bg-zinc-100"));
    assert!(actual.ends_with("font-semibold"));
  }

  #[test]
  fn navigation_menu_primitive_config_is_reexported() {
    let config = PopoverPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
