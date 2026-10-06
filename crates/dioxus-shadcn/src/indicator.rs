use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

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
  merge_classes(classes([Some(INDICATOR_BASE_CLASS)]), class)
}

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
