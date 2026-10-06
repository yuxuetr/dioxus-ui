use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AlertVariant {
  #[default]
  Default,
  Destructive,
  Success,
  Warning,
  Info,
}

impl AlertVariant {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => "border-border bg-card text-foreground",
      Self::Destructive => "border-destructive bg-card text-destructive",
      // The status colors are too light to be text (RFC 0058), so they tint
      // the surface and border instead.
      Self::Success => "border-success/50 bg-success/10 text-foreground",
      Self::Warning => "border-warning/50 bg-warning/10 text-foreground",
      Self::Info => "border-info/50 bg-info/10 text-foreground",
    }
  }
}

pub const ALERT_BASE_CLASS: &str = "relative w-full rounded-md border p-4";
pub const ALERT_TITLE_BASE_CLASS: &str = "mb-1 font-medium leading-none tracking-normal";
pub const ALERT_DESCRIPTION_BASE_CLASS: &str = "text-sm";

pub fn alert_class(variant: AlertVariant, class: &str) -> String {
  merge_classes(classes([Some(ALERT_BASE_CLASS), Some(variant.class())]), class)
}

pub fn alert_title_class(class: &str) -> String {
  merge_classes(classes([Some(ALERT_TITLE_BASE_CLASS)]), class)
}

pub fn alert_description_class(variant: AlertVariant, class: &str) -> String {
  let variant_class = match variant {
    AlertVariant::Default => "text-muted-foreground",
    AlertVariant::Destructive => "text-destructive",
    // Muted text is checked only on plain surfaces, not on the status tints.
    AlertVariant::Success | AlertVariant::Warning | AlertVariant::Info => "text-foreground",
  };

  merge_classes(classes([Some(ALERT_DESCRIPTION_BASE_CLASS), Some(variant_class)]), class)
}

#[component]
pub fn Alert(
  #[props(default)] variant: AlertVariant,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = alert_class(variant, &class);

  rsx! {
    div {
      role: "alert",
      class,
      {children}
    }
  }
}

#[component]
pub fn AlertTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = alert_title_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn AlertDescription(
  #[props(default)] variant: AlertVariant,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = alert_description_class(variant, &class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}
