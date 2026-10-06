use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

use crate::default_attribute::default_attribute;
pub use dioxus_shadcn_primitives::{
  CarouselState, LayoutOrientation as CarouselOrientation, carousel_can_go_next,
  carousel_can_go_previous, carousel_clamp_index, carousel_next, carousel_previous,
};

pub const CAROUSEL_BASE_CLASS: &str = "relative";
pub const CAROUSEL_VIEWPORT_BASE_CLASS: &str = "overflow-hidden";
pub const CAROUSEL_CONTENT_BASE_CLASS: &str = "flex data-[orientation=horizontal]:-ml-4 data-[orientation=vertical]:-mt-4 data-[orientation=vertical]:flex-col";
pub const CAROUSEL_ITEM_BASE_CLASS: &str = "min-w-0 shrink-0 grow-0 basis-full transition-transform duration-300 motion-reduce:transition-none data-[orientation=horizontal]:pl-4 data-[orientation=vertical]:pt-4";
pub const CAROUSEL_CONTROL_BASE_CLASS: &str = "inline-flex h-9 w-9 items-center justify-center rounded-full border border-border bg-background text-sm shadow-sm transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
pub const CAROUSEL_INDICATOR_BASE_CLASS: &str = "h-2 w-2 rounded-full transition-colors";

pub fn carousel_orientation_attribute(orientation: CarouselOrientation) -> &'static str {
  match orientation {
    CarouselOrientation::Horizontal => "horizontal",
    CarouselOrientation::Vertical => "vertical",
  }
}

pub fn carousel_class(class: &str) -> String {
  merge_classes(classes([Some(CAROUSEL_BASE_CLASS)]), class)
}

pub fn carousel_viewport_class(class: &str) -> String {
  merge_classes(classes([Some(CAROUSEL_VIEWPORT_BASE_CLASS)]), class)
}

pub fn carousel_content_class(orientation: CarouselOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    CarouselOrientation::Horizontal => "-ml-4",
    CarouselOrientation::Vertical => "-mt-4 flex-col",
  };

  merge_classes(classes([Some(CAROUSEL_CONTENT_BASE_CLASS), Some(orientation_class)]), class)
}

pub fn carousel_item_class(
  orientation: CarouselOrientation,
  selected: bool,
  class: &str,
) -> String {
  let orientation_class = match orientation {
    CarouselOrientation::Horizontal => "pl-4",
    CarouselOrientation::Vertical => "pt-4",
  };

  merge_classes(
    classes([
      Some(CAROUSEL_ITEM_BASE_CLASS),
      Some(orientation_class),
      selected.then_some("data-selected"),
    ]),
    class,
  )
}

pub fn carousel_control_class(disabled: bool, class: &str) -> String {
  merge_classes(
    classes([
      Some(CAROUSEL_CONTROL_BASE_CLASS),
      disabled.then_some("pointer-events-none opacity-50"),
    ]),
    class,
  )
}

pub fn carousel_indicator_class(selected: bool, class: &str) -> String {
  merge_classes(
    classes([
      Some(CAROUSEL_INDICATOR_BASE_CLASS),
      Some(if selected { "bg-primary" } else { "bg-muted-foreground/40" }),
    ]),
    class,
  )
}

/// A slide step requested from the keyboard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CarouselStep {
  Previous,
  Next,
}

/// Maps an arrow key to a step along the carousel's orientation.
pub fn carousel_key_step(key: &str, orientation: CarouselOrientation) -> Option<CarouselStep> {
  match (orientation, key) {
    (CarouselOrientation::Horizontal, "ArrowLeft") | (CarouselOrientation::Vertical, "ArrowUp") => {
      Some(CarouselStep::Previous)
    }
    (CarouselOrientation::Horizontal, "ArrowRight")
    | (CarouselOrientation::Vertical, "ArrowDown") => Some(CarouselStep::Next),
    _ => None,
  }
}

/// The custom property that carries the selected index from the content to
/// its items.
pub const CAROUSEL_INDEX_PROPERTY: &str = "--dxui-carousel-index";

/// A percentage translate is relative to the item itself, so every item
/// moves by `index` item sizes whatever its basis.
pub fn carousel_item_transform(orientation: CarouselOrientation) -> &'static str {
  match orientation {
    CarouselOrientation::Horizontal => "translateX(calc(var(--dxui-carousel-index, 0) * -100%))",
    CarouselOrientation::Vertical => "translateY(calc(var(--dxui-carousel-index, 0) * -100%))",
  }
}

