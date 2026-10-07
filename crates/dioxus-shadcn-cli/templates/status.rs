//! Status: a small colored dot for presence or health, such as online, away, or an
//! outage.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// The dot's color, by meaning.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StatusVariant {
  /// Muted, for offline or unknown.
  #[default]
  Neutral,
  /// The primary color.
  Primary,
  /// Green, for online or healthy.
  Success,
  /// Amber, for away or degraded.
  Warning,
  /// The info color, for a neutral notice.
  Info,
  /// Red, for busy or an outage.
  Destructive,
}

impl StatusVariant {
  /// The background color of this variant.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Neutral => "bg-muted-foreground",
      Self::Primary => "bg-primary",
      Self::Success => "bg-success",
      Self::Warning => "bg-warning",
      Self::Info => "bg-info",
      Self::Destructive => "bg-destructive",
    }
  }
}

/// The dot's diameter.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StatusSize {
  /// 6 CSS pixels.
  Sm,
  /// 8 CSS pixels.
  #[default]
  Md,
  /// 12 CSS pixels.
  Lg,
}

impl StatusSize {
  /// The width and height of this size.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "size-1.5",
      Self::Md => "size-2",
      Self::Lg => "size-3",
    }
  }
}

const STATUS_BASE_CLASS: &str = "inline-block shrink-0 rounded-full";

/// Classes for the dot: a round base, the variant's color, the size, then `class`
/// merged over them.
pub fn status_class(variant: StatusVariant, size: StatusSize, class: &str) -> String {
  merge_classes(classes([Some(STATUS_BASE_CLASS), Some(variant.class()), Some(size.class())]), class)
}

/// A colored dot. With `label` it is an image named by the label; without
/// it, it is hidden from assistive technology, for a dot beside text that
/// already says the state.
#[component]
pub fn Status(
  #[props(default)] variant: StatusVariant,
  #[props(default)] size: StatusSize,
  #[props(default)] label: Option<String>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
  let class = status_class(variant, size, &class);
  let hidden = label.is_none().then_some("true");
  let role = label.is_some().then_some("img");

  rsx! {
    span { class, role, "aria-label": label, "aria-hidden": hidden, ..attributes }
  }
}
