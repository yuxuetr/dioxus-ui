use dioxus::prelude::*;
use super::utils::classes;

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
pub const SCROLL_AREA_VIEWPORT_BASE_CLASS: &str = "h-full w-full overflow-auto rounded-inherit";
pub const SCROLL_AREA_CONTENT_BASE_CLASS: &str = "min-w-full";
pub const SCROLL_AREA_SCROLLBAR_BASE_CLASS: &str = "flex touch-none select-none transition-colors data-[orientation=horizontal]:h-2.5 data-[orientation=horizontal]:flex-col data-[orientation=vertical]:h-full data-[orientation=vertical]:w-2.5";
pub const SCROLL_AREA_THUMB_BASE_CLASS: &str = "relative flex-1 rounded-full bg-zinc-300";
pub const SCROLL_AREA_CORNER_BASE_CLASS: &str = "bg-zinc-100";

pub fn scroll_area_class(class: &str) -> String {
  classes([Some(SCROLL_AREA_BASE_CLASS), Some(class)])
}

pub fn scroll_area_viewport_class(class: &str) -> String {
  classes([Some(SCROLL_AREA_VIEWPORT_BASE_CLASS), Some(class)])
}

pub fn scroll_area_content_class(class: &str) -> String {
  classes([Some(SCROLL_AREA_CONTENT_BASE_CLASS), Some(class)])
}

pub fn scroll_area_scrollbar_class(orientation: ScrollAreaOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    ScrollAreaOrientation::Vertical => "h-full w-2.5 border-l border-l-transparent p-px",
    ScrollAreaOrientation::Horizontal => "h-2.5 flex-col border-t border-t-transparent p-px",
    ScrollAreaOrientation::Both => "h-full w-2.5 border-l border-l-transparent p-px",
  };

  classes([
    Some(SCROLL_AREA_SCROLLBAR_BASE_CLASS),
    Some(orientation_class),
    Some(class),
  ])
}

pub fn scroll_area_thumb_class(class: &str) -> String {
  classes([Some(SCROLL_AREA_THUMB_BASE_CLASS), Some(class)])
}

pub fn scroll_area_corner_class(class: &str) -> String {
  classes([Some(SCROLL_AREA_CORNER_BASE_CLASS), Some(class)])
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
pub fn ScrollAreaViewport(#[props(default)] class: String, children: Element) -> Element {
  let class = scroll_area_viewport_class(&class);

  rsx! {
    div {
      class,
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
