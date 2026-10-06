use super::utils::classes;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SidebarSide {
  #[default]
  Left,
  Right,
}

pub const fn sidebar_toggle(collapsed: bool) -> bool {
  !collapsed
}

pub const SIDEBAR_BASE_CLASS: &str = "flex h-full flex-col border-sidebar-border bg-sidebar text-sidebar-foreground transition-[width] data-[side=left]:border-r data-[side=right]:border-l";
pub const SIDEBAR_RAIL_BASE_CLASS: &str = "absolute inset-y-0 z-10 hidden w-3 -translate-x-1/2 transition-colors hover:bg-sidebar-accent data-[collapsed=true]:block";
pub const SIDEBAR_HEADER_BASE_CLASS: &str =
  "flex min-h-14 items-center gap-2 border-b border-sidebar-border px-3";
pub const SIDEBAR_CONTENT_BASE_CLASS: &str = "flex-1 overflow-auto p-2";
pub const SIDEBAR_FOOTER_BASE_CLASS: &str = "border-t border-sidebar-border p-2";
pub const SIDEBAR_GROUP_BASE_CLASS: &str = "grid gap-1 py-2";
pub const SIDEBAR_GROUP_LABEL_BASE_CLASS: &str =
  "px-2 py-1 text-xs font-medium text-muted-foreground";
pub const SIDEBAR_ITEM_BASE_CLASS: &str = "flex min-h-9 items-center gap-2 rounded-md px-2 text-sm transition-colors hover:bg-sidebar-accent data-[active=true]:bg-sidebar-accent data-[active=true]:text-sidebar-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
pub const SIDEBAR_TRIGGER_BASE_CLASS: &str = "inline-flex h-9 w-9 items-center justify-center rounded-md text-sm transition-colors hover:bg-sidebar-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sidebar-ring disabled:pointer-events-none disabled:opacity-50";

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

  classes([
    Some(SIDEBAR_BASE_CLASS),
    Some(side_class),
    Some(if collapsed { "w-14" } else { "w-64" }),
    Some(class),
  ])
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
    active.then_some("bg-sidebar-accent text-sidebar-accent-foreground"),
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
    if !disabled {
      if let Some(handler) = onclick {
        handler.call(event);
      }
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
        if !disabled {
          if let Some(handler) = on_collapsed_change {
            handler.call(!collapsed);
          }
        }
      },
      ..attributes,
      {children}
    }
  }
}
