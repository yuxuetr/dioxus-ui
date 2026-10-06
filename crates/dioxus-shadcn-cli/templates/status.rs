use super::utils::classes;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StatusVariant {
  #[default]
  Neutral,
  Primary,
  Success,
  Warning,
  Info,
  Destructive,
}

impl StatusVariant {
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StatusSize {
  Sm,
  #[default]
  Md,
  Lg,
}

impl StatusSize {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "size-1.5",
      Self::Md => "size-2",
      Self::Lg => "size-3",
    }
  }
}

pub const STATUS_BASE_CLASS: &str = "inline-block shrink-0 rounded-full";

pub fn status_class(variant: StatusVariant, size: StatusSize, class: &str) -> String {
  classes([Some(STATUS_BASE_CLASS), Some(variant.class()), Some(size.class()), Some(class)])
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
