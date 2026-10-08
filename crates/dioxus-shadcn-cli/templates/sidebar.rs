//! Sidebar: app shell and navigation parts, with an opt-in off-canvas panel
//! for phones and an opt-in keyboard shortcut. Routing and persistence stay
//! with the app.
use std::cell::Cell;
use std::rc::Rc;

use super::default_attribute::default_attribute;
use super::element_id::next_element_id;
use super::media_query::use_media_query;
use super::modal_focus::use_modal_focus_scope;
use super::root_state::{Controllable, use_controllable, use_root_context};
use super::utils::{classes, merge_classes};
use super::density::{density_control_class, use_density, with_density};
use super::script::{Script, component_script};
use super::safe_url::safe_href;
use dioxus::prelude::*;

/// Below this width the sidebar of an `off_canvas` provider is off-canvas.
pub const SIDEBAR_MOBILE_QUERY: &str = "(max-width: 767px)";

// Sends a message for Ctrl or Command and the shortcut key, until the
// sidebar is removed.
// Keep in sync with `sidebar_shortcut_script` in the crate `sidebar.rs`.
component_script!(sidebar_shortcut_script = r#"
export async function run(dioxus) {
  const [scopeId, key] = await dioxus.recv();
  const present = () => document.querySelector(`[data-dxui-sidebar="${scopeId}"]`) !== null;
  // Typing keeps the shortcut for itself, such as Ctrl+B for bold in an
  // editor.
  const editing = (target) =>
    target instanceof Element && (target.isContentEditable || target.closest("input, textarea, select") !== null);
  const onKeyDown = (event) => {
    if (event.key.toLowerCase() !== key || event.altKey || event.shiftKey) return;
    if (!event.ctrlKey && !event.metaKey) return;
    if (event.defaultPrevented || editing(event.target)) return;
    event.preventDefault();
    dioxus.send(null);
  };
  await new Promise((resolve) => requestAnimationFrame(resolve));
  if (!present()) return;
  window.addEventListener("keydown", onKeyDown);
  await new Promise((resolve) => {
    const observer = new MutationObserver(() => {
      if (!present()) {
        observer.disconnect();
        resolve();
      }
    });
    observer.observe(document.documentElement, { subtree: true, childList: true });
  });
  window.removeEventListener("keydown", onKeyDown);
}
"#);

/// Which edge of the screen the sidebar sits on.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SidebarSide {
  /// The left edge.
  #[default]
  Left,
  /// The right edge.
  Right,
}

/// The collapsed state after a toggle: the opposite of `collapsed`.
pub const fn sidebar_toggle(collapsed: bool) -> bool {
  !collapsed
}

const SIDEBAR_BASE_CLASS: &str = "flex h-full flex-col border-sidebar-border bg-sidebar text-sidebar-foreground transition-[width] data-[side=left]:border-r data-[side=right]:border-l";
const SIDEBAR_RAIL_BASE_CLASS: &str = "absolute inset-y-0 z-10 w-3 -translate-x-1/2 transition-colors hover:bg-sidebar-accent data-[collapsed=true]:block";
const SIDEBAR_HEADER_BASE_CLASS: &str =
  "flex min-h-14 items-center gap-2 border-b border-sidebar-border px-3";
const SIDEBAR_CONTENT_BASE_CLASS: &str = "flex-1 overflow-auto p-2";
const SIDEBAR_FOOTER_BASE_CLASS: &str = "border-t border-sidebar-border p-2";
const SIDEBAR_GROUP_BASE_CLASS: &str = "grid gap-1 py-2";
const SIDEBAR_GROUP_LABEL_BASE_CLASS: &str =
  "px-2 py-1 text-xs font-medium text-muted-foreground";
