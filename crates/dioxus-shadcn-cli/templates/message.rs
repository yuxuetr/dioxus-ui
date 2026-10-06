use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MessageAlign {
  #[default]
  Start,
  End,
}

pub const MESSAGE_BASE_CLASS: &str = "flex w-full min-w-0 gap-3 text-sm";
pub const MESSAGE_ALIGN_START_CLASS: &str = "items-start justify-start";
pub const MESSAGE_ALIGN_END_CLASS: &str = "items-start justify-end flex-row-reverse";
pub const MESSAGE_GROUP_BASE_CLASS: &str = "flex flex-col gap-4";
pub const MESSAGE_AVATAR_BASE_CLASS: &str = "flex h-8 w-8 shrink-0 items-center justify-center overflow-hidden rounded-full bg-muted text-xs font-medium text-foreground";
pub const MESSAGE_CONTENT_BASE_CLASS: &str = "grid min-w-0 max-w-full flex-1 gap-1";
pub const MESSAGE_CONTENT_ALIGN_START_CLASS: &str = "justify-items-start";
pub const MESSAGE_CONTENT_ALIGN_END_CLASS: &str = "justify-items-end";
pub const MESSAGE_HEADER_BASE_CLASS: &str =
  "flex min-w-0 items-center gap-2 text-xs text-muted-foreground";
pub const MESSAGE_FOOTER_BASE_CLASS: &str =
  "flex min-w-0 items-center gap-2 text-xs text-muted-foreground";

impl MessageAlign {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Start => MESSAGE_ALIGN_START_CLASS,
      Self::End => MESSAGE_ALIGN_END_CLASS,
    }
  }

  pub const fn content_class(self) -> &'static str {
    match self {
      Self::Start => MESSAGE_CONTENT_ALIGN_START_CLASS,
      Self::End => MESSAGE_CONTENT_ALIGN_END_CLASS,
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Start => "start",
      Self::End => "end",
    }
  }
}

pub fn message_class(align: MessageAlign, class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_BASE_CLASS), Some(align.class())]), class)
}

pub fn message_group_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_GROUP_BASE_CLASS)]), class)
}

pub fn message_avatar_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_AVATAR_BASE_CLASS)]), class)
}

pub fn message_content_class(align: MessageAlign, class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_CONTENT_BASE_CLASS), Some(align.content_class())]), class)
}

pub fn message_header_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_HEADER_BASE_CLASS)]), class)
}

pub fn message_footer_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_FOOTER_BASE_CLASS)]), class)
}

#[component]
pub fn Message(
  #[props(default)] align: MessageAlign,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = message_class(align, &class);

  rsx! {
    div {
      class,
      "data-align": align.attribute(),
      {children}
    }
  }
}

#[component]
pub fn MessageGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = message_group_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn MessageAvatar(#[props(default)] class: String, children: Element) -> Element {
  let class = message_avatar_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn MessageContent(
  #[props(default)] align: MessageAlign,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = message_content_class(align, &class);

  rsx! {
    div {
      class,
      "data-align": align.attribute(),
      {children}
    }
  }
}

#[component]
pub fn MessageHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = message_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn MessageFooter(#[props(default)] class: String, children: Element) -> Element {
  let class = message_footer_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}
