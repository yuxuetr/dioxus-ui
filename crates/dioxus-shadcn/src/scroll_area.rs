use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{ScrollAreaOrientation, scroll_area_orientation_attribute};

use crate::default_attribute::default_attribute;

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

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_viewport_is_focusable_and_takes_attributes() {
    fn app() -> Element {
      rsx! {
        ScrollAreaViewport { role: "region", "aria-label": "Tags", "a" }
        ScrollAreaViewport { tabindex: "-1", "b" }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"tabindex="0""#));
    assert!(html.contains(r#"aria-label="Tags""#));
    assert!(html.contains(r#"tabindex="-1""#));
    assert_eq!(html.matches("tabindex=").count(), 2);
  }

  #[test]
  fn scroll_area_scrollbar_class_reflects_orientation() {
    let actual = scroll_area_scrollbar_class(ScrollAreaOrientation::Horizontal, "bg-muted");

    assert!(actual.contains(SCROLL_AREA_SCROLLBAR_BASE_CLASS));
    assert!(actual.contains("h-2.5 flex-col"));
    assert!(actual.ends_with("bg-muted"));
  }

  #[test]
  fn scroll_area_orientation_helper_is_reexported() {
    assert_eq!(scroll_area_orientation_attribute(ScrollAreaOrientation::Both), "both");
  }
}