const SIDEBAR_ITEM_BASE_CLASS: &str = "flex min-h-9 items-center gap-2 rounded-md px-2 text-sm transition-colors hover:bg-sidebar-accent data-[active=true]:bg-sidebar-accent data-[active=true]:text-sidebar-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
const SIDEBAR_MOBILE_PANEL_CLASS: &str =
  "fixed inset-y-0 z-50 flex w-72 max-w-[85vw] shadow-lg outline-none";
const SIDEBAR_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-40 bg-black/50";
const SIDEBAR_TRIGGER_BASE_CLASS: &str = "inline-flex h-9 w-9 items-center justify-center rounded-md text-sm transition-colors hover:bg-sidebar-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sidebar-ring disabled:pointer-events-none disabled:opacity-50";

fn sidebar_side_attribute(side: SidebarSide) -> &'static str {
  match side {
    SidebarSide::Left => "left",
    SidebarSide::Right => "right",
  }
}

fn sidebar_class(collapsed: bool, side: SidebarSide, class: &str) -> String {
  let side_class = match side {
    SidebarSide::Left => "border-r",
    SidebarSide::Right => "border-l",
  };

  merge_classes(classes([Some(SIDEBAR_BASE_CLASS), Some(side_class), Some(if collapsed { "w-14" } else { "w-64" })]), class)
}

/// The off-canvas panel shown below `SIDEBAR_MOBILE_QUERY`, holding the
/// sidebar.
pub fn sidebar_mobile_panel_class(side: SidebarSide) -> String {
  let side_class = match side {
    SidebarSide::Left => "left-0",
    SidebarSide::Right => "right-0",
  };

  classes([Some(SIDEBAR_MOBILE_PANEL_CLASS), Some(side_class)])
}

/// The sidebar inside the off-canvas panel, at the panel's full width.
pub fn sidebar_mobile_class(side: SidebarSide, class: &str) -> String {
  let side_class = match side {
    SidebarSide::Left => "border-r",
    SidebarSide::Right => "border-l",
  };

  merge_classes(classes([Some(SIDEBAR_BASE_CLASS), Some(side_class), Some("w-full")]), class)
}

/// Classes for the dimmed backdrop behind the off-canvas panel, with `class`
/// merged over them.
pub fn sidebar_overlay_class(class: &str) -> String {
  merge_classes(classes([Some(SIDEBAR_OVERLAY_BASE_CLASS)]), class)
}

fn sidebar_rail_class(collapsed: bool, class: &str) -> String {
  merge_classes(classes([Some(SIDEBAR_RAIL_BASE_CLASS), Some(if collapsed { "block" } else { "hidden" })]), class)
}

fn sidebar_header_class(class: &str) -> String {
  merge_classes(classes([Some(SIDEBAR_HEADER_BASE_CLASS)]), class)
}

fn sidebar_content_class(class: &str) -> String {
  merge_classes(classes([Some(SIDEBAR_CONTENT_BASE_CLASS)]), class)
}

fn sidebar_footer_class(class: &str) -> String {
  merge_classes(classes([Some(SIDEBAR_FOOTER_BASE_CLASS)]), class)
}

fn sidebar_group_class(class: &str) -> String {
  merge_classes(classes([Some(SIDEBAR_GROUP_BASE_CLASS)]), class)
}

fn sidebar_group_label_class(class: &str) -> String {
  merge_classes(classes([Some(SIDEBAR_GROUP_LABEL_BASE_CLASS)]), class)
}

fn sidebar_item_class(active: bool, disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(SIDEBAR_ITEM_BASE_CLASS), active.then_some("bg-sidebar-accent text-sidebar-accent-foreground"), disabled.then_some("pointer-events-none opacity-50")]), class)
}

fn sidebar_trigger_class(class: &str) -> String {
  merge_classes(classes([Some(SIDEBAR_TRIGGER_BASE_CLASS)]), class)
}

#[derive(Clone, Copy)]
struct SidebarContext {
  collapsed: Controllable<bool>,
  mobile_open: Controllable<bool>,
  off_canvas: bool,
  id: usize,
}

