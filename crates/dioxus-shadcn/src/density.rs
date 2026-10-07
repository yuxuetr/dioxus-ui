//! The density a page's interactive controls take from `DensityProvider`
//! (RFC 0078).

use dioxus::prelude::*;
use dioxus_shadcn_core::{UiDensity, merge_classes};

/// Gives every interactive component inside it `density`. Without one they
/// render at `UiDensity::Comfortable`, the shadcn/ui sizes. It renders no
/// element of its own.
#[component]
pub fn DensityProvider(density: UiDensity, children: Element) -> Element {
  let mut shared = use_context_provider(|| Signal::new(density));
  if *shared.peek() != density {
    shared.set(density);
  }

  rsx! { {children} }
}

/// The density of the nearest `DensityProvider`, or `Comfortable`.
pub fn use_density() -> UiDensity {
  try_use_context::<Signal<UiDensity>>().map(|density| density()).unwrap_or_default()
}

/// The Touch floor for a control that holds text: 44 CSS pixels high, and as
/// wide, so an icon button stays square.
pub fn density_control_class(density: UiDensity) -> &'static str {
  match density {
    UiDensity::Touch => "min-h-11 min-w-11",
    UiDensity::Compact | UiDensity::Comfortable => "",
  }
}

/// The Touch hit area for a control drawn smaller than a target: a 44 by 44
/// CSS pixel `after:` pseudo-element centered on it, which takes the press
/// without changing the look. `relative` positions it; an inline `position`,
/// such as a slider thumb's, still wins.
pub fn density_hit_area_class(density: UiDensity) -> &'static str {
  match density {
    UiDensity::Touch => {
      "relative after:absolute after:left-1/2 after:top-1/2 after:size-11 after:-translate-x-1/2 after:-translate-y-1/2"
    }
    UiDensity::Compact | UiDensity::Comfortable => "",
  }
}

/// Merges the app's `class` over `density_class`, so the app's classes win
/// where they conflict, as they do over a component's own classes.
#[allow(dead_code)] // Features with no interactive control leave it unused.
pub(crate) fn with_density(density_class: &str, class: &str) -> String {
  if density_class.is_empty() {
    class.to_string()
  } else {
    merge_classes(density_class.to_string(), class)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn only_touch_adds_classes() {
    for density in [UiDensity::Compact, UiDensity::Comfortable] {
      assert_eq!(density_control_class(density), "");
      assert_eq!(density_hit_area_class(density), "");
    }
    assert_eq!(density_control_class(UiDensity::Touch), "min-h-11 min-w-11");
    assert!(density_hit_area_class(UiDensity::Touch).contains("after:size-11"));
  }

  #[test]
  fn the_app_class_comes_last() {
    assert_eq!(with_density("min-h-11 min-w-11", "min-h-8"), "min-w-11 min-h-8");
    assert_eq!(with_density("", "px-2"), "px-2");
    assert_eq!(with_density("min-h-11", ""), "min-h-11");
  }

  #[test]
  fn components_read_the_nearest_provider() {
    thread_local! {
      static SEEN: std::cell::RefCell<Vec<UiDensity>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    #[component]
    fn Probe() -> Element {
      let density = use_density();
      SEEN.with(|seen| seen.borrow_mut().push(density));
      rsx! {}
    }
    fn app() -> Element {
      rsx! {
        Probe {}
        DensityProvider { density: UiDensity::Touch, Probe {} }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();

    assert_eq!(SEEN.with(|seen| seen.borrow().clone()), [UiDensity::Comfortable, UiDensity::Touch]);
  }
}
