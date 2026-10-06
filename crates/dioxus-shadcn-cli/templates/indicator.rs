use super::utils::classes;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IndicatorPlacement {
  #[default]
  TopEnd,
  TopStart,
  BottomEnd,
  BottomStart,
}

impl IndicatorPlacement {
  /// Logical insets put the item on the inline end or start, and the
  /// translate pushes it half outside, mirrored for right-to-left.
  pub const fn class(self) -> &'static str {
    match self {
      Self::TopEnd => "top-0 end-0 -translate-y-1/2 translate-x-1/2 rtl:-translate-x-1/2",
      Self::TopStart => "top-0 start-0 -translate-y-1/2 -translate-x-1/2 rtl:translate-x-1/2",
      Self::BottomEnd => "bottom-0 end-0 translate-y-1/2 translate-x-1/2 rtl:-translate-x-1/2",
      Self::BottomStart => "bottom-0 start-0 translate-y-1/2 -translate-x-1/2 rtl:translate-x-1/2",
    }
  }
}

pub const INDICATOR_BASE_CLASS: &str = "relative inline-flex";
pub const INDICATOR_ITEM_BASE_CLASS: &str = "absolute z-10";

pub fn indicator_class(class: &str) -> String {
  classes([Some(INDICATOR_BASE_CLASS), Some(class)])
}

pub fn indicator_item_class(placement: IndicatorPlacement, class: &str) -> String {
  classes([Some(INDICATOR_ITEM_BASE_CLASS), Some(placement.class()), Some(class)])
}

/// Wraps an element so an `IndicatorItem` can sit on one of its corners.
#[component]
pub fn Indicator(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = indicator_class(&class);

  rsx! {
    span { class, ..attributes, {children} }
  }
}

/// Content on a corner of the `Indicator`, such as a count `Badge` or a
/// `Status` dot. `IndicatorPlacement::TopEnd` is the default.
#[component]
pub fn IndicatorItem(
  #[props(default)] placement: IndicatorPlacement,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = indicator_item_class(placement, &class);

  rsx! {
    span { class, ..attributes, {children} }
  }
}
