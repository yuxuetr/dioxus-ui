//! Button: the main control for actions and form submission.
use crate::density::use_density;
use dioxus::prelude::*;
use dioxus_shadcn_core::{UiDensity, classes, merge_classes};

/// The look of a `Button`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonVariant {
  /// The primary fill, for the main action.
  #[default]
  Primary,
  /// The secondary fill, for other actions.
  Secondary,
  /// The destructive fill, for actions that delete or cannot be undone.
  Destructive,
  /// A border on the background.
  Outline,
  /// No fill until hovered.
  Ghost,
  /// Looks like a link, underlined on hover.
  Link,
}

impl ButtonVariant {
  /// The variant's color and hover classes.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Primary => "bg-primary text-primary-foreground hover:bg-primary/90",
      Self::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
      Self::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
      Self::Outline => "border border-input bg-background hover:bg-accent",
      Self::Ghost => "bg-transparent hover:bg-accent",
      Self::Link => {
        "bg-transparent text-foreground decoration-primary underline-offset-4 hover:underline"
      }
    }
  }
}

/// The height and padding of a `Button`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonSize {
  /// Small: 32 pixels high.
  Sm,
  /// Medium: 40 pixels high.
  #[default]
  Md,
  /// Large: 48 pixels high.
  Lg,
  /// A 40 pixel square, for a button that holds only an icon.
  Icon,
}

impl ButtonSize {
  /// The size's height, padding, and text size classes.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "h-8 px-3 text-sm",
      Self::Md => "h-10 px-4 text-sm",
      Self::Lg => "h-12 px-6 text-base",
      Self::Icon => "h-10 w-10",
    }
  }
}

const BUTTON_BASE_CLASS: &str = "inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:pointer-events-none disabled:opacity-50";

/// Classes for the button: base classes, the variant's, the size's, a minimum
/// height for `density` (and a minimum width at Touch), then `class` merged over them.
pub fn button_class(
  variant: ButtonVariant,
  size: ButtonSize,
  density: UiDensity,
  class: &str,
) -> String {
  let density_class = match density {
    UiDensity::Compact => "min-h-8",
    UiDensity::Comfortable => "min-h-10",
    UiDensity::Touch => "min-h-12 min-w-11",
  };

  merge_classes(
    classes([
      Some(BUTTON_BASE_CLASS),
      Some(variant.class()),
      Some(size.class()),
      Some(density_class),
    ]),
    class,
  )
}

/// Takes its density from the nearest `DensityProvider` (RFC 0078). Calls
/// `onclick` on a click, Enter, or Space. Other attributes, such as
/// `type`, `name`, and `aria-label`, are passed to the button, which keeps the
/// native `submit` type inside a form unless `r#type` says otherwise.
#[component]
pub fn Button(
  #[props(default)] variant: ButtonVariant,
  #[props(default)] size: ButtonSize,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = button_class(variant, size, use_density(), &class);

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
  use crate::density::DensityProvider;

  #[test]
  fn button_takes_its_density_from_the_provider() {
    fn app() -> Element {
      rsx! {
        Button { "Plain" }
        DensityProvider { density: UiDensity::Touch, Button { "Touch" } }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert_eq!(html.matches("min-h-12").count(), 1, "{html}");
    assert_eq!(html.matches("min-h-10").count(), 1);
  }

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

  #[test]
  fn link_text_uses_the_foreground_color() {
    // Theme presets may have a light primary that cannot be text on the page
    // background (RFC 0057), so only the underline takes the primary color.
    let actual = button_class(ButtonVariant::Link, ButtonSize::Md, UiDensity::Comfortable, "");

    assert!(actual.contains("text-foreground decoration-primary"));
    assert!(!actual.contains("text-primary"));
  }
}
