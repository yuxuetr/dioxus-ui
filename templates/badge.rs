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
      Self::Default => "border-transparent bg-blue-600 text-white",
      Self::Secondary => "border-transparent bg-zinc-100 text-zinc-900",
      Self::Destructive => "border-transparent bg-red-600 text-white",
      Self::Outline => "border-zinc-200 text-zinc-900",
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