#[component]
pub fn Carousel(
  #[props(default)] orientation: CarouselOrientation,
  #[props(default)] on_key_step: Option<EventHandler<CarouselStep>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = carousel_class(&class);

  rsx! {
    div {
      role: "region",
      class,
      "aria-roledescription": "carousel",
      "data-orientation": carousel_orientation_attribute(orientation),
      onkeydown: move |event: KeyboardEvent| {
        if let Some(step) = carousel_key_step(&event.key().to_string(), orientation) {
          // Up and Down would otherwise scroll the page.
          event.prevent_default();
          if let Some(handler) = on_key_step {
            handler.call(step);
          }
        }
      },
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn CarouselViewport(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = carousel_viewport_class(&class);

  rsx! {
    div {
      class,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn CarouselContent(
  #[props(default)] orientation: CarouselOrientation,
  #[props(default)] index: usize,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = carousel_content_class(orientation, &class);
  // A style property merges with any `style` the app passes, where a `style`
  // string would replace it.
  let mut attributes = attributes;
  attributes.push(Attribute::new(CAROUSEL_INDEX_PROPERTY, index.to_string(), Some("style"), false));

  rsx! {
    div {
      class,
      "data-orientation": carousel_orientation_attribute(orientation),
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn CarouselItem(
  #[props(default)] orientation: CarouselOrientation,
  #[props(default)] selected: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = carousel_item_class(orientation, selected, &class);

  rsx! {
    div {
      role: "group",
      class,
      transform: carousel_item_transform(orientation),
      "aria-roledescription": "slide",
      "data-orientation": carousel_orientation_attribute(orientation),
      "data-selected": selected.to_string(),
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn CarouselPrevious(
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = carousel_control_class(disabled, &class);
  let aria_label = default_attribute(&attributes, "aria-label", "Previous slide");

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": aria_label,
      onclick: move |event| {
        if !disabled {
          if let Some(handler) = onclick {
            handler.call(event);
          }
        }
      },
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn CarouselNext(
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = carousel_control_class(disabled, &class);
  let aria_label = default_attribute(&attributes, "aria-label", "Next slide");

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": aria_label,
      onclick: move |event| {
        if !disabled {
          if let Some(handler) = onclick {
            handler.call(event);
          }
        }
      },
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn CarouselIndicator(
  #[props(default)] selected: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
  let class = carousel_indicator_class(selected, &class);
  let aria_label = default_attribute(&attributes, "aria-label", "Go to slide");

  rsx! {
    button {
      r#type: "button",
      class,
      "aria-current": selected.to_string(),
      "aria-label": aria_label,
      "data-selected": selected.to_string(),
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
      },
      ..attributes,
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

  fn aria_labels(html: &str) -> Vec<&str> {
    html.split("aria-label=\"").skip(1).filter_map(|rest| rest.split('"').next()).collect()
  }

  #[test]
  fn ssr_renders_only_a_passed_aria_label() {
    fn app() -> Element {
      rsx! {
        CarouselPrevious { "aria-label": "Diapositive précédente" }
        CarouselNext { "aria-label": "Diapositive suivante" }
        CarouselIndicator { "aria-label": "Show product 1" }
      }
    }

    assert_eq!(
      aria_labels(&render(app)),
      ["Diapositive précédente", "Diapositive suivante", "Show product 1"]
    );
  }

  #[test]
  fn ssr_renders_the_default_attribute() {
    fn app() -> Element {
      rsx! {
        CarouselPrevious {}
        CarouselNext {}
        CarouselIndicator {}
      }
    }

    assert_eq!(aria_labels(&render(app)), ["Previous slide", "Next slide", "Go to slide"]);
  }

  #[test]
  fn carousel_content_class_reflects_orientation() {
    let actual = carousel_content_class(CarouselOrientation::Vertical, "gap-4");

    assert!(actual.contains(CAROUSEL_CONTENT_BASE_CLASS));
    assert!(actual.contains("-mt-4 flex-col"));
    assert!(actual.ends_with("gap-4"));
  }

  #[test]
  fn carousel_item_class_reflects_state() {
    let actual = carousel_item_class(CarouselOrientation::Horizontal, true, "basis-1/2");

    assert_eq!(
      actual,
      "min-w-0 shrink-0 grow-0 transition-transform duration-300 motion-reduce:transition-none data-[orientation=horizontal]:pl-4 data-[orientation=vertical]:pt-4 pl-4 data-selected basis-1/2"
    );
    assert!(actual.contains("pl-4"));
    assert!(actual.contains("data-selected"));
    assert!(actual.ends_with("basis-1/2"));
  }

  #[test]
  fn carousel_key_step_follows_orientation() {
    let horizontal = CarouselOrientation::Horizontal;
    let vertical = CarouselOrientation::Vertical;

    assert_eq!(carousel_key_step("ArrowLeft", horizontal), Some(CarouselStep::Previous));
    assert_eq!(carousel_key_step("ArrowRight", horizontal), Some(CarouselStep::Next));
    assert_eq!(carousel_key_step("ArrowDown", horizontal), None);
    assert_eq!(carousel_key_step("ArrowUp", vertical), Some(CarouselStep::Previous));
    assert_eq!(carousel_key_step("ArrowDown", vertical), Some(CarouselStep::Next));
    assert_eq!(carousel_key_step("ArrowRight", vertical), None);
  }

  #[test]
  fn carousel_items_translate_by_the_content_index() {
    assert!(
      carousel_item_transform(CarouselOrientation::Horizontal).contains(CAROUSEL_INDEX_PROPERTY)
    );
    assert!(carousel_item_transform(CarouselOrientation::Horizontal).starts_with("translateX("));
    assert!(carousel_item_transform(CarouselOrientation::Vertical).starts_with("translateY("));
  }

  #[test]
  fn carousel_primitives_are_reexported() {
    let state = CarouselState::new(2, 3).with_looping(true).next();

    assert_eq!(state.index, 0);
    assert!(carousel_can_go_previous(state.index, state.item_count, state.looping));
    assert_eq!(carousel_previous(state.index, state.item_count, state.looping), 2);
  }
}
