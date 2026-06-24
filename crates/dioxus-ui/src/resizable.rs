use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  layout_orientation_attribute, resizable_clamp, resizable_resize_pair, LayoutOrientation,
  ResizablePanelState,
};

pub const RESIZABLE_PANEL_GROUP_BASE_CLASS: &str = "flex h-full w-full data-orientation-vertical:flex-col";
pub const RESIZABLE_PANEL_BASE_CLASS: &str = "min-w-0 overflow-hidden";
pub const RESIZABLE_HANDLE_BASE_CLASS: &str = "relative flex w-px items-center justify-center bg-zinc-200 after:absolute after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 data-orientation-vertical:h-px data-orientation-vertical:w-full data-orientation-vertical:after:inset-x-0 data-orientation-vertical:after:top-1/2 data-orientation-vertical:after:h-1 data-orientation-vertical:after:w-full data-orientation-vertical:after:-translate-y-1/2 data-disabled:opacity-50";

pub fn resizable_panel_group_class(orientation: LayoutOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    LayoutOrientation::Horizontal => "flex-row",
    LayoutOrientation::Vertical => "flex-col",
  };

  classes([
    Some(RESIZABLE_PANEL_GROUP_BASE_CLASS),
    Some(orientation_class),
    Some(class),
  ])
}

pub fn resizable_panel_class(collapsed: bool, class: &str) -> String {
  classes([
    Some(RESIZABLE_PANEL_BASE_CLASS),
    collapsed.then_some("hidden"),
    Some(class),
  ])
}

pub fn resizable_handle_class(disabled: bool, class: &str) -> String {
  classes([
    Some(RESIZABLE_HANDLE_BASE_CLASS),
    disabled.then_some("pointer-events-none opacity-50"),
    Some(class),
  ])
}

pub fn resizable_panel_style(size: f64, min_size: f64, max_size: f64) -> String {
  let size = resizable_clamp(size, min_size, max_size);

  format!("flex-basis: {size}%;")
}

#[component]
pub fn ResizablePanelGroup(
  #[props(default)] orientation: LayoutOrientation,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = resizable_panel_group_class(orientation, &class);

  rsx! {
    div {
      class,
      "data-orientation": layout_orientation_attribute(orientation),
      {children}
    }
  }
}

#[component]
pub fn ResizablePanel(
  #[props(default = 50.0)] size: f64,
  #[props(default = 0.0)] min_size: f64,
  #[props(default = 100.0)] max_size: f64,
  #[props(default)] collapsed: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = resizable_panel_class(collapsed, &class);
  let style = resizable_panel_style(size, min_size, max_size);

  rsx! {
    div {
      class,
      style,
      hidden: collapsed,
      "data-collapsed": collapsed.to_string(),
      {children}
    }
  }
}

#[component]
pub fn ResizableHandle(
  #[props(default)] orientation: LayoutOrientation,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
) -> Element {
  let class = resizable_handle_class(disabled, &class);

  rsx! {
    div {
      role: "separator",
      class,
      "aria-disabled": disabled.to_string(),
      "aria-orientation": layout_orientation_attribute(orientation),
      "data-disabled": disabled.to_string(),
      "data-orientation": layout_orientation_attribute(orientation),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn resizable_group_class_reflects_orientation() {
    let actual = resizable_panel_group_class(LayoutOrientation::Vertical, "h-80");

    assert!(actual.contains(RESIZABLE_PANEL_GROUP_BASE_CLASS));
    assert!(actual.contains("flex-col"));
    assert!(actual.ends_with("h-80"));
  }

  #[test]
  fn resizable_panel_style_clamps_size() {
    assert_eq!(resizable_panel_style(120.0, 20.0, 80.0), "flex-basis: 80%;");
  }

  #[test]
  fn resizable_primitives_are_reexported() {
    let first = ResizablePanelState::new(50.0, 20.0, 80.0);
    let second = ResizablePanelState::new(50.0, 20.0, 80.0);
    let (first, second) = resizable_resize_pair(first, second, 10.0);

    assert_eq!(first.size, 60.0);
    assert_eq!(second.size, 40.0);
  }
}