impl SidebarContext {
  fn panel_id(&self) -> String {
    format!("dxui-sidebar-{}-panel", self.id)
  }

  /// Toggles what the sidebar shows: the off-canvas panel while it is
  /// modal, else its width.
  fn toggle(&self, modal: bool) {
    let state = if modal { self.mobile_open } else { self.collapsed };
    state.set(!state.get());
  }
}

fn use_sidebar(part: &str) -> SidebarContext {
  use_root_context::<Signal<SidebarContext>>(part, "SidebarProvider").cloned()
}

/// The root of a sidebar layout: it owns whether the sidebar is collapsed and,
/// with `off_canvas`, whether its off-canvas panel is open (RFC 0077), and
/// shares that with `Sidebar`, `SidebarRail`, and `SidebarTrigger`, which may
/// sit anywhere inside it. Pass `collapsed` or `mobile_open` to control
/// either, or `default_collapsed` to start collapsed; the change callbacks
/// hear every change the user makes. It renders no element of its own.
#[component]
pub fn SidebarProvider(
  #[props(default)] collapsed: ReadSignal<Option<bool>>,
  #[props(default)] default_collapsed: bool,
  #[props(default)] on_collapsed_change: Option<EventHandler<bool>>,
  #[props(default)] off_canvas: bool,
  #[props(default)] mobile_open: ReadSignal<Option<bool>>,
  #[props(default)] on_mobile_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let collapsed =
    use_controllable(move || collapsed.cloned(), move || default_collapsed, on_collapsed_change);
  let mobile_open = use_controllable(move || mobile_open.cloned(), || false, on_mobile_open_change);
  let id = use_hook(next_element_id);
  let context = SidebarContext { collapsed, mobile_open, off_canvas, id };
  // `off_canvas` may change, so the context is refreshed on every render.
  let mut shared = use_context_provider(|| Signal::new(context));
  if shared.peek().off_canvas != off_canvas {
    shared.set(context);
  }

  rsx! { {children} }
}

