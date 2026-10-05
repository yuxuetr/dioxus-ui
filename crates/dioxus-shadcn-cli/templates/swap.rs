use dioxus::prelude::*;
use super::utils::classes;

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
  classes([Some(SWAP_BASE_CLASS), Some(class)])
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
