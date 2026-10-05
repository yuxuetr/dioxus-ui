use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use super::utils::{classes, group_part_id, use_roving_group};

static NEXT_TABS_ID: AtomicUsize = AtomicUsize::new(0);

pub const TABS_BASE_CLASS: &str = "data-[orientation=vertical]:flex data-[orientation=vertical]:gap-4";
pub const TABS_LIST_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground data-[orientation=vertical]:h-auto data-[orientation=vertical]:flex-col data-[orientation=vertical]:items-stretch data-[orientation=vertical]:justify-start";
pub const TABS_TRIGGER_BASE_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
pub const TABS_CONTENT_BASE_CLASS: &str = "mt-2 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring data-[orientation=vertical]:mt-0";

/// Whether moving focus to a trigger also selects it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsActivation {
  /// Arrow, Home, and End keys select the trigger they move to.
  #[default]
  Automatic,
  /// Keys only move focus; Enter, Space, or a click selects.
  Manual,
}

impl TabsActivation {
  /// The roving group script's `data-dxui-roving-activation` value.
  pub fn roving_attribute(self) -> &'static str {
    match self {
      Self::Automatic => "focus",
      Self::Manual => "manual",
    }
  }
}

/// The direction the triggers are laid out and moved through.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsOrientation {
  /// Left and Right move between triggers.
  #[default]
  Horizontal,
  /// Up and Down move between triggers, and the list sits beside the panels.
  Vertical,
}

impl TabsOrientation {
  /// The value for `aria-orientation` and `data-orientation`.
  pub fn attribute(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

pub fn tabs_class(class: &str) -> String {
  classes([Some(TABS_BASE_CLASS), Some(class)])
}

pub fn tabs_list_class(class: &str) -> String {
  classes([Some(TABS_LIST_BASE_CLASS), Some(class)])
}

pub fn tabs_trigger_class(active: bool, class: &str) -> String {
  let active_class =
    if active { "bg-background text-foreground shadow-sm" } else { "text-muted-foreground hover:text-foreground" };

  classes([Some(TABS_TRIGGER_BASE_CLASS), Some(active_class), Some(class)])
}

pub fn tabs_content_class(class: &str) -> String {
  classes([Some(TABS_CONTENT_BASE_CLASS), Some(class)])
}

#[derive(Clone, PartialEq)]
struct TabsContext {
  base_id: String,
  on_value_change: Option<EventHandler<String>>,
  activation: TabsActivation,
  orientation: TabsOrientation,
}

/// Links each trigger to its panel by id and reports tab requests: a click,
/// Enter, or Space on a trigger calls `on_value_change` with its `value`, and
/// with `TabsActivation::Automatic` so does a key that moves focus to another
/// trigger. `activation` and `orientation` are read when the root mounts.
#[component]
pub fn Tabs(
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] activation: TabsActivation,
  #[props(default)] orientation: TabsOrientation,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = tabs_class(&class);
  let base_id = use_hook(|| format!("dxui-tabs-{}", NEXT_TABS_ID.fetch_add(1, Ordering::Relaxed)));
  use_context_provider(|| TabsContext { base_id, on_value_change, activation, orientation });

  rsx! {
    div {
      class,
      "data-orientation": orientation.attribute(),
      {children}
    }
  }
}

/// Keeps one Tab stop on the selected trigger, which focus leaving the list
/// restores. Left and Right, or Up and Down when vertical, move between
/// enabled triggers and wrap; Home and End jump to the first and last.
#[component]
pub fn TabsList(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = tabs_list_class(&class);
  let context = try_use_context::<TabsContext>();
  let on_value_change = context.as_ref().and_then(|context| context.on_value_change);
  let activation = context.as_ref().map(|context| context.activation).unwrap_or_default();
  let orientation = context.as_ref().map(|context| context.orientation).unwrap_or_default();
  let scope_id = use_roving_group(on_value_change);

  rsx! {
    div {
      role: "tablist",
      class,
      "aria-orientation": orientation.attribute(),
      "data-orientation": orientation.attribute(),
      "data-dxui-roving-group": scope_id,
      "data-dxui-roving-orientation": orientation.attribute(),
      "data-dxui-roving-loop": "true",
      "data-dxui-roving-activation": activation.roving_attribute(),
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn TabsTrigger(
  value: String,
  #[props(default)] active: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = tabs_trigger_class(active, &class);
  let base_id = try_use_context::<TabsContext>().map(|context| context.base_id);
  let id = base_id.as_deref().map(|base_id| group_part_id(base_id, "trigger", &value));
  let controls = base_id.as_deref().map(|base_id| group_part_id(base_id, "content", &value));

  rsx! {
    button {
      r#type: "button",
      role: "tab",
      id,
      class,
      disabled,
      "aria-selected": active.to_string(),
      "aria-controls": controls,
      "data-value": value,
      "data-dxui-roving-item": "",
      {children}
    }
  }
}

#[component]
pub fn TabsContent(
  value: String,
  #[props(default)] active: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = tabs_content_class(&class);
  let context = try_use_context::<TabsContext>();
  let orientation = context.as_ref().map(|context| context.orientation.attribute());
  let base_id = context.map(|context| context.base_id);
  let id = base_id.as_deref().map(|base_id| group_part_id(base_id, "content", &value));
  let labelledby = base_id.as_deref().map(|base_id| group_part_id(base_id, "trigger", &value));

  rsx! {
    div {
      role: "tabpanel",
      id,
      class,
      tabindex: "0",
      hidden: !active,
      "aria-labelledby": labelledby,
      "data-orientation": orientation,
      "data-value": value,
      {children}
    }
  }
}
