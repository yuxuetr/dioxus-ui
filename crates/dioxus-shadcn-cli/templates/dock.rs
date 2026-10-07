//! Dock: a bottom navigation bar for phone layouts, with an icon and a label per
//! destination.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

const DOCK_BASE_CLASS: &str = "z-40 flex h-16 items-stretch border-t border-border bg-background pb-[env(safe-area-inset-bottom)] text-foreground";
const DOCK_FIXED_CLASS: &str = "fixed inset-x-0 bottom-0";
const DOCK_STATIC_CLASS: &str = "relative w-full";
// The active item shows a primary bar above its icon; the text stays the
// foreground color, which every theme keeps readable.
const DOCK_ITEM_BASE_CLASS: &str = "relative flex flex-1 flex-col items-center justify-center gap-1 transition-colors after:absolute after:top-0 after:h-0.5 after:w-8 after:rounded-full after:bg-primary hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset [&_svg]:size-5";
const DOCK_ITEM_INACTIVE_CLASS: &str = "text-muted-foreground after:opacity-0";
const DOCK_ITEM_ACTIVE_CLASS: &str = "text-foreground after:opacity-100";
const DOCK_LABEL_CLASS: &str = "text-xs font-medium";

/// Classes for the bar: base classes, fixed to the viewport bottom or in the flow by
/// `fixed`, then `class` merged over them.
pub fn dock_class(fixed: bool, class: &str) -> String {
  let position = if fixed { DOCK_FIXED_CLASS } else { DOCK_STATIC_CLASS };
  merge_classes(classes([Some(DOCK_BASE_CLASS), Some(position)]), class)
}

/// Classes for an item: base classes, the active state's bar and color or the
/// inactive muted color, then `class` merged over them.
pub fn dock_item_class(active: bool, class: &str) -> String {
  let state = if active { DOCK_ITEM_ACTIVE_CLASS } else { DOCK_ITEM_INACTIVE_CLASS };
  merge_classes(classes([Some(DOCK_ITEM_BASE_CLASS), Some(state)]), class)
}

/// Classes for an item's label: small medium text, then `class` merged over it.
pub fn dock_label_class(class: &str) -> String {
  merge_classes(classes([Some(DOCK_LABEL_CLASS)]), class)
}

/// A bottom navigation bar, fixed to the viewport and padded for the home
/// indicator; `fixed: false` keeps it in the flow. Name it with `aria-label`.
#[component]
pub fn Dock(
  #[props(default = true)] fixed: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = dock_class(fixed, &class);

  rsx! {
    nav { class, ..attributes, {children} }
  }
}

/// A link with `href`, a button without. `active` sets `aria-current="page"`.
/// For the router's `Link`, apply `dock_item_class` to it instead.
#[component]
pub fn DockItem(
  #[props(default)] href: Option<String>,
  #[props(default)] active: bool,
  #[props(default)] class: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = dock_item_class(active, &class);
  let current = active.then_some("page");
  let click = move |event: MouseEvent| {
    if let Some(handler) = onclick {
      handler.call(event);
    }
  };

  rsx! {
    if let Some(href) = href {
      a { class, href, "aria-current": current, onclick: click, ..attributes, {children} }
    } else {
      button { class, r#type: "button", "aria-current": current, onclick: click, ..attributes, {children} }
    }
  }
}

#[component]
pub fn DockLabel(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = dock_label_class(&class);

  rsx! {
    span { class, ..attributes, {children} }
  }
}
