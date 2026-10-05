use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BadgeVariant {
  Default,
  Secondary,
  Destructive,
  Outline,
}

impl BadgeVariant {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => "border-transparent bg-primary text-primary-foreground",
      Self::Secondary => "border-transparent bg-secondary text-secondary-foreground",
      Self::Destructive => "border-transparent bg-destructive text-destructive-foreground",
      Self::Outline => "border-border text-foreground",
    }
  }
}

pub const BADGE_BASE_CLASS: &str = "inline-flex items-center rounded-md border px-2 py-0.5 text-xs font-semibold transition-colors";

pub fn badge_class(variant: BadgeVariant, class: &str) -> String {
  classes([Some(BADGE_BASE_CLASS), Some(variant.class()), Some(class)])
}

#[component]
pub fn Badge(
  #[props(default = BadgeVariant::Default)] variant: BadgeVariant,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = badge_class(variant, &class);

  rsx! {
    span {
      class,
      {children}
    }
  }
}
