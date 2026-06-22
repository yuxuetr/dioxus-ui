use dioxus::prelude::*;
use dioxus_ui_core::classes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AlertVariant {
  #[default]
  Default,
  Destructive,
}

impl AlertVariant {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => "border-zinc-200 text-zinc-950",
      Self::Destructive => "border-red-500 text-red-900",
    }
  }
}

pub const ALERT_BASE_CLASS: &str = "relative w-full rounded-md border bg-white p-4";
pub const ALERT_TITLE_BASE_CLASS: &str = "mb-1 font-medium leading-none tracking-normal";
pub const ALERT_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";

pub fn alert_class(variant: AlertVariant, class: &str) -> String {
  classes([Some(ALERT_BASE_CLASS), Some(variant.class()), Some(class)])
}

pub fn alert_title_class(class: &str) -> String {
  classes([Some(ALERT_TITLE_BASE_CLASS), Some(class)])
}

pub fn alert_description_class(variant: AlertVariant, class: &str) -> String {
  let variant_class = match variant {
    AlertVariant::Default => "text-zinc-600",
    AlertVariant::Destructive => "text-red-800",
  };

  classes([Some(ALERT_DESCRIPTION_BASE_CLASS), Some(variant_class), Some(class)])
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
    h5 {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn alert_class_includes_variant_and_user_class() {
    let actual = alert_class(AlertVariant::Destructive, "mt-4");

    assert!(actual.contains(ALERT_BASE_CLASS));
    assert!(actual.contains("border-red-500 text-red-900"));
    assert!(actual.ends_with("mt-4"));
  }

  #[test]
  fn alert_description_class_reflects_variant() {
    let actual = alert_description_class(AlertVariant::Destructive, "leading-6");

    assert!(actual.contains(ALERT_DESCRIPTION_BASE_CLASS));
    assert!(actual.contains("text-red-800"));
    assert!(actual.ends_with("leading-6"));
  }
}
