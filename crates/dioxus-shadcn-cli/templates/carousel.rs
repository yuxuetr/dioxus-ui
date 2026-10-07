//! Carousel: slides shown one at a time, with previous, next, and indicator
//! controls. The app owns the selected index; the parts show it and report clicks
//! and arrow keys.
use super::default_attribute::default_attribute;
use super::root_state::{Controllable, use_controllable, use_root_context};
use super::utils::{classes, merge_classes};
use super::density::{density_control_class, density_hit_area_class, use_density, with_density};
use dioxus::prelude::*;

pub use self::LayoutOrientation as CarouselOrientation;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LayoutOrientation {
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
    Self { index, item_count, looping: false }
  }

  pub const fn with_looping(mut self, looping: bool) -> Self {
    self.looping = looping;
    self
  }

  pub fn clamped(self) -> Self {
    Self { index: carousel_clamp_index(self.index, self.item_count), ..self }
  }

  pub fn next(self) -> Self {
    Self { index: carousel_next(self.index, self.item_count, self.looping), ..self }
  }

  pub fn previous(self) -> Self {
    Self { index: carousel_previous(self.index, self.item_count, self.looping), ..self }
  }
}

const CAROUSEL_BASE_CLASS: &str = "relative";
const CAROUSEL_VIEWPORT_BASE_CLASS: &str = "overflow-hidden";
const CAROUSEL_CONTENT_BASE_CLASS: &str = "flex data-[orientation=horizontal]:-ml-4 data-[orientation=vertical]:-mt-4 data-[orientation=vertical]:flex-col";
const CAROUSEL_ITEM_BASE_CLASS: &str = "min-w-0 shrink-0 grow-0 basis-full transition-transform duration-300 motion-reduce:transition-none data-[orientation=horizontal]:pl-4 data-[orientation=vertical]:pt-4";
const CAROUSEL_CONTROL_BASE_CLASS: &str = "inline-flex h-9 w-9 items-center justify-center rounded-full border border-border bg-background text-sm shadow-sm transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const CAROUSEL_INDICATOR_BASE_CLASS: &str = "h-2 w-2 rounded-full transition-colors";

fn carousel_orientation_attribute(orientation: CarouselOrientation) -> &'static str {
  match orientation {
    CarouselOrientation::Horizontal => "horizontal",
    CarouselOrientation::Vertical => "vertical",
  }
}

pub fn carousel_clamp_index(index: usize, item_count: usize) -> usize {
  if item_count == 0 { 0 } else { index.min(item_count - 1) }
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

fn carousel_class(class: &str) -> String {
  merge_classes(classes([Some(CAROUSEL_BASE_CLASS)]), class)
}

fn carousel_viewport_class(class: &str) -> String {
  merge_classes(classes([Some(CAROUSEL_VIEWPORT_BASE_CLASS)]), class)
}

fn carousel_content_class(orientation: CarouselOrientation, class: &str) -> String {
  let orientation_class = match orientation {
    CarouselOrientation::Horizontal => "-ml-4",
    CarouselOrientation::Vertical => "-mt-4 flex-col",
  };

  merge_classes(classes([Some(CAROUSEL_CONTENT_BASE_CLASS), Some(orientation_class)]), class)
}

fn carousel_item_class(
  orientation: CarouselOrientation,
  selected: bool,
  class: &str,
) -> String {
  let orientation_class = match orientation {
    CarouselOrientation::Horizontal => "pl-4",
    CarouselOrientation::Vertical => "pt-4",
  };

  merge_classes(classes([Some(CAROUSEL_ITEM_BASE_CLASS), Some(orientation_class), selected.then_some("data-selected")]), class)
}

fn carousel_control_class(disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(CAROUSEL_CONTROL_BASE_CLASS), disabled.then_some("pointer-events-none opacity-50")]), class)
}

fn carousel_indicator_class(selected: bool, class: &str) -> String {
  merge_classes(classes([Some(CAROUSEL_INDICATOR_BASE_CLASS), Some(if selected { "bg-primary" } else { "bg-muted-foreground/40" })]), class)
}

/// A slide step requested from the keyboard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CarouselStep {
  /// Back one slide.
  Previous,
  /// Forward one slide.
  Next,
}

/// Maps an arrow key to a step along the carousel's orientation.
fn carousel_key_step(key: &str, orientation: CarouselOrientation) -> Option<CarouselStep> {
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
const CAROUSEL_INDEX_PROPERTY: &str = "--dxui-carousel-index";

/// A percentage translate is relative to the item itself, so every item
/// moves by `index` item sizes whatever its basis.
fn carousel_item_transform(orientation: CarouselOrientation) -> &'static str {
  match orientation {
    CarouselOrientation::Horizontal => "translateX(calc(var(--dxui-carousel-index, 0) * -100%))",
    CarouselOrientation::Vertical => "translateY(calc(var(--dxui-carousel-index, 0) * -100%))",
  }
}

