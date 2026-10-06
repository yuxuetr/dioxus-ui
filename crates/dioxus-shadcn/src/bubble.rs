use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BubbleVariant {
  #[default]
  Default,
  Secondary,
  Muted,
  Tinted,
  Outline,
  Ghost,
  Destructive,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BubbleAlign {
  #[default]
  Start,
  End,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BubbleReactionSide {
  #[default]
  Bottom,
  Top,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BubbleReactionAlign {
  Start,
  #[default]
  Center,
  End,
}

pub const BUBBLE_BASE_CLASS: &str = "group flex max-w-full flex-col gap-1";
pub const BUBBLE_ALIGN_START_CLASS: &str = "items-start";
pub const BUBBLE_ALIGN_END_CLASS: &str = "items-end";
pub const BUBBLE_GROUP_BASE_CLASS: &str = "flex flex-col gap-2";
pub const BUBBLE_CONTENT_BASE_CLASS: &str = "min-w-0 rounded-md px-3 py-2 text-sm leading-6";
pub const BUBBLE_DEFAULT_CLASS: &str = "bg-primary text-primary-foreground";
pub const BUBBLE_SECONDARY_CLASS: &str = "bg-secondary text-secondary-foreground";
pub const BUBBLE_MUTED_CLASS: &str = "bg-muted text-muted-foreground";
pub const BUBBLE_TINTED_CLASS: &str = "bg-info/10 text-foreground";
pub const BUBBLE_OUTLINE_CLASS: &str = "border border-border bg-background text-foreground";
pub const BUBBLE_GHOST_CLASS: &str = "bg-transparent text-foreground";
pub const BUBBLE_DESTRUCTIVE_CLASS: &str =
  "bg-destructive/10 text-foreground border border-destructive/30";
pub const BUBBLE_REACTIONS_BASE_CLASS: &str =
  "flex items-center gap-1 text-xs text-muted-foreground";
pub const BUBBLE_REACTIONS_TOP_CLASS: &str = "order-first mb-1";
pub const BUBBLE_REACTIONS_BOTTOM_CLASS: &str = "order-last mt-1";
pub const BUBBLE_REACTIONS_ALIGN_START_CLASS: &str = "self-start";
pub const BUBBLE_REACTIONS_ALIGN_CENTER_CLASS: &str = "self-center";
pub const BUBBLE_REACTIONS_ALIGN_END_CLASS: &str = "self-end";

impl BubbleVariant {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => BUBBLE_DEFAULT_CLASS,
      Self::Secondary => BUBBLE_SECONDARY_CLASS,
      Self::Muted => BUBBLE_MUTED_CLASS,
      Self::Tinted => BUBBLE_TINTED_CLASS,
      Self::Outline => BUBBLE_OUTLINE_CLASS,
      Self::Ghost => BUBBLE_GHOST_CLASS,
      Self::Destructive => BUBBLE_DESTRUCTIVE_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Secondary => "secondary",
      Self::Muted => "muted",
      Self::Tinted => "tinted",
      Self::Outline => "outline",
      Self::Ghost => "ghost",
      Self::Destructive => "destructive",
    }
  }
}

impl BubbleAlign {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Start => BUBBLE_ALIGN_START_CLASS,
      Self::End => BUBBLE_ALIGN_END_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Start => "start",
      Self::End => "end",
    }
  }
}

impl BubbleReactionSide {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Top => BUBBLE_REACTIONS_TOP_CLASS,
      Self::Bottom => BUBBLE_REACTIONS_BOTTOM_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Top => "top",
      Self::Bottom => "bottom",
    }
  }
}

impl BubbleReactionAlign {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Start => BUBBLE_REACTIONS_ALIGN_START_CLASS,
      Self::Center => BUBBLE_REACTIONS_ALIGN_CENTER_CLASS,
      Self::End => BUBBLE_REACTIONS_ALIGN_END_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Start => "start",
      Self::Center => "center",
      Self::End => "end",
    }
  }
}

pub fn bubble_class(align: BubbleAlign, class: &str) -> String {
  merge_classes(classes([Some(BUBBLE_BASE_CLASS), Some(align.class())]), class)
}

pub fn bubble_group_class(class: &str) -> String {
  merge_classes(classes([Some(BUBBLE_GROUP_BASE_CLASS)]), class)
}

pub fn bubble_content_class(variant: BubbleVariant, class: &str) -> String {
  merge_classes(classes([Some(BUBBLE_CONTENT_BASE_CLASS), Some(variant.class())]), class)
}

pub fn bubble_reactions_class(
  side: BubbleReactionSide,
  align: BubbleReactionAlign,
  class: &str,
) -> String {
  merge_classes(
    classes([Some(BUBBLE_REACTIONS_BASE_CLASS), Some(side.class()), Some(align.class())]),
    class,
  )
}

#[component]
pub fn Bubble(
  #[props(default)] variant: BubbleVariant,
  #[props(default)] align: BubbleAlign,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = bubble_class(align, &class);

  rsx! {
    div {
      class,
      "data-align": align.attribute(),
      "data-variant": variant.attribute(),
      {children}
    }
  }
}

#[component]
pub fn BubbleGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = bubble_group_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn BubbleContent(
  #[props(default)] variant: BubbleVariant,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = bubble_content_class(variant, &class);

  rsx! {
    div {
      class,
      "data-variant": variant.attribute(),
      {children}
    }
  }
}

#[component]
pub fn BubbleReactions(
  #[props(default)] side: BubbleReactionSide,
  #[props(default)] align: BubbleReactionAlign,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = bubble_reactions_class(side, align, &class);

  rsx! {
    div {
      class,
      "data-side": side.attribute(),
      "data-align": align.attribute(),
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bubble_class_reflects_alignment() {
    let actual = bubble_class(BubbleAlign::End, "max-w-sm");

    assert_eq!(actual, "group flex flex-col gap-1 items-end max-w-sm");
    assert!(actual.contains(BUBBLE_ALIGN_END_CLASS));
    assert!(actual.ends_with("max-w-sm"));
  }

  #[test]
  fn bubble_content_class_reflects_variant() {
    let actual = bubble_content_class(BubbleVariant::Destructive, "rounded-lg");

    assert_eq!(
      actual,
      "min-w-0 px-3 py-2 text-sm leading-6 bg-destructive/10 text-foreground border border-destructive/30 rounded-lg"
    );
    assert!(actual.contains(BUBBLE_DESTRUCTIVE_CLASS));
    assert!(actual.ends_with("rounded-lg"));
  }

  #[test]
  fn bubble_reactions_class_reflects_side_and_align() {
    let actual =
      bubble_reactions_class(BubbleReactionSide::Top, BubbleReactionAlign::End, "opacity-80");

    assert!(actual.contains(BUBBLE_REACTIONS_BASE_CLASS));
    assert!(actual.contains(BUBBLE_REACTIONS_TOP_CLASS));
    assert!(actual.contains(BUBBLE_REACTIONS_ALIGN_END_CLASS));
    assert!(actual.ends_with("opacity-80"));
  }

  #[test]
  fn maps_bubble_attributes() {
    assert_eq!(BubbleVariant::Tinted.attribute(), "tinted");
    assert_eq!(BubbleAlign::Start.attribute(), "start");
    assert_eq!(BubbleReactionSide::Bottom.attribute(), "bottom");
    assert_eq!(BubbleReactionAlign::Center.attribute(), "center");
  }
}
