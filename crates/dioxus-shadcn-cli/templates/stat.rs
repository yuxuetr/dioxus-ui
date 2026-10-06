use super::utils::classes;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StatGroupOrientation {
  #[default]
  Horizontal,
  Vertical,
}

impl StatGroupOrientation {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Horizontal => "grid-flow-col divide-x overflow-x-auto",
      Self::Vertical => "grid-flow-row divide-y",
    }
  }
}

pub const STAT_GROUP_BASE_CLASS: &str =
  "inline-grid divide-border rounded-lg border border-border bg-card text-card-foreground";
// The figure takes a second column spanning the rows, so the title can come
// first in source order, as the `dl` needs.
pub const STAT_BASE_CLASS: &str = "grid grid-cols-[1fr_auto] content-start gap-x-4 px-6 py-4";
pub const STAT_TITLE_BASE_CLASS: &str =
  "col-start-1 text-sm whitespace-nowrap text-muted-foreground";
pub const STAT_VALUE_BASE_CLASS: &str =
  "col-start-1 text-3xl font-bold tracking-tight whitespace-nowrap";
pub const STAT_DESCRIPTION_BASE_CLASS: &str =
  "col-start-1 text-xs whitespace-nowrap text-muted-foreground";
pub const STAT_FIGURE_BASE_CLASS: &str = "col-start-2 row-span-3 row-start-1 self-center";

pub fn stat_group_class(orientation: StatGroupOrientation, class: &str) -> String {
  classes([Some(STAT_GROUP_BASE_CLASS), Some(orientation.class()), Some(class)])
}

pub fn stat_class(class: &str) -> String {
  classes([Some(STAT_BASE_CLASS), Some(class)])
}

pub fn stat_title_class(class: &str) -> String {
  classes([Some(STAT_TITLE_BASE_CLASS), Some(class)])
}

pub fn stat_value_class(class: &str) -> String {
  classes([Some(STAT_VALUE_BASE_CLASS), Some(class)])
}

pub fn stat_description_class(class: &str) -> String {
  classes([Some(STAT_DESCRIPTION_BASE_CLASS), Some(class)])
}

pub fn stat_figure_class(class: &str) -> String {
  classes([Some(STAT_FIGURE_BASE_CLASS), Some(class)])
}

/// A definition list of stats; each `Stat` inside pairs a title with its
/// value for assistive technology.
#[component]
pub fn StatGroup(
  #[props(default)] orientation: StatGroupOrientation,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = stat_group_class(orientation, &class);

  rsx! {
    dl { class, ..attributes, {children} }
  }
}

/// One stat; place it inside a `StatGroup`, title first.
#[component]
pub fn Stat(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = stat_class(&class);

  rsx! {
    div { class, ..attributes, {children} }
  }
}

#[component]
pub fn StatTitle(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = stat_title_class(&class);

  rsx! {
    dt { class, ..attributes, {children} }
  }
}

#[component]
pub fn StatValue(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = stat_value_class(&class);

  rsx! {
    dd { class, ..attributes, {children} }
  }
}

#[component]
pub fn StatDescription(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = stat_description_class(&class);

  rsx! {
    dd { class, ..attributes, {children} }
  }
}

/// An icon or image beside the stat; mark a purely decorative icon
/// `aria-hidden`.
#[component]
pub fn StatFigure(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = stat_figure_class(&class);

  rsx! {
    dd { class, ..attributes, {children} }
  }
}
