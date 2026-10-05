use dioxus::prelude::*;
use dioxus_ui_core::{UiDensity, classes};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonVariant {
  #[default]
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
      Self::Primary => "bg-primary text-primary-foreground hover:bg-primary/90",
      Self::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
      Self::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
      Self::Outline => "border border-input bg-background hover:bg-accent",
      Self::Ghost => "bg-transparent hover:bg-accent",
      Self::Link => "bg-transparent text-primary underline-offset-4 hover:underline",
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonSize {
  Sm,
  #[default]
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
  #[props(default)] variant: ButtonVariant,
  #[props(default)] size: ButtonSize,
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn button_class_includes_variant_size_density_and_user_class() {
    let actual =
      button_class(ButtonVariant::Destructive, ButtonSize::Lg, UiDensity::Touch, "w-full");

    assert!(actual.contains(BUTTON_BASE_CLASS));
    assert!(actual.contains("bg-destructive text-destructive-foreground hover:bg-destructive/90"));
    assert!(actual.contains("h-12 px-6 text-base"));
    assert!(actual.contains("min-h-12"));
    assert!(actual.ends_with("w-full"));
  }

  #[test]
  fn button_class_preserves_static_tailwind_tokens() {
    let actual = button_class(ButtonVariant::Primary, ButtonSize::Md, UiDensity::Comfortable, "");

    assert!(actual.contains("bg-primary"));
    assert!(actual.contains("hover:bg-primary/90"));
    assert!(!actual.contains("{}"));
  }
}
