use dioxus::prelude::*;
use super::utils::{classes, UiDensity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonVariant {
  Primary,
  Secondary,
  Destructive,
  Outline,
  Ghost,
  Link,
}

impl ButtonVariant {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Primary => "bg-blue-600 text-white hover:bg-blue-700",
      Self::Secondary => "bg-zinc-100 text-zinc-900 hover:bg-zinc-200",
      Self::Destructive => "bg-red-600 text-white hover:bg-red-700",
      Self::Outline => "border border-zinc-200 bg-white hover:bg-zinc-100",
      Self::Ghost => "bg-transparent hover:bg-zinc-100",
      Self::Link => "bg-transparent text-blue-600 underline-offset-4 hover:underline",
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonSize {
  Sm,
  Md,
  Lg,
  Icon,
}

impl ButtonSize {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "h-8 px-3 text-sm",
      Self::Md => "h-10 px-4 text-sm",
      Self::Lg => "h-12 px-6 text-base",
      Self::Icon => "h-10 w-10",
    }
  }
}

pub const BUTTON_BASE_CLASS: &str = "inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:pointer-events-none disabled:opacity-50";

pub fn button_class(
  variant: ButtonVariant,
  size: ButtonSize,
  density: UiDensity,
  class: &str,
) -> String {
  let density_class = match density {
    UiDensity::Compact => "min-h-8",
    UiDensity::Comfortable => "min-h-10",
    UiDensity::Touch => "min-h-12",
  };

  classes([
    Some(BUTTON_BASE_CLASS),
    Some(variant.class()),
    Some(size.class()),
    Some(density_class),
    Some(class),
  ])
}

/// Calls `onclick` on a click, Enter, or Space. Other attributes, such as
/// `type`, `name`, and `aria-label`, are passed to the button, which keeps the
/// native `submit` type inside a form unless `r#type` says otherwise.
#[component]
pub fn Button(
  #[props(default = ButtonVariant::Primary)] variant: ButtonVariant,
  #[props(default = ButtonSize::Md)] size: ButtonSize,
  #[props(default)] density: UiDensity,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = button_class(variant, size, density, &class);

  rsx! {
    button {
      class,
      disabled,
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
      },
      ..attributes,
      {children}
    }
  }
}
