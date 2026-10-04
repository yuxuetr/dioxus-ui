use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToggleVariant {
  #[default]
  Default,
  Outline,
}

impl ToggleVariant {
  pub const fn class(self, pressed: bool) -> &'static str {
    match (self, pressed) {
      (Self::Default, true) => "bg-zinc-900 text-white hover:bg-zinc-800",
      (Self::Default, false) => "bg-transparent hover:bg-zinc-100",
      (Self::Outline, true) => "border border-blue-600 bg-blue-50 text-blue-700",
      (Self::Outline, false) => "border border-zinc-200 bg-white hover:bg-zinc-100",
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToggleSize {
  Sm,
  #[default]
  Md,
  Lg,
}

impl ToggleSize {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "h-8 px-2 text-sm",
      Self::Md => "h-10 px-3 text-sm",
      Self::Lg => "h-11 px-4 text-base",
    }
  }
}

pub const TOGGLE_BASE_CLASS: &str = "inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";

pub fn toggle_class(
  variant: ToggleVariant,
  size: ToggleSize,
  pressed: bool,
  class: &str,
) -> String {
  classes([
    Some(TOGGLE_BASE_CLASS),
    Some(variant.class(pressed)),
    Some(size.class()),
    Some(class),
  ])
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
  let class = toggle_class(variant, size, pressed, &class);

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
