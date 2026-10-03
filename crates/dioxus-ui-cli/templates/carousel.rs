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
pub const CAROUSEL_ITEM_BASE_CLASS: &str = "min-w-0 shrink-0 grow-0 basis-full data-orientation-horizontal:pl-4 data-orientation-vertical:pt-4";
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

#[component]
pub fn Carousel(
  #[props(default)] orientation: CarouselOrientation,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = carousel_class(&class);

  rsx! {
    div {
      role: "region",
      class,
      "aria-roledescription": "carousel",
      "data-orientation": carousel_orientation_attribute(orientation),
      {children}
    }
  }
}

#[component]
pub fn CarouselViewport(#[props(default)] class: String, children: Element) -> Element {
  let class = carousel_viewport_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CarouselContent(
  #[props(default)] orientation: CarouselOrientation,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = carousel_content_class(orientation, &class);

  rsx! {
    div {
      class,
      "data-orientation": carousel_orientation_attribute(orientation),
      {children}
    }
  }
}

#[component]
pub fn CarouselItem(
  #[props(default)] orientation: CarouselOrientation,
  #[props(default)] selected: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = carousel_item_class(orientation, selected, &class);

  rsx! {
    div {
      role: "group",
      class,
      "aria-roledescription": "slide",
      "data-orientation": carousel_orientation_attribute(orientation),
      "data-selected": selected.to_string(),
      {children}
    }
  }
}

#[component]
pub fn CarouselPrevious(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = carousel_control_class(disabled, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": "Previous slide",
      {children}
    }
  }
}

#[component]
pub fn CarouselNext(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = carousel_control_class(disabled, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": "Next slide",
      {children}
    }
  }
}

#[component]
pub fn CarouselIndicator(
  #[props(default)] selected: bool,
  #[props(default)] class: String,
) -> Element {
  let class = carousel_indicator_class(selected, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      "aria-current": selected.to_string(),
      "aria-label": "Go to slide",
      "data-selected": selected.to_string(),
    }
  }
}
