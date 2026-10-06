use super::element_id::next_element_id;
pub use super::overlay::PopoverPrimitiveConfig;
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

// Runs for the menu's lifetime and reads items from the DOM on every event.
// Sends the `NavigationMenuItem` value to open, or an empty string to close.
// Keep in sync with `NAVIGATION_MENU_SCRIPT` in the crate `navigation_menu.rs`.
pub const NAVIGATION_MENU_SCRIPT: &str = r#"
const scopeId = await dioxus.recv();
const root = document.querySelector(`[data-dxui-navigation-menu="${scopeId}"]`);
if (!root) return;
const itemSelector = "[data-dxui-navigation-item]";
const triggerSelector = "[data-dxui-navigation-trigger]";
const contentSelector = "[data-dxui-navigation-content]";
// A vertical menu is a submenu inside another menu's content (RFC 0063).
const vertical = root.dataset.orientation === "vertical";
const openDelay = vertical ? 0 : 200;
const closeDelay = 300;
const enabled = (element) => !element.disabled && element.getAttribute("aria-disabled") !== "true";
const inside = (target) => target instanceof Node && root.contains(target);
// An element belongs to the menu whose root is its closest menu ancestor, so
// a nested menu's items are left to its own script.
const owned = (element) => element.closest("[data-dxui-navigation-menu]") === root;
const ownedIn = (scope, selector) => Array.from(scope.querySelectorAll(selector)).filter(owned);
const ownContent = (element) => {
  const content = element.closest(contentSelector);
  return content && owned(content) ? content : null;
};
// The closest item this menu owns, skipping a nested menu's items.
const ownItem = (element) => {
  let item = element.closest(itemSelector);
  while (item && !owned(item)) item = item.parentElement ? item.parentElement.closest(itemSelector) : null;
  return item;
};
// A vertical menu's contents sit beside its list and pair with items by value.
const itemOf = (element) => {
  const item = ownItem(element);
  if (item) return item;
  const content = ownContent(element);
  if (!content) return null;
  return ownedIn(root, itemSelector).find((candidate) => candidate.dataset.value === content.dataset.value) || null;
};
const triggerOf = (item) => ownedIn(item, triggerSelector)[0] || null;
const contentOf = (item) =>
  ownedIn(item, contentSelector)[0] ||
  ownedIn(root, contentSelector).find((content) => !ownItem(content) && content.dataset.value === item.dataset.value) ||
  null;
const linksOf = (content) => ownedIn(content, "a").filter(enabled);
// Top-level items are triggers and links outside content.
const topLevel = () =>
  ownedIn(root, `${triggerSelector}, a`).filter((element) => enabled(element) && !ownContent(element));
