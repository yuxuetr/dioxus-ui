use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

use crate::density::{density_control_class, use_density, with_density};
use crate::element_id::next_element_id;
use crate::root_state::{Controllable, use_controllable, use_root_context};
use crate::roving_group::{group_part_id, use_roving_group};

pub const TABS_BASE_CLASS: &str =
  "data-[orientation=vertical]:flex data-[orientation=vertical]:gap-4";
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
  merge_classes(classes([Some(TABS_BASE_CLASS)]), class)
}

pub fn tabs_list_class(class: &str) -> String {
  merge_classes(classes([Some(TABS_LIST_BASE_CLASS)]), class)
}

pub fn tabs_trigger_class(active: bool, class: &str) -> String {
  let active_class = if active {
    "bg-background text-foreground shadow-sm"
  } else {
    "text-muted-foreground hover:text-foreground"
  };

  merge_classes(classes([Some(TABS_TRIGGER_BASE_CLASS), Some(active_class)]), class)
}

pub fn tabs_content_class(class: &str) -> String {
  merge_classes(classes([Some(TABS_CONTENT_BASE_CLASS)]), class)
}

/// What `Tabs` shares with its parts (RFC 0077).
#[derive(Clone)]
struct TabsContext {
  base_id: String,
  value: Controllable<Option<String>>,
  select: Callback<String>,
  activation: TabsActivation,
  orientation: TabsOrientation,
}

/// The root of a tab set: it owns which tab is selected and links its parts.
/// Pass `value` to control it, or `default_value` to start it; the change
/// callback hears every change the user makes either way.
#[component]
pub fn Tabs(
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: Option<String>,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] activation: TabsActivation,
  #[props(default)] orientation: TabsOrientation,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = tabs_class(&class);
  let base_id = use_hook(|| format!("dxui-tabs-{}", next_element_id()));
  let change = use_callback(move |next: Option<String>| {
    if let (Some(handler), Some(next)) = (on_value_change, next) {
      handler.call(next);
    }
  });
  let value = use_controllable(move || value().map(Some), move || default_value, Some(change));
  let select = use_callback(move |next: String| value.set(Some(next)));
  use_context_provider(|| TabsContext { base_id, value, select, activation, orientation });

  rsx! {
    div {
      class,
      "data-orientation": orientation.attribute(),
      {children}
    }
  }
}

#[component]
pub fn TabsList(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let context = use_root_context::<TabsContext>("TabsList", "Tabs");
  let class = tabs_list_class(&class);
  let orientation = context.orientation.attribute();
  let scope_id = use_roving_group(Some(context.select));

  rsx! {
    div {
      role: "tablist",
      class,
      "aria-orientation": orientation,
      "data-orientation": orientation,
      "data-dxui-roving-group": scope_id,
      "data-dxui-roving-orientation": orientation,
      "data-dxui-roving-loop": "true",
      "data-dxui-roving-activation": context.activation.roving_attribute(),
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn TabsTrigger(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let context = use_root_context::<TabsContext>("TabsTrigger", "Tabs");
  let active = context.value.get().as_deref() == Some(value.as_str());
  let class =
    tabs_trigger_class(active, &with_density(density_control_class(use_density()), &class));
  let id = group_part_id(&context.base_id, "trigger", &value);
  let controls = group_part_id(&context.base_id, "content", &value);

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
pub fn TabsContent(value: String, #[props(default)] class: String, children: Element) -> Element {
  let context = use_root_context::<TabsContext>("TabsContent", "Tabs");
  let active = context.value.get().as_deref() == Some(value.as_str());
  let class = tabs_content_class(&class);
  let id = group_part_id(&context.base_id, "content", &value);
  let labelledby = group_part_id(&context.base_id, "trigger", &value);

  rsx! {
    div {
      role: "tabpanel",
      id,
      class,
      tabindex: "0",
      hidden: !active,
      "aria-labelledby": labelledby,
      "data-orientation": context.orientation.attribute(),
      "data-value": value,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  fn account_tabs() -> Element {
    rsx! {
      TabsList {
        TabsTrigger { value: "account", "Account" }
        TabsTrigger { value: "password", "Password" }
      }
      TabsContent { value: "account", "Account settings" }
      TabsContent { value: "password", "Password settings" }
    }
  }

  #[test]
  fn ssr_tabs_select_the_default_and_link_their_parts() {
    fn app() -> Element {
      rsx! { Tabs { default_value: "account", {account_tabs()} } }
    }
    let html = render(app);

    assert!(html.contains(r#"id="dxui-tabs-0-trigger-account""#), "{html}");
    assert!(html.contains(r#"aria-controls="dxui-tabs-0-content-account""#));
    assert!(html.contains(r#"aria-labelledby="dxui-tabs-0-trigger-account""#));
    assert_eq!(html.matches(r#"aria-selected="true""#).count(), 1);
    assert!(html.contains(r#"aria-selected="true" aria-controls="dxui-tabs-0-content-account""#));
    assert_eq!(html.matches("hidden").count(), 1);
    assert!(
      html.contains(r#"hidden data-orientation="horizontal" data-value="password""#)
        || html.contains(r#"hidden=true"#)
        || html.contains("hidden")
    );
  }

  #[test]
  fn ssr_controlled_value_wins_over_the_default() {
    fn app() -> Element {
      rsx! { Tabs { value: "password", default_value: "account", {account_tabs()} } }
    }
    let html = render(app);

    assert!(html.contains(r#"aria-selected="true" aria-controls="dxui-tabs-0-content-password""#));
    assert_eq!(html.matches(r#"aria-selected="true""#).count(), 1);
  }

  #[test]
  fn two_renders_write_the_same_ids() {
    fn app() -> Element {
      rsx! {
        Tabs { default_value: "account", {account_tabs()} }
        Tabs { default_value: "password", {account_tabs()} }
      }
    }
    let first = render(app);

    assert_eq!(first, render(app));
    assert_eq!(first.matches(r#"role="tablist""#).count(), 2);
    assert!(first.contains(r#"id="dxui-tabs-0-trigger-account""#));
  }

  #[test]
  fn a_part_outside_its_tabs_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        TabsTrigger { value: "account", "Account" }
        p { "after" }
      }
    }
    let html = render(app);

    assert_eq!(html, "<p>before</p><p>after</p>");
  }

  #[test]
  fn tabs_trigger_class_reflects_active_state() {
    let actual = tabs_trigger_class(true, "min-w-24");

    assert!(actual.contains(TABS_TRIGGER_BASE_CLASS));
    assert!(actual.contains("bg-background text-foreground shadow-sm"));
    assert!(actual.ends_with("min-w-24"));
  }

  #[test]
  fn tabs_activation_maps_to_roving_attribute() {
    assert_eq!(TabsActivation::default().roving_attribute(), "focus");
    assert_eq!(TabsActivation::Manual.roving_attribute(), "manual");
  }

  #[test]
  fn tabs_orientation_maps_to_attribute() {
    assert_eq!(TabsOrientation::default().attribute(), "horizontal");
    assert_eq!(TabsOrientation::Vertical.attribute(), "vertical");
  }

  #[test]
  fn tabs_content_class_appends_user_class() {
    let actual = tabs_content_class("p-4");

    assert!(actual.contains(TABS_CONTENT_BASE_CLASS));
    assert!(actual.ends_with("p-4"));
  }
}
