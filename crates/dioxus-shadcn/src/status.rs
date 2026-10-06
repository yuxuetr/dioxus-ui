use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

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
  merge_classes(
    classes([Some(STATUS_BASE_CLASS), Some(variant.class()), Some(size.class())]),
    class,
  )
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

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_labelled_status_is_an_image() {
    fn app() -> Element {
      rsx! { Status { variant: StatusVariant::Success, label: "Online" } }
    }
    let html = render(app);

    assert!(html.contains("role=\"img\""));
    assert!(html.contains("aria-label=\"Online\""));
    assert!(!html.contains("aria-hidden"));
    assert!(html.contains("bg-success size-2"));
  }

  #[test]
  fn ssr_unlabelled_status_is_hidden() {
    fn app() -> Element {
      rsx! { Status {} }
    }
    let html = render(app);

    assert!(html.contains("aria-hidden=\"true\""));
    assert!(!html.contains("role="));
    assert!(html.contains("bg-muted-foreground"));
  }

  #[test]
  fn status_class_reflects_size() {
    assert!(status_class(StatusVariant::Warning, StatusSize::Lg, "").contains("bg-warning size-3"));
  }
}
