use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use dioxus_ui_core::classes;

use crate::roving_group::{group_part_id, use_roving_group};

static NEXT_TABS_ID: AtomicUsize = AtomicUsize::new(0);

pub const TABS_LIST_BASE_CLASS: &str =
  "inline-flex h-10 items-center justify-center rounded-md bg-zinc-100 p-1 text-zinc-600";
pub const TABS_TRIGGER_BASE_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const TABS_CONTENT_BASE_CLASS: &str =
  "mt-2 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";

pub fn tabs_list_class(class: &str) -> String {
  classes([Some(TABS_LIST_BASE_CLASS), Some(class)])
}

pub fn tabs_trigger_class(active: bool, class: &str) -> String {
  let active_class =
    if active { "bg-white text-zinc-950 shadow-sm" } else { "text-zinc-600 hover:text-zinc-950" };

  classes([Some(TABS_TRIGGER_BASE_CLASS), Some(active_class), Some(class)])
}

pub fn tabs_content_class(class: &str) -> String {
  classes([Some(TABS_CONTENT_BASE_CLASS), Some(class)])
}

#[derive(Clone, PartialEq)]
struct TabsContext {
  base_id: String,
  on_value_change: Option<EventHandler<String>>,
}

/// Links each trigger to its panel by id and reports tab requests: a click on
/// a trigger, or an arrow, Home, or End key that moves focus to another
/// trigger, calls `on_value_change` with that trigger's `value`.
#[component]
pub fn Tabs(
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let base_id = use_hook(|| format!("dxui-tabs-{}", NEXT_TABS_ID.fetch_add(1, Ordering::Relaxed)));
  use_context_provider(|| TabsContext { base_id, on_value_change });

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// Keeps one Tab stop on the selected trigger. Left and Right move between
/// enabled triggers and wrap; Home and End jump to the first and last.
#[component]
pub fn TabsList(#[props(default)] class: String, children: Element) -> Element {
  let class = tabs_list_class(&class);
  let on_value_change =
    try_use_context::<TabsContext>().and_then(|context| context.on_value_change);
  let scope_id = use_roving_group(on_value_change);

  rsx! {
    div {
      role: "tablist",
      class,
      "aria-orientation": "horizontal",
      "data-dxui-roving-group": scope_id,
      "data-dxui-roving-orientation": "horizontal",
      "data-dxui-roving-loop": "true",
      "data-dxui-roving-activation": "focus",
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
  let base_id = try_use_context::<TabsContext>().map(|context| context.base_id);
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
      "data-value": value,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tabs_trigger_class_reflects_active_state() {
    let actual = tabs_trigger_class(true, "min-w-24");

    assert!(actual.contains(TABS_TRIGGER_BASE_CLASS));
    assert!(actual.contains("bg-white text-zinc-950 shadow-sm"));
    assert!(actual.ends_with("min-w-24"));
  }

  #[test]
  fn tabs_content_class_appends_user_class() {
    let actual = tabs_content_class("p-4");

    assert!(actual.contains(TABS_CONTENT_BASE_CLASS));
    assert!(actual.ends_with("p-4"));
  }
}
