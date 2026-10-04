use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{SidebarState, sidebar_toggle};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SidebarSide {
  #[default]
  Left,
  Right,
}

pub const SIDEBAR_BASE_CLASS: &str = "flex h-full w-64 flex-col border-zinc-200 bg-white text-zinc-950 transition-[width] data-collapsed:w-14 data-side-left:border-r data-side-right:border-l";
pub const SIDEBAR_RAIL_BASE_CLASS: &str = "absolute inset-y-0 z-10 hidden w-3 -translate-x-1/2 transition-colors hover:bg-zinc-100 data-collapsed:block";
pub const SIDEBAR_HEADER_BASE_CLASS: &str =
  "flex min-h-14 items-center gap-2 border-b border-zinc-200 px-3";
pub const SIDEBAR_CONTENT_BASE_CLASS: &str = "flex-1 overflow-auto p-2";
pub const SIDEBAR_FOOTER_BASE_CLASS: &str = "border-t border-zinc-200 p-2";
pub const SIDEBAR_GROUP_BASE_CLASS: &str = "grid gap-1 py-2";
pub const SIDEBAR_GROUP_LABEL_BASE_CLASS: &str = "px-2 py-1 text-xs font-medium text-zinc-500";
pub const SIDEBAR_ITEM_BASE_CLASS: &str = "flex min-h-9 items-center gap-2 rounded-md px-2 text-sm transition-colors hover:bg-zinc-100 data-active:bg-zinc-100 data-active:text-zinc-950 data-disabled:pointer-events-none data-disabled:opacity-50";
pub const SIDEBAR_TRIGGER_BASE_CLASS: &str = "inline-flex h-9 w-9 items-center justify-center rounded-md text-sm transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";

pub fn sidebar_side_attribute(side: SidebarSide) -> &'static str {
  match side {
    SidebarSide::Left => "left",
    SidebarSide::Right => "right",
  }
}

pub fn sidebar_class(collapsed: bool, side: SidebarSide, class: &str) -> String {
  let side_class = match side {
    SidebarSide::Left => "border-r",
    SidebarSide::Right => "border-l",
  };

  classes([Some(SIDEBAR_BASE_CLASS), Some(side_class), collapsed.then_some("w-14"), Some(class)])
}

pub fn sidebar_rail_class(collapsed: bool, class: &str) -> String {
  classes([Some(SIDEBAR_RAIL_BASE_CLASS), collapsed.then_some("block"), Some(class)])
}

pub fn sidebar_header_class(class: &str) -> String {
  classes([Some(SIDEBAR_HEADER_BASE_CLASS), Some(class)])
}

pub fn sidebar_content_class(class: &str) -> String {
  classes([Some(SIDEBAR_CONTENT_BASE_CLASS), Some(class)])
}

pub fn sidebar_footer_class(class: &str) -> String {
  classes([Some(SIDEBAR_FOOTER_BASE_CLASS), Some(class)])
}

pub fn sidebar_group_class(class: &str) -> String {
  classes([Some(SIDEBAR_GROUP_BASE_CLASS), Some(class)])
}

pub fn sidebar_group_label_class(class: &str) -> String {
  classes([Some(SIDEBAR_GROUP_LABEL_BASE_CLASS), Some(class)])
}

pub fn sidebar_item_class(active: bool, disabled: bool, class: &str) -> String {
  classes([
    Some(SIDEBAR_ITEM_BASE_CLASS),
    active.then_some("bg-zinc-100 text-zinc-950"),
    disabled.then_some("pointer-events-none opacity-50"),
    Some(class),
  ])
}

pub fn sidebar_trigger_class(class: &str) -> String {
  classes([Some(SIDEBAR_TRIGGER_BASE_CLASS), Some(class)])
}

