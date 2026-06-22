use dioxus::prelude::*;
use dioxus_ui_core::classes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpinnerSize {
  Sm,
  #[default]
  Md,
  Lg,
}

impl SpinnerSize {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "h-4 w-4 border-2",
      Self::Md => "h-5 w-5 border-2",
      Self::Lg => "h-6 w-6 border-[3px]",
    }
  }
}

pub const SPINNER_BASE_CLASS: &str =
  "inline-block shrink-0 animate-spin rounded-full border-zinc-200 border-t-zinc-900";

pub fn spinner_class(size: SpinnerSize, class: &str) -> String {
  classes([Some(SPINNER_BASE_CLASS), Some(size.class()), Some(class)])
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
    let actual = spinner_class(SpinnerSize::Lg, "text-blue-600");

    assert!(actual.contains(SPINNER_BASE_CLASS));
    assert!(actual.contains("h-6 w-6 border-[3px]"));
    assert!(actual.ends_with("text-blue-600"));
  }

  #[test]
  fn spinner_class_preserves_static_tailwind_tokens() {
    let actual = spinner_class(SpinnerSize::Sm, "");

    assert!(actual.contains("animate-spin"));
    assert!(actual.contains("border-t-zinc-900"));
    assert!(actual.contains("h-4 w-4 border-2"));
  }
}