#[derive(Clone, Copy)]
struct CarouselContext {
  index: Controllable<usize>,
  count: usize,
  looping: bool,
  orientation: CarouselOrientation,
}

impl CarouselContext {
  fn current(&self) -> usize {
    carousel_clamp_index(self.index.get(), self.count)
  }

  fn step(&self, step: CarouselStep) {
    let current = self.current();
    self.index.set(match step {
      CarouselStep::Previous => carousel_previous(current, self.count, self.looping),
      CarouselStep::Next => carousel_next(current, self.count, self.looping),
    });
  }
}

/// The root of a carousel of `count` slides: it owns the selected slide's
/// index (RFC 0077). Pass `index` to control it, or `default_index` to start
/// it; `on_index_change` hears every change the user makes either way. The
/// arrow keys along `orientation` step it, as the previous and next buttons
/// do, wrapping when `looping`.
#[component]
pub fn Carousel(
  count: usize,
  #[props(default)] index: ReadSignal<Option<usize>>,
  #[props(default)] default_index: usize,
  #[props(default)] on_index_change: Option<EventHandler<usize>>,
  #[props(default)] looping: bool,
  #[props(default)] orientation: CarouselOrientation,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let index = use_controllable(move || index.cloned(), move || default_index, on_index_change);
  let context = CarouselContext { index, count, looping, orientation };
  // `count`, `looping`, and `orientation` may change, so the context is
  // refreshed on every render.
  let mut shared = use_context_provider(|| Signal::new(context));
  let stale = *shared.peek();
  if (stale.count, stale.looping, stale.orientation) != (count, looping, orientation) {
    shared.set(context);
  }
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
          shared.peek().step(step);
        }
      },
      ..attributes,
      {children}
    }
  }
}

fn use_carousel(part: &str) -> CarouselContext {
  use_root_context::<Signal<CarouselContext>>(part, "Carousel").cloned()
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
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let carousel = use_carousel("CarouselContent");
  let class = carousel_content_class(carousel.orientation, &class);
  // A style property merges with any `style` the app passes, where a `style`
  // string would replace it.
  let mut attributes = attributes;
  attributes.push(Attribute::new(
    CAROUSEL_INDEX_PROPERTY,
    carousel.current().to_string(),
    Some("style"),
    false,
  ));

  rsx! {
    div {
      class,
      "data-orientation": carousel_orientation_attribute(carousel.orientation),
      ..attributes,
      {children}
    }
  }
}

/// The slide at `index`, counting from zero.
#[component]
pub fn CarouselItem(
  index: usize,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let carousel = use_carousel("CarouselItem");
  let orientation = carousel.orientation;
  let selected = carousel.current() == index;
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

/// Steps to the previous slide; disabled on the first unless the carousel
/// loops, or with `disabled`.
#[component]
pub fn CarouselPrevious(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let carousel = use_carousel("CarouselPrevious");
  let disabled =
    disabled || !carousel_can_go_previous(carousel.current(), carousel.count, carousel.looping);
  let class = carousel_control_class(disabled, &with_density(density_control_class(use_density()), &class));
  let aria_label = default_attribute(&attributes, "aria-label", "Previous slide");

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": aria_label,
      onclick: move |_| {
        if !disabled {
          carousel.step(CarouselStep::Previous);
        }
      },
      ..attributes,
      {children}
    }
  }
}

/// Steps to the next slide; disabled on the last unless the carousel loops,
/// or with `disabled`.
#[component]
pub fn CarouselNext(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let carousel = use_carousel("CarouselNext");
  let disabled =
    disabled || !carousel_can_go_next(carousel.current(), carousel.count, carousel.looping);
  let class = carousel_control_class(disabled, &with_density(density_control_class(use_density()), &class));
  let aria_label = default_attribute(&attributes, "aria-label", "Next slide");

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": aria_label,
      onclick: move |_| {
        if !disabled {
          carousel.step(CarouselStep::Next);
        }
      },
      ..attributes,
      {children}
    }
  }
}

/// Selects the slide at `index`, and shows whether it is selected.
#[component]
pub fn CarouselIndicator(
  index: usize,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
  let carousel = use_carousel("CarouselIndicator");
  let selected = carousel.current() == index;
  let class = carousel_indicator_class(selected, &with_density(density_hit_area_class(use_density()), &class));
  let aria_label = default_attribute(&attributes, "aria-label", "Go to slide");

  rsx! {
    button {
      r#type: "button",
      class,
      "aria-current": selected.to_string(),
      "aria-label": aria_label,
      "data-selected": selected.to_string(),
      onclick: move |_| carousel.index.set(index),
      ..attributes,
    }
  }
}
