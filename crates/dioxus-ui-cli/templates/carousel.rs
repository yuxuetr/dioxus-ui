use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CarouselOrientation {
  #[default]
  Horizontal,
  Vertical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CarouselState {
  pub index: usize,
  pub item_count: usize,
  pub looping: bool,
}

impl CarouselState {
  pub const fn new(index: usize, item_count: usize) -> Self {
    Self {
      index,
      item_count,
      looping: false,
    }
  }

  pub const fn with_looping(mut self, looping: bool) -> Self {
    self.looping = looping;
    self
  }

  pub fn clamped(self) -> Self {
    Self {
      index: carousel_clamp_index(self.index, self.item_count),
      ..self
    }
  }

  pub fn next(self) -> Self {
    Self {
      index: carousel_next(self.index, self.item_count, self.looping),
      ..self
    }
  }

  pub fn previous(self) -> Self {
    Self {
      index: carousel_previous(self.index, self.item_count, self.looping),
      ..self
    }
  }
}

pub const CAROUSEL_BASE_CLASS: &str = "relative";
pub const CAROUSEL_VIEWPORT_BASE_CLASS: &str = "overflow-hidden";
pub const CAROUSEL_CONTENT_BASE_CLASS: &str = "flex data-orientation-horizontal:-ml-4 data-orientation-vertical:-mt-4 data-orientation-vertical:flex-col";
pub const CAROUSEL_ITEM_BASE_CLASS: &str = "min-w-0 shrink-0 grow-0 basis-full transition-transform duration-300 motion-reduce:transition-none data-orientation-horizontal:pl-4 data-orientation-vertical:pt-4";
pub const CAROUSEL_CONTROL_BASE_CLASS: &str = "inline-flex h-9 w-9 items-center justify-center rounded-full border border-zinc-200 bg-white text-sm shadow-sm transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const CAROUSEL_INDICATOR_BASE_CLASS: &str = "h-2 w-2 rounded-full bg-zinc-300 transition-colors data-selected:bg-zinc-950";

pub fn carousel_orientation_attribute(orientation: CarouselOrientation) -> &'static str {
  match orientation {
    CarouselOrientation::Horizontal => "horizontal",
    CarouselOrientation::Vertical => "vertical",
  }
}

pub fn carousel_clamp_index(index: usize, item_count: usize) -> usize {
  if item_count == 0 {
    0
  } else {
    index.min(item_count - 1)
  }
}

pub fn carousel_can_go_next(index: usize, item_count: usize, looping: bool) -> bool {
  item_count > 1 && (looping || index + 1 < item_count)
}

pub fn carousel_can_go_previous(index: usize, item_count: usize, looping: bool) -> bool {
  item_count > 1 && (looping || index > 0)
}

pub fn carousel_next(index: usize, item_count: usize, looping: bool) -> usize {
  if item_count == 0 {
    return 0;
  }

  let index = carousel_clamp_index(index, item_count);

  if index + 1 < item_count {
    index + 1
  } else if looping {
    0
  } else {
    index
  }
}

pub fn carousel_previous(index: usize, item_count: usize, looping: bool) -> usize {
  if item_count == 0 {
    return 0;
  }

  let index = carousel_clamp_index(index, item_count);

  if index > 0 {
    index - 1
  } else if looping {
    item_count - 1
  } else {
    index
  }
}

pub fn carousel_class(class: &str) -> String {
  classes([Some(CAROUSEL_BASE_CLASS), Some(class)])
}

pub fn carousel_viewport_class(class: &str) -> String {
  classes([Some(CAROUSEL_VIEWPORT_BASE_CLASS), Some(class)])
}

pub fn carousel_content_class(orientation: CarouselOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    CarouselOrientation::Horizontal => "-ml-4",
    CarouselOrientation::Vertical => "-mt-4 flex-col",
  };

  classes([
    Some(CAROUSEL_CONTENT_BASE_CLASS),
    Some(orientation_class),
    Some(class),
  ])
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

  classes([
    Some(CAROUSEL_ITEM_BASE_CLASS),
    Some(orientation_class),
    selected.then_some("data-selected"),
    Some(class),
  ])
}

pub fn carousel_control_class(disabled: bool, class: &str) -> String {
  classes([
    Some(CAROUSEL_CONTROL_BASE_CLASS),
    disabled.then_some("pointer-events-none opacity-50"),
    Some(class),
  ])
}

pub fn carousel_indicator_class(selected: bool, class: &str) -> String {
  classes([
    Some(CAROUSEL_INDICATOR_BASE_CLASS),
    selected.then_some("bg-zinc-950"),
    Some(class),
  ])
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

/// Keeps a control's default label only when the app passed none. The
/// browser applies the later spread value anyway, but SSR writes both
/// attributes and an HTML parser keeps the first.
fn default_label(attributes: &[Attribute], label: &'static str) -> Option<&'static str> {
  (!attributes.iter().any(|attribute| attribute.name == "aria-label")).then_some(label)
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
  let aria_label = default_label(&attributes, "Previous slide");

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
  let aria_label = default_label(&attributes, "Next slide");

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
  let aria_label = default_label(&attributes, "Go to slide");

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
