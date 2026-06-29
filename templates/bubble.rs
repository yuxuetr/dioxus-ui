use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BubbleVariant {
  Default,
  Secondary,
  Muted,
  Tinted,
  Outline,
  Ghost,
  Destructive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BubbleAlign {
  Start,
  End,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BubbleReactionSide {
  Bottom,
  Top,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BubbleReactionAlign {
  Start,
  Center,
  End,
}

pub const BUBBLE_BASE_CLASS: &str = "group flex max-w-full flex-col gap-1";
pub const BUBBLE_ALIGN_START_CLASS: &str = "items-start";
pub const BUBBLE_ALIGN_END_CLASS: &str = "items-end";
pub const BUBBLE_GROUP_BASE_CLASS: &str = "flex flex-col gap-2";
pub const BUBBLE_CONTENT_BASE_CLASS: &str = "min-w-0 rounded-md px-3 py-2 text-sm leading-6";
pub const BUBBLE_DEFAULT_CLASS: &str = "bg-zinc-950 text-white";
pub const BUBBLE_SECONDARY_CLASS: &str = "bg-zinc-100 text-zinc-950";
pub const BUBBLE_MUTED_CLASS: &str = "bg-zinc-50 text-zinc-700";
pub const BUBBLE_TINTED_CLASS: &str = "bg-blue-50 text-blue-950";
pub const BUBBLE_OUTLINE_CLASS: &str = "border border-zinc-200 bg-white text-zinc-950";
pub const BUBBLE_GHOST_CLASS: &str = "bg-transparent text-zinc-950";
pub const BUBBLE_DESTRUCTIVE_CLASS: &str = "bg-red-50 text-red-950 border border-red-200";
pub const BUBBLE_REACTIONS_BASE_CLASS: &str = "flex items-center gap-1 text-xs text-zinc-500";
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
  classes([Some(BUBBLE_BASE_CLASS), Some(align.class()), Some(class)])
}

pub fn bubble_group_class(class: &str) -> String {
  classes([Some(BUBBLE_GROUP_BASE_CLASS), Some(class)])
}

pub fn bubble_content_class(variant: BubbleVariant, class: &str) -> String {
  classes([
    Some(BUBBLE_CONTENT_BASE_CLASS),
    Some(variant.class()),
    Some(class),
  ])
}

pub fn bubble_reactions_class(
  side: BubbleReactionSide,
  align: BubbleReactionAlign,
  class: &str,
) -> String {
  classes([
    Some(BUBBLE_REACTIONS_BASE_CLASS),
    Some(side.class()),
    Some(align.class()),
    Some(class),
  ])
}

#[component]
pub fn Bubble(
  #[props(default = BubbleVariant::Default)] variant: BubbleVariant,
  #[props(default = BubbleAlign::Start)] align: BubbleAlign,
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
  #[props(default = BubbleVariant::Default)] variant: BubbleVariant,
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
  #[props(default = BubbleReactionSide::Bottom)] side: BubbleReactionSide,
  #[props(default = BubbleReactionAlign::Center)] align: BubbleReactionAlign,
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