const openItem = () => {
  const content = ownedIn(root, contentSelector).find((element) => !element.hidden);
  return content ? itemOf(content) : null;
};
const open = (item) => dioxus.send(item.dataset.value || "");
const close = () => dioxus.send("");
const step = (list, current, key, nextKey, previousKey) => {
  const index = list.indexOf(current);
  if (index < 0) return null;
  if (key === nextKey) return list[(index + 1) % list.length];
  if (key === previousKey) return list[(index - 1 + list.length) % list.length];
  if (key === "Home") return list[0];
  if (key === "End") return list[list.length - 1];
  return null;
};
// ArrowDown on a closed trigger opens it; its first link takes focus once
// the content is shown.
let pendingFocus = null;
const focusFirstLink = (item) => {
  const content = contentOf(item);
  const first = content ? linksOf(content)[0] : null;
  if (first) first.focus();
};
// In a right-to-left layout, ArrowLeft points at the next item.
const visualKey = (key) => {
  if (getComputedStyle(root).direction !== "rtl") return key;
  if (key === "ArrowLeft") return "ArrowRight";
  if (key === "ArrowRight") return "ArrowLeft";
  return key;
};
// A horizontal menu enters content with ArrowDown and moves along with Left
// and Right; a vertical one enters with the arrow pointing at its panels and
// moves with Down and Up.
const enterKey = vertical ? "ArrowRight" : "ArrowDown";
const onKeyDown = (event) => {
  if (event.defaultPrevented || !(event.target instanceof Element)) return;
  const target = event.target;
  const key = visualKey(event.key);
  const content = ownContent(target);
  let next = null;
  if (content) {
    if (vertical && key === "ArrowLeft") {
      next = triggerOf(itemOf(content));
    } else {
      next = step(linksOf(content), target, event.key, "ArrowDown", "ArrowUp");
    }
  } else if (target.matches(triggerSelector) && owned(target) && key === enterKey) {
    event.preventDefault();
    const item = itemOf(target);
    const itemContent = contentOf(item);
    if (itemContent && !itemContent.hidden) {
      focusFirstLink(item);
    } else {
      pendingFocus = item;
      open(item);
    }
    return;
  } else if (vertical) {
    next = step(topLevel(), target, event.key, "ArrowDown", "ArrowUp");
  } else {
    next = step(topLevel(), target, key, "ArrowRight", "ArrowLeft");
  }
  if (next) {
    event.preventDefault();
    next.focus();
  }
};
let openTimer = 0;
let pendingOpen = null;
let closeTimer = 0;
// A trigger closed by a click stays closed under the pointer until it leaves.
let clickClosed = null;
const cancelOpen = () => {
  clearTimeout(openTimer);
  pendingOpen = null;
};
const cancelClose = () => {
  clearTimeout(closeTimer);
  closeTimer = 0;
};
const onClick = (event) => {
  if (!(event.target instanceof Element) || !inside(event.target)) return;
  const trigger = event.target.closest(triggerSelector);
  if (trigger && owned(trigger) && enabled(trigger)) {
    const item = itemOf(trigger);
    cancelOpen();
    if (item === openItem()) {
      clickClosed = item;
      close();
    } else {
      open(item);
    }
    return;
  }
  const link = event.target.closest("a");
  if (link && enabled(link) && link.closest(contentSelector)) close();
};
// Pointer movement, not pointerover: Chrome also sends pointerover when the
// layout shifts under a resting cursor, such as when content opens.
const onPointerMove = (event) => {
  if (event.pointerType !== "mouse") return;
  const target = event.target instanceof Element && inside(event.target) ? event.target : null;
  const closestTrigger = target ? target.closest(triggerSelector) : null;
  const trigger = closestTrigger && owned(closestTrigger) ? closestTrigger : null;
  const item = target ? itemOf(target) : null;
  const current = openItem();
  if (clickClosed && trigger !== triggerOf(clickClosed)) clickClosed = null;
  if (trigger && enabled(trigger)) {
    cancelClose();
    if (item === current || item === clickClosed || item === pendingOpen) return;
    cancelOpen();
    pendingOpen = item;
    openTimer = setTimeout(() => {
      pendingOpen = null;
      open(item);
    }, current ? 0 : openDelay);
    return;
  }
  cancelOpen();
  if (current && item === current && ownContent(target)) return cancelClose();
  // A vertical menu keeps its panel; leaving it would leave an empty area.
  if (current && !closeTimer && !vertical) {
    closeTimer = setTimeout(() => {
      closeTimer = 0;
      close();
    }, closeDelay);
  }
};
const onDocumentKeyDown = (event) => {
  const item = event.key === "Escape" ? openItem() : null;
  if (!item) return;
  const focusInside = inside(document.activeElement);
  close();
  if (focusInside) triggerOf(item)?.focus();
};
const onOutside = (event) => {
  if (!vertical && openItem() && !inside(event.target)) close();
};
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const observer = new MutationObserver(() => {
  if (!root.isConnected) return finish();
  const content = pendingFocus ? contentOf(pendingFocus) : null;
  if (content && !content.hidden) {
    const item = pendingFocus;
    pendingFocus = null;
    focusFirstLink(item);
  }
});
observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["hidden"] });
root.addEventListener("keydown", onKeyDown);
root.addEventListener("click", onClick);
document.addEventListener("pointermove", onPointerMove);
document.addEventListener("keydown", onDocumentKeyDown);
document.addEventListener("pointerdown", onOutside, true);
document.addEventListener("focusin", onOutside);
await ended;
observer.disconnect();
cancelOpen();
cancelClose();
document.removeEventListener("pointermove", onPointerMove);
document.removeEventListener("keydown", onDocumentKeyDown);
document.removeEventListener("pointerdown", onOutside, true);
document.removeEventListener("focusin", onOutside);
"#;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NavigationMenuOrientation {
  #[default]
  Horizontal,
  /// A submenu inside another menu's content: triggers in a column, with
  /// their contents beside the list (RFC 0063).
  Vertical,
}

