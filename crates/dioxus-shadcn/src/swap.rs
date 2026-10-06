use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SwapEffect {
  #[default]
  Fade,
  Rotate,
  Flip,
}

impl SwapEffect {
  /// The layer's transform and opacity when shown and when hidden.
  pub const fn layer_class(self, shown: bool) -> &'static str {
    match (self, shown) {
      (Self::Fade, true) => "opacity-100",
      (Self::Fade, false) => "opacity-0",
      (Self::Rotate, true) => "rotate-0 opacity-100",
      (Self::Rotate, false) => "rotate-90 opacity-0",
      (Self::Flip, true) => "[transform:rotateY(0deg)]",
      (Self::Flip, false) => "[transform:rotateY(180deg)]",
    }
  }
}

pub const SWAP_BASE_CLASS: &str = "relative inline-grid cursor-pointer place-items-center rounded-md [perspective:600px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
pub const SWAP_LAYER_BASE_CLASS: &str = "col-start-1 row-start-1 transition duration-300 [backface-visibility:hidden] motion-reduce:transition-none";

pub fn swap_class(class: &str) -> String {
  merge_classes(classes([Some(SWAP_BASE_CLASS)]), class)
}

pub fn swap_layer_class(effect: SwapEffect, shown: bool) -> String {
  classes([Some(SWAP_LAYER_BASE_CLASS), Some(effect.layer_class(shown))])
}

/// A toggle button that shows `on` when `active` and `off` otherwise. A
/// press calls `on_active_change` with the new state; the hidden layer is
/// `aria-hidden`. Name it with `aria-label` when the layers are icons.
#[component]
pub fn Swap(
  #[props(default)] active: bool,
  on: Element,
  off: Element,
  #[props(default)] effect: SwapEffect,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] on_active_change: Option<EventHandler<bool>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
  let class = swap_class(&class);

  rsx! {
    button {
      class,
      r#type: "button",
      disabled,
      "aria-pressed": "{active}",
      "data-state": if active { "on" } else { "off" },
      onclick: move |_| {
        if let Some(handler) = on_active_change {
          handler.call(!active);
        }
      },
      ..attributes,
      span {
        class: swap_layer_class(effect, active),
        "aria-hidden": (!active).then_some("true"),
        {on}
      }
      span {
        class: swap_layer_class(effect, !active),
        "aria-hidden": active.then_some("true"),
        {off}
      }
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
  fn ssr_swap_is_a_pressed_button_hiding_the_off_layer() {
    fn app() -> Element {
      rsx! {
        Swap {
          active: true,
          effect: SwapEffect::Rotate,
          "aria-label": "Menu",
          on: rsx! { "Close" },
          off: rsx! { "Open" },
        }
      }
    }
    let html = render(app);

    assert!(html.starts_with("<button"));
    assert!(html.contains("aria-pressed=\"true\""));
    assert!(html.contains("data-state=\"on\""));
    assert!(html.contains("rotate-0 opacity-100\">Close</span>"));
    assert!(html.contains("rotate-90 opacity-0\" aria-hidden=\"true\">Open</span>"));
  }

  #[test]
  fn layer_class_reflects_effect_and_visibility() {
    assert!(swap_layer_class(SwapEffect::Fade, false).ends_with("opacity-0"));
    assert!(swap_layer_class(SwapEffect::Flip, true).contains("[transform:rotateY(0deg)]"));
    assert!(SWAP_LAYER_BASE_CLASS.contains("motion-reduce:transition-none"));
  }
}