#[component]
pub fn Sidebar(
  #[props(default)] collapsed: bool,
  #[props(default)] side: SidebarSide,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = aside)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = sidebar_class(collapsed, side, &class);

  rsx! {
    aside {
      class,
      "data-collapsed": collapsed.to_string(),
      "data-side": sidebar_side_attribute(side),
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn SidebarRail(#[props(default)] collapsed: bool, #[props(default)] class: String) -> Element {
  let class = sidebar_rail_class(collapsed, &class);

  rsx! {
    div {
      class,
      "aria-hidden": "true",
      "data-collapsed": collapsed.to_string(),
    }
  }
}

#[component]
pub fn SidebarHeader(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = sidebar_header_class(&class);

  rsx! {
    div {
      class,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn SidebarContent(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = sidebar_content_class(&class);

  rsx! {
    div {
      class,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn SidebarFooter(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = sidebar_footer_class(&class);

  rsx! {
    div {
      class,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn SidebarGroup(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = sidebar_group_class(&class);

  rsx! {
    div {
      class,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn SidebarGroupLabel(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = sidebar_group_label_class(&class);

  rsx! {
    div {
      class,
      ..attributes,
      {children}
    }
  }
}

/// Renders a link with an `href`, a button with an `onclick`, and otherwise a
/// wrapper for a link or button the app renders inside.
#[component]
pub fn SidebarItem(
  #[props(default)] active: bool,
  #[props(default)] disabled: bool,
  #[props(default)] href: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = sidebar_item_class(active, disabled, &class);
  let aria_current = active.then_some("page");
  let has_onclick = onclick.is_some();
  let onclick = move |event: MouseEvent| {
    if !disabled && let Some(handler) = onclick {
      handler.call(event);
    }
  };

  if !href.is_empty() {
    rsx! {
      a {
        class,
        href: (!disabled).then_some(href),
        role: disabled.then_some("link"),
        "aria-current": aria_current,
        "aria-disabled": disabled.to_string(),
        "data-active": active.to_string(),
        "data-disabled": disabled.to_string(),
        onclick,
        ..attributes,
        {children}
      }
    }
  } else if has_onclick {
    rsx! {
      button {
        r#type: "button",
        class,
        disabled,
        "aria-current": aria_current,
        "data-active": active.to_string(),
        "data-disabled": disabled.to_string(),
        onclick,
        ..attributes,
        {children}
      }
    }
  } else {
    rsx! {
      div {
        class,
        "aria-disabled": disabled.to_string(),
        "data-active": active.to_string(),
        "data-disabled": disabled.to_string(),
        ..attributes,
        {children}
      }
    }
  }
}

#[component]
pub fn SidebarTrigger(
  #[props(default)] collapsed: bool,
  #[props(default)] disabled: bool,
  #[props(default)] on_collapsed_change: Option<EventHandler<bool>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = sidebar_trigger_class(&class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-expanded": (!collapsed).to_string(),
      "data-collapsed": collapsed.to_string(),
      onclick: move |_| {
        if !disabled && let Some(handler) = on_collapsed_change {
          handler.call(!collapsed);
        }
      },
      ..attributes,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sidebar_class_reflects_collapsed_and_side() {
    let actual = sidebar_class(true, SidebarSide::Right, "shrink-0");

    assert!(actual.contains(SIDEBAR_BASE_CLASS));
    assert!(actual.contains("border-l"));
    assert!(actual.contains("w-14"));
    assert!(actual.ends_with("shrink-0"));
  }

  #[test]
  fn sidebar_item_class_reflects_states() {
    let actual = sidebar_item_class(true, true, "font-medium");

    assert!(actual.contains("bg-zinc-100 text-zinc-950"));
    assert!(actual.contains("pointer-events-none opacity-50"));
    assert!(actual.ends_with("font-medium"));
  }

  #[test]
  fn sidebar_primitives_are_reexported() {
    let state = SidebarState::new(false).toggled();

    assert!(state.collapsed);
    assert!(!sidebar_toggle(state.collapsed));
  }
}