impl NavigationMenuOrientation {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

// The parts read a vertical menu's orientation through the
// `navigation-menu` group.
pub const NAVIGATION_MENU_BASE_CLASS: &str = "group/navigation-menu relative z-10 flex max-w-max flex-1 items-center justify-center data-[orientation=vertical]:max-w-none data-[orientation=vertical]:items-start data-[orientation=vertical]:justify-start data-[orientation=vertical]:gap-4";
pub const NAVIGATION_MENU_LIST_BASE_CLASS: &str = "group flex flex-1 list-none items-center justify-center gap-1 group-data-[orientation=vertical]/navigation-menu:flex-none group-data-[orientation=vertical]/navigation-menu:flex-col group-data-[orientation=vertical]/navigation-menu:items-stretch";
pub const NAVIGATION_MENU_ITEM_BASE_CLASS: &str = "relative";
pub const NAVIGATION_MENU_TRIGGER_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md px-4 py-2 text-sm font-medium text-foreground transition-colors hover:bg-accent focus:bg-accent focus:outline-none disabled:pointer-events-none disabled:opacity-50 group-data-[orientation=vertical]/navigation-menu:w-full group-data-[orientation=vertical]/navigation-menu:justify-start";
pub const NAVIGATION_MENU_CONTENT_BASE_CLASS: &str = "left-0 top-full mt-1.5 w-full rounded-md border border-border bg-popover p-4 text-popover-foreground shadow-md md:absolute md:w-auto group-data-[orientation=vertical]/navigation-menu:static group-data-[orientation=vertical]/navigation-menu:mt-0 group-data-[orientation=vertical]/navigation-menu:border-0 group-data-[orientation=vertical]/navigation-menu:p-0 group-data-[orientation=vertical]/navigation-menu:shadow-none";
pub const NAVIGATION_MENU_LINK_BASE_CLASS: &str = "block select-none rounded-md p-3 text-sm leading-none text-foreground no-underline outline-none transition-colors hover:bg-accent focus:bg-accent data-[active=true]:bg-accent data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const NAVIGATION_MENU_VIEWPORT_BASE_CLASS: &str = "absolute left-0 top-full flex h-[var(--navigation-menu-viewport-height)] w-full justify-center overflow-hidden rounded-md border border-border bg-popover text-popover-foreground shadow md:w-[var(--navigation-menu-viewport-width)]";
pub const NAVIGATION_MENU_INDICATOR_BASE_CLASS: &str =
  "top-full z-10 flex h-2 items-end justify-center overflow-hidden";

pub fn navigation_menu_class(class: &str) -> String {
  merge_classes(classes([Some(NAVIGATION_MENU_BASE_CLASS)]), class)
}

pub fn navigation_menu_list_class(class: &str) -> String {
  merge_classes(classes([Some(NAVIGATION_MENU_LIST_BASE_CLASS)]), class)
}

pub fn navigation_menu_item_class(class: &str) -> String {
  merge_classes(classes([Some(NAVIGATION_MENU_ITEM_BASE_CLASS)]), class)
}

pub fn navigation_menu_trigger_class(open: bool, class: &str) -> String {
  let state_class = if open { "bg-accent" } else { "bg-background" };

  merge_classes(classes([Some(NAVIGATION_MENU_TRIGGER_BASE_CLASS), Some(state_class)]), class)
}

pub fn navigation_menu_content_class(class: &str) -> String {
  merge_classes(classes([Some(NAVIGATION_MENU_CONTENT_BASE_CLASS)]), class)
}

pub fn navigation_menu_link_class(active: bool, class: &str) -> String {
  let state_class = if active { "bg-accent" } else { "" };

  merge_classes(classes([Some(NAVIGATION_MENU_LINK_BASE_CLASS), Some(state_class)]), class)
}

pub fn navigation_menu_viewport_class(class: &str) -> String {
  merge_classes(classes([Some(NAVIGATION_MENU_VIEWPORT_BASE_CLASS)]), class)
}

pub fn navigation_menu_indicator_class(open: bool, class: &str) -> String {
  let state_class = if open { "opacity-100" } else { "opacity-0" };

  merge_classes(classes([Some(NAVIGATION_MENU_INDICATOR_BASE_CLASS), Some(state_class)]), class)
}

/// Follows the disclosure navigation pattern: a click, Enter, or Space on a
/// trigger toggles its content, the mouse resting on a trigger opens it, and
/// leaving the open item closes it. Left, Right, Home, and End move between
/// top-level triggers and links; ArrowDown enters content, where ArrowDown,
/// ArrowUp, Home, and End move between links. Escape, an outside press, focus
/// leaving the menu, or a click on a content link closes it.
///
/// Every request calls `on_value_change` with the `NavigationMenuItem` value
/// to open, or an empty string to close.
///
/// For a submenu, nest a `NavigationMenu` with
/// `NavigationMenuOrientation::Vertical` in a content: its triggers form a
/// column, its contents sit beside its list with their item's `value`,
/// ArrowDown and ArrowUp move between triggers, and ArrowRight enters a panel
/// (RFC 0063).
#[component]
pub fn NavigationMenu(
  #[props(default)] orientation: NavigationMenuOrientation,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = nav)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = navigation_menu_class(&class);
  let scope_id = use_hook(|| format!("dxui-navigation-menu-{}", next_element_id()));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let mut eval = document::eval(NAVIGATION_MENU_SCRIPT);
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
    nav {
      class,
      "data-orientation": orientation.as_str(),
      "data-dxui-navigation-menu": scope_id,
      ..attributes,
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

/// `value` identifies the item in `NavigationMenu`'s `on_value_change`.
#[component]
pub fn NavigationMenuItem(
  #[props(default)] value: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = navigation_menu_item_class(&class);

  rsx! {
    li {
      class,
      "data-dxui-navigation-item": "",
      "data-value": value,
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
      "data-dxui-navigation-trigger": "",
      {children}
    }
  }
}

/// In a vertical menu, place contents beside the list and give each its
/// item's `value`; in a horizontal menu, place each inside its item.
#[component]
pub fn NavigationMenuContent(
  #[props(default)] open: bool,
  #[props(default)] value: Option<String>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = navigation_menu_content_class(&class);

  rsx! {
    div {
      class,
      hidden: !open,
      "data-value": value,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-navigation-content": "",
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
