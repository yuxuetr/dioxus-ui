use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MarkerVariant {
  Default,
  Border,
  Separator,
}

pub const MARKER_BASE_CLASS: &str = "flex min-w-0 items-center gap-2 text-sm text-muted-foreground";
pub const MARKER_DEFAULT_CLASS: &str = "rounded-md bg-muted px-3 py-2";
pub const MARKER_BORDER_CLASS: &str = "rounded-md border border-border bg-background px-3 py-2";
pub const MARKER_SEPARATOR_CLASS: &str = "w-full py-2 before:h-px before:flex-1 before:bg-border after:h-px after:flex-1 after:bg-border";
pub const MARKER_ICON_BASE_CLASS: &str = "flex shrink-0 items-center justify-center text-muted-foreground";
pub const MARKER_CONTENT_BASE_CLASS: &str = "min-w-0 truncate";

impl MarkerVariant {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => MARKER_DEFAULT_CLASS,
      Self::Border => MARKER_BORDER_CLASS,
      Self::Separator => MARKER_SEPARATOR_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Border => "border",
      Self::Separator => "separator",
    }
  }
}

pub fn marker_class(variant: MarkerVariant, class: &str) -> String {
  classes([Some(MARKER_BASE_CLASS), Some(variant.class()), Some(class)])
}

pub fn marker_icon_class(class: &str) -> String {
  classes([Some(MARKER_ICON_BASE_CLASS), Some(class)])
}

pub fn marker_content_class(class: &str) -> String {
  classes([Some(MARKER_CONTENT_BASE_CLASS), Some(class)])
}

#[component]
pub fn Marker(
  #[props(default = MarkerVariant::Default)] variant: MarkerVariant,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = marker_class(variant, &class);

  rsx! {
    div {
      class,
      "data-variant": variant.attribute(),
      {children}
    }
  }
}

#[component]
pub fn MarkerIcon(
  #[props(default = true)] decorative: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = marker_icon_class(&class);

  rsx! {
    span {
      class,
      "aria-hidden": decorative.to_string(),
      {children}
    }
  }
}

#[component]
pub fn MarkerContent(#[props(default)] class: String, children: Element) -> Element {
  let class = marker_content_class(&class);

  rsx! {
    span {
      class,
      {children}
    }
  }
}