/// The sidebar is an `aside` that the provider's collapsed state narrows.
/// With an `off_canvas` provider it is off-canvas below
/// `SIDEBAR_MOBILE_QUERY`: a modal panel shown while the provider's
/// `mobile_open`, over an overlay, that traps focus, locks page scroll, and
/// closes on Escape or an overlay press. With `shortcut`, Ctrl or Command and
/// that key toggle it: the panel when off-canvas, the width otherwise.
#[component]
pub fn Sidebar(
  #[props(default)] side: SidebarSide,
  #[props(default)] shortcut: Option<char>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = aside)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let sidebar = use_sidebar("Sidebar");
  let collapsed = sidebar.collapsed.get();
  let mobile_open = sidebar.mobile_open.get();
  let (mobile, media_scope) = use_media_query(SIDEBAR_MOBILE_QUERY, sidebar.off_canvas);
  let modal = mobile && sidebar.off_canvas;
  let focus_scope = use_modal_focus_scope(modal && mobile_open, true);
  let scope_id = use_hook(|| format!("dxui-sidebar-scope-{}", next_element_id()));
  // The shortcut handler outlives this render, so it reads the latest mode.
  let latest_modal = use_hook(|| Rc::new(Cell::new(false)));
  latest_modal.set(modal);
  let effect_scope_id = scope_id.clone();
  use_effect(use_reactive((&shortcut,), move |(shortcut,)| {
    let Some(key) = shortcut else {
      return;
    };
    let latest_modal = latest_modal.clone();
    let script = sidebar_shortcut_script::start();
    // A send error means the page already finished the script; nothing to track.
    let _ = script.send((effect_scope_id.as_str(), key.to_ascii_lowercase().to_string()));
    spawn(async move {
      while script.recv::<()>().await.is_ok() {
        sidebar.toggle(latest_modal.get());
      }
    });
  }));
  let close = move |_| sidebar.mobile_open.set(false);
  let generated_id = sidebar.panel_id();

  if !sidebar.off_canvas {
    let class = sidebar_class(collapsed, side, &class);
    let id = default_attribute(&attributes, "id", generated_id);
    return rsx! {
      aside {
        id,
        class,
        "data-collapsed": collapsed.to_string(),
        "data-side": sidebar_side_attribute(side),
        "data-dxui-sidebar": scope_id,
        ..attributes,
        {children}
      }
    };
  }

  // Off-canvas: a wrapper is the modal panel below the breakpoint, named by
  // the sidebar. Above it the wrapper is a flex item that stretches to the
  // layout's height, so the sidebar's `h-full` resolves against it; a
  // `display: contents` wrapper leaves that percentage unresolved in Chrome.
  let passed_id = attributes.iter().find(|attribute| attribute.name == "id").and_then(
    |attribute| match &attribute.value {
      dioxus::dioxus_core::AttributeValue::Text(id) => Some(id.clone()),
      _ => None,
    },
  );
  let label_id = passed_id.unwrap_or_else(|| generated_id.clone());
  let id = default_attribute(&attributes, "id", generated_id);
  let panel_class =
    if modal { sidebar_mobile_panel_class(side) } else { "flex shrink-0".to_string() };
  let class =
    if modal { sidebar_mobile_class(side, &class) } else { sidebar_class(collapsed, side, &class) };

  rsx! {
    div {
      class: panel_class,
      role: modal.then_some("dialog"),
      "aria-modal": modal.then_some("true"),
      "aria-labelledby": modal.then_some(label_id),
      tabindex: modal.then_some("-1"),
      hidden: modal && !mobile_open,
      "data-dxui-focus-scope": focus_scope,
      onkeydown: move |event: KeyboardEvent| {
        if modal && event.key() == Key::Escape {
          event.prevent_default();
          close(());
        }
      },
      aside {
        id,
        class,
        "data-collapsed": collapsed.to_string(),
        "data-side": sidebar_side_attribute(side),
        "data-mobile": modal.to_string(),
        "data-dxui-media": media_scope,
        "data-dxui-sidebar": scope_id,
        ..attributes,
        {children}
      }
    }
    if modal && mobile_open {
      div {
        class: sidebar_overlay_class(""),
        "aria-hidden": "true",
        onclick: move |_| close(()),
      }
    }
  }
}

#[component]
pub fn SidebarRail(#[props(default)] class: String) -> Element {
  let collapsed = use_sidebar("SidebarRail").collapsed.get();
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
  let class = sidebar_item_class(active, disabled, &with_density(density_control_class(use_density()), &class));
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
        href: (!disabled).then_some(href).and_then(safe_href),
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

/// Toggles the provider's collapsed state, or below `SIDEBAR_MOBILE_QUERY`
/// with an `off_canvas` provider, its off-canvas panel; `aria-expanded`
/// follows whichever it toggles, and `aria-controls` names the sidebar.
#[component]
pub fn SidebarTrigger(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let sidebar = use_sidebar("SidebarTrigger");
  let class = sidebar_trigger_class(&with_density(density_control_class(use_density()), &class));
  let (mobile, media_scope) = use_media_query(SIDEBAR_MOBILE_QUERY, sidebar.off_canvas);
  let modal = mobile && sidebar.off_canvas;
  let collapsed = sidebar.collapsed.get();
  let expanded = if modal { sidebar.mobile_open.get() } else { !collapsed };
  let controls = default_attribute(&attributes, "aria-controls", sidebar.panel_id());

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-expanded": expanded.to_string(),
      "aria-controls": controls,
      "data-collapsed": collapsed.to_string(),
      "data-dxui-media": media_scope,
      onclick: move |_| {
        if !disabled {
          sidebar.toggle(modal);
        }
      },
      ..attributes,
      {children}
    }
  }
}
