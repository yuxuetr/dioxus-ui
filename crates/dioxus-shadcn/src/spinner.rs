//! Spinner: a compact indicator that tells the user something is loading.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// How large the spinner renders: its diameter and ring width.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpinnerSize {
  /// 16 pixels across, for inline text and small buttons.
  Sm,
  /// 20 pixels across.
  #[default]
  Md,
  /// 24 pixels across, with a thicker ring.
  Lg,
}

impl SpinnerSize {
  /// The diameter and ring width for this size.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "h-4 w-4 border-2",
      Self::Md => "h-5 w-5 border-2",
      Self::Lg => "h-6 w-6 border-[3px]",
    }
  }
}

const SPINNER_BASE_CLASS: &str =
  "inline-block shrink-0 animate-spin rounded-full border-border border-t-foreground";

/// Classes for the spinner: base classes, the size's, then `class` merged over them.
pub fn spinner_class(size: SpinnerSize, class: &str) -> String {
  merge_classes(classes([Some(SPINNER_BASE_CLASS), Some(size.class())]), class)
}

#[component]
pub fn Spinner(
  #[props(default)] size: SpinnerSize,
  #[props(default = "Loading".to_string())] label: String,
  #[props(default)] class: String,
) -> Element {
  let class = spinner_class(size, &class);

  rsx! {
    span {
      class,
      role: "status",
      "aria-label": "{label}",
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn spinner_class_includes_size_and_user_class() {
    let actual = spinner_class(SpinnerSize::Lg, "text-primary");

    assert!(actual.contains(SPINNER_BASE_CLASS));
    assert!(actual.contains("h-6 w-6 border-[3px]"));
    assert!(actual.ends_with("text-primary"));
  }

  #[test]
  fn spinner_class_preserves_static_tailwind_tokens() {
    let actual = spinner_class(SpinnerSize::Sm, "");

    assert!(actual.contains("animate-spin"));
    assert!(actual.contains("border-t-foreground"));
    assert!(actual.contains("h-4 w-4 border-2"));
  }
}
