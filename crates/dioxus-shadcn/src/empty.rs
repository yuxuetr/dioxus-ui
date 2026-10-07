//! Empty: the layout for an empty state, such as zero results, a first run, or a filter that
//! matches nothing. Actions, icons, and loading stay app-owned.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

const EMPTY_BASE_CLASS: &str = "flex min-h-40 flex-col items-center justify-center gap-6 rounded-md border border-dashed border-border p-8 text-center";
const EMPTY_HEADER_BASE_CLASS: &str = "flex flex-col items-center gap-2";
const EMPTY_TITLE_BASE_CLASS: &str = "text-lg font-semibold text-foreground";
const EMPTY_DESCRIPTION_BASE_CLASS: &str = "max-w-sm text-sm text-muted-foreground";
const EMPTY_CONTENT_BASE_CLASS: &str = "text-sm text-muted-foreground";
const EMPTY_ACTIONS_BASE_CLASS: &str = "flex flex-wrap items-center justify-center gap-2";

/// Classes for the dashed, centered empty-state box, with `class` merged over them.
pub fn empty_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_BASE_CLASS)]), class)
}

/// Classes for the header that stacks the media, title, and description.
pub fn empty_header_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_HEADER_BASE_CLASS)]), class)
}

/// Classes for the title.
pub fn empty_title_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_TITLE_BASE_CLASS)]), class)
}

/// Classes for the description, kept to a readable width.
pub fn empty_description_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_DESCRIPTION_BASE_CLASS)]), class)
}

/// Classes for the extra content below the header.
pub fn empty_content_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_CONTENT_BASE_CLASS)]), class)
}

/// Classes for the row of actions, which wraps and stays centered.
pub fn empty_actions_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_ACTIONS_BASE_CLASS)]), class)
}

#[component]
pub fn Empty(#[props(default)] class: String, children: Element) -> Element {
  let class = empty_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn EmptyHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = empty_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn EmptyTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = empty_title_class(&class);

  rsx! {
    h3 {
      class,
      {children}
    }
  }
}

#[component]
pub fn EmptyDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = empty_description_class(&class);

  rsx! {
    p {
      class,
      {children}
    }
  }
}

#[component]
pub fn EmptyContent(#[props(default)] class: String, children: Element) -> Element {
  let class = empty_content_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn EmptyActions(#[props(default)] class: String, children: Element) -> Element {
  let class = empty_actions_class(&class);

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
  fn empty_class_appends_user_class() {
    let actual = empty_class("min-h-64");

    assert_eq!(
      actual,
      "flex flex-col items-center justify-center gap-6 rounded-md border border-dashed border-border p-8 text-center min-h-64"
    );
    assert!(actual.ends_with("min-h-64"));
  }

  #[test]
  fn empty_actions_class_appends_user_class() {
    let actual = empty_actions_class("justify-start");

    assert_eq!(actual, "flex flex-wrap items-center gap-2 justify-start");
    assert!(actual.ends_with("justify-start"));
  }
}
