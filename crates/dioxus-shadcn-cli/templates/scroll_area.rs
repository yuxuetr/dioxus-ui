use super::default_attribute::default_attribute;
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScrollAreaOrientation {
  Vertical,
  Horizontal,
  #[default]
  Both,
}

pub fn scroll_area_orientation_attribute(orientation: ScrollAreaOrientation) -> &'static str {
  match orientation {
    ScrollAreaOrientation::Vertical => "vertical",
    ScrollAreaOrientation::Horizontal => "horizontal",
    ScrollAreaOrientation::Both => "both",
  }
}

pub const SCROLL_AREA_BASE_CLASS: &str = "relative overflow-hidden";
pub const SCROLL_AREA_VIEWPORT_BASE_CLASS: &str = "h-full w-full overflow-auto rounded-inherit focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring";
pub const SCROLL_AREA_CONTENT_BASE_CLASS: &str = "min-w-full";
pub const SCROLL_AREA_SCROLLBAR_BASE_CLASS: &str = "flex touch-none select-none transition-colors data-[orientation=horizontal]:h-2.5 data-[orientation=horizontal]:flex-col data-[orientation=vertical]:h-full data-[orientation=vertical]:w-2.5";
pub const SCROLL_AREA_THUMB_BASE_CLASS: &str = "relative flex-1 rounded-full bg-border";
pub const SCROLL_AREA_CORNER_BASE_CLASS: &str = "bg-accent";

pub fn scroll_area_class(class: &str) -> String {
  merge_classes(classes([Some(SCROLL_AREA_BASE_CLASS)]), class)
}

pub fn scroll_area_viewport_class(class: &str) -> String {
  merge_classes(classes([Some(SCROLL_AREA_VIEWPORT_BASE_CLASS)]), class)
}

pub fn scroll_area_content_class(class: &str) -> String {
  merge_classes(classes([Some(SCROLL_AREA_CONTENT_BASE_CLASS)]), class)
}

pub fn scroll_area_scrollbar_class(orientation: ScrollAreaOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    ScrollAreaOrientation::Vertical => "h-full w-2.5 border-l border-l-transparent p-px",
    ScrollAreaOrientation::Horizontal => "h-2.5 flex-col border-t border-t-transparent p-px",
    ScrollAreaOrientation::Both => "h-full w-2.5 border-l border-l-transparent p-px",
  };

  merge_classes(classes([Some(SCROLL_AREA_SCROLLBAR_BASE_CLASS), Some(orientation_class)]), class)
}

pub fn scroll_area_thumb_class(class: &str) -> String {
  merge_classes(classes([Some(SCROLL_AREA_THUMB_BASE_CLASS)]), class)
}

pub fn scroll_area_corner_class(class: &str) -> String {
  merge_classes(classes([Some(SCROLL_AREA_CORNER_BASE_CLASS)]), class)
}

#[component]
pub fn ScrollArea(
  #[props(default)] orientation: ScrollAreaOrientation,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = scroll_area_class(&class);

  rsx! {
    div {
      class,
      "data-orientation": scroll_area_orientation_attribute(orientation),
      {children}
    }
  }
}

#[component]
pub fn ScrollAreaViewport(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = scroll_area_viewport_class(&class);
  let tabindex = default_attribute(&attributes, "tabindex", "0");

  rsx! {
    div {
      class,
      tabindex,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn ScrollAreaContent(#[props(default)] class: String, children: Element) -> Element {
  let class = scroll_area_content_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn ScrollAreaScrollbar(
  #[props(default = ScrollAreaOrientation::Vertical)] orientation: ScrollAreaOrientation,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = scroll_area_scrollbar_class(orientation, &class);

  rsx! {
    div {
      class,
      "aria-hidden": "true",
      "data-orientation": scroll_area_orientation_attribute(orientation),
      {children}
    }
  }
}

#[component]
pub fn ScrollAreaThumb(#[props(default)] class: String) -> Element {
  let class = scroll_area_thumb_class(&class);

  rsx! {
    div {
      class,
    }
  }
}

#[component]
pub fn ScrollAreaCorner(#[props(default)] class: String) -> Element {
  let class = scroll_area_corner_class(&class);

  rsx! {
    div {
      class,
    }
  }
}
