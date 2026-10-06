use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

pub const EMPTY_BASE_CLASS: &str = "flex min-h-40 flex-col items-center justify-center gap-6 rounded-md border border-dashed border-border p-8 text-center";
pub const EMPTY_HEADER_BASE_CLASS: &str = "flex flex-col items-center gap-2";
pub const EMPTY_TITLE_BASE_CLASS: &str = "text-lg font-semibold text-foreground";
pub const EMPTY_DESCRIPTION_BASE_CLASS: &str = "max-w-sm text-sm text-muted-foreground";
pub const EMPTY_CONTENT_BASE_CLASS: &str = "text-sm text-muted-foreground";
pub const EMPTY_ACTIONS_BASE_CLASS: &str = "flex flex-wrap items-center justify-center gap-2";

pub fn empty_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_BASE_CLASS)]), class)
}

pub fn empty_header_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_HEADER_BASE_CLASS)]), class)
}

pub fn empty_title_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_TITLE_BASE_CLASS)]), class)
}

pub fn empty_description_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_DESCRIPTION_BASE_CLASS)]), class)
}

pub fn empty_content_class(class: &str) -> String {
  merge_classes(classes([Some(EMPTY_CONTENT_BASE_CLASS)]), class)
}

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
