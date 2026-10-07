//! Indicator: places a badge, dot, or other small element on a corner of another
//! element, such as an unread count on a button.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// The corner an `IndicatorItem` sits on, in logical directions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IndicatorPlacement {
  /// Top corner at the inline end: top right, or top left in right-to-left.
  #[default]
  TopEnd,
  /// Top corner at the inline start.
  TopStart,
  /// Bottom corner at the inline end.
  BottomEnd,
  /// Bottom corner at the inline start.
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

const INDICATOR_BASE_CLASS: &str = "relative inline-flex";
const INDICATOR_ITEM_BASE_CLASS: &str = "absolute z-10";

/// Classes for the wrapper: a positioning context for its items, then `class` merged
/// over it.
pub fn indicator_class(class: &str) -> String {
  merge_classes(classes([Some(INDICATOR_BASE_CLASS)]), class)
}

/// Classes for an item: absolute positioning, the placement's corner, then `class`
/// merged over them.
pub fn indicator_item_class(placement: IndicatorPlacement, class: &str) -> String {
  merge_classes(classes([Some(INDICATOR_ITEM_BASE_CLASS), Some(placement.class())]), class)
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn indicator_item_class_reflects_placement() {
    let actual = indicator_item_class(IndicatorPlacement::BottomStart, "size-3");

    assert!(actual.contains(INDICATOR_ITEM_BASE_CLASS));
    assert!(
      actual.contains("bottom-0 start-0 translate-y-1/2 -translate-x-1/2 rtl:translate-x-1/2")
    );
    assert!(actual.ends_with("size-3"));
    assert!(IndicatorPlacement::default().class().starts_with("top-0 end-0"));
  }

  #[test]
  fn indicator_class_appends_user_class() {
    assert_eq!(indicator_class("align-middle"), "relative inline-flex align-middle");
  }
}
