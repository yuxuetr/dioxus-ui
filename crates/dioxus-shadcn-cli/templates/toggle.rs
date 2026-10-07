//! Toggle: a controlled pressed button for compact commands and formatting
//! controls.
use super::utils::{classes, merge_classes};
use super::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;

/// The look of a `Toggle`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToggleVariant {
  /// No fill until hovered or pressed.
  #[default]
  Default,
  /// A border, filled when pressed.
  Outline,
}

impl ToggleVariant {
  /// The variant's colors, pressed or not.
  pub const fn class(self, pressed: bool) -> &'static str {
    match (self, pressed) {
      (Self::Default, true) => "bg-accent text-accent-foreground",
      (Self::Default, false) => "bg-transparent hover:bg-accent",
      (Self::Outline, true) => "border border-input bg-accent text-accent-foreground",
      (Self::Outline, false) => "border border-input bg-background hover:bg-accent",
    }
  }
}

/// The height and padding of a `Toggle`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToggleSize {
  /// Small: 32 pixels high.
  Sm,
  /// Medium: 40 pixels high.
  #[default]
  Md,
  /// Large: 44 pixels high.
  Lg,
}

impl ToggleSize {
  /// The size's height, padding, and text size classes.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "h-8 px-2 text-sm",
      Self::Md => "h-10 px-3 text-sm",
      Self::Lg => "h-11 px-4 text-base",
    }
  }
}

const TOGGLE_BASE_CLASS: &str = "inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

/// Classes for the toggle: base classes, the variant's for `pressed`, the size's,
/// then `class` merged over them.
pub fn toggle_class(
  variant: ToggleVariant,
  size: ToggleSize,
  pressed: bool,
  class: &str,
) -> String {
  merge_classes(classes([Some(TOGGLE_BASE_CLASS), Some(variant.class(pressed)), Some(size.class())]), class)
}

/// A controlled toggle button. A click, Enter, or Space calls
/// `on_pressed_change` with the requested state, `!pressed`; the app passes it
/// back as `pressed`. Other attributes, such as `aria-label`, are passed to the
/// button.
#[component]
pub fn Toggle(
  #[props(default)] variant: ToggleVariant,
  #[props(default)] size: ToggleSize,
  #[props(default)] pressed: bool,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] on_pressed_change: Option<EventHandler<bool>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = toggle_class(variant, size, pressed, &with_density(density_control_class(use_density()), &class));

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-pressed": pressed.to_string(),
      onclick: move |_| {
        if let Some(handler) = on_pressed_change {
          handler.call(!pressed);
        }
      },
      ..attributes,
      {children}
    }
  }
}
