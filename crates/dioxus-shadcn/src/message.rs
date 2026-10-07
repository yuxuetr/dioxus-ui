//! Message: chat row layout for an avatar, header metadata, content, and footer
//! actions, independent of any chat provider.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// Which side of the conversation a message sits on.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MessageAlign {
  /// At the start of the row, avatar first, as for the other party.
  #[default]
  Start,
  /// At the end of the row, avatar last, as for the user's own messages.
  End,
}

const MESSAGE_BASE_CLASS: &str = "flex w-full min-w-0 gap-3 text-sm";
const MESSAGE_ALIGN_START_CLASS: &str = "items-start justify-start";
const MESSAGE_ALIGN_END_CLASS: &str = "items-start justify-end flex-row-reverse";
const MESSAGE_GROUP_BASE_CLASS: &str = "flex flex-col gap-4";
const MESSAGE_AVATAR_BASE_CLASS: &str = "flex h-8 w-8 shrink-0 items-center justify-center overflow-hidden rounded-full bg-muted text-xs font-medium text-foreground";
const MESSAGE_CONTENT_BASE_CLASS: &str = "grid min-w-0 max-w-full flex-1 gap-1";
const MESSAGE_CONTENT_ALIGN_START_CLASS: &str = "justify-items-start";
const MESSAGE_CONTENT_ALIGN_END_CLASS: &str = "justify-items-end";
const MESSAGE_HEADER_BASE_CLASS: &str =
  "flex min-w-0 items-center gap-2 text-xs text-muted-foreground";
const MESSAGE_FOOTER_BASE_CLASS: &str =
  "flex min-w-0 items-center gap-2 text-xs text-muted-foreground";

impl MessageAlign {
  /// The row's justification and order for this side.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Start => MESSAGE_ALIGN_START_CLASS,
      Self::End => MESSAGE_ALIGN_END_CLASS,
    }
  }

  /// The content column's item alignment for this side.
  pub const fn content_class(self) -> &'static str {
    match self {
      Self::Start => MESSAGE_CONTENT_ALIGN_START_CLASS,
      Self::End => MESSAGE_CONTENT_ALIGN_END_CLASS,
    }
  }

  /// The `data-align` value: `start` or `end`.
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Start => "start",
      Self::End => "end",
    }
  }
}

/// Classes for the row: base classes, the alignment's, then `class` merged over them.
pub fn message_class(align: MessageAlign, class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_BASE_CLASS), Some(align.class())]), class)
}

/// Classes for a group of messages stacked with a gap, then `class` merged over them.
pub fn message_group_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_GROUP_BASE_CLASS)]), class)
}

/// Classes for the avatar: a small muted circle, then `class` merged over it.
pub fn message_avatar_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_AVATAR_BASE_CLASS)]), class)
}

/// Classes for the content column: base classes, the alignment's, then `class`
/// merged over them.
pub fn message_content_class(align: MessageAlign, class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_CONTENT_BASE_CLASS), Some(align.content_class())]), class)
}

/// Classes for the header: a row of small muted metadata, then `class` merged over it.
pub fn message_header_class(class: &str) -> String {
  merge_classes(classes([Some(MESSAGE_HEADER_BASE_CLASS)]), class)
}

/// Classes for the footer: a row of small muted actions or metadata, then `class`
/// merged over it.
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn message_class_reflects_alignment() {
    let actual = message_class(MessageAlign::End, "max-w-xl");

    assert!(actual.contains(MESSAGE_BASE_CLASS));
    assert!(actual.contains(MESSAGE_ALIGN_END_CLASS));
    assert!(actual.ends_with("max-w-xl"));
  }

  #[test]
  fn message_content_class_reflects_alignment() {
    let actual = message_content_class(MessageAlign::Start, "gap-2");

    assert_eq!(actual, "grid min-w-0 max-w-full flex-1 justify-items-start gap-2");
    assert!(actual.contains(MESSAGE_CONTENT_ALIGN_START_CLASS));
    assert!(actual.ends_with("gap-2"));
  }

  #[test]
  fn message_part_classes_append_user_classes() {
    assert!(message_group_class("space-y-1").ends_with("space-y-1"));
    assert!(message_avatar_class("bg-blue-100").ends_with("bg-blue-100"));
    assert!(message_header_class("font-medium").ends_with("font-medium"));
    assert!(message_footer_class("justify-end").ends_with("justify-end"));
  }

  #[test]
  fn maps_message_align_attribute() {
    assert_eq!(MessageAlign::Start.attribute(), "start");
    assert_eq!(MessageAlign::End.attribute(), "end");
  }
}
