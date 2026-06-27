use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const EMPTY_BASE_CLASS: &str = "flex min-h-40 flex-col items-center justify-center gap-6 rounded-md border border-dashed border-zinc-200 p-8 text-center";
pub const EMPTY_HEADER_BASE_CLASS: &str = "flex flex-col items-center gap-2";
pub const EMPTY_TITLE_BASE_CLASS: &str = "text-lg font-semibold text-zinc-950";
pub const EMPTY_DESCRIPTION_BASE_CLASS: &str = "max-w-sm text-sm text-zinc-600";
pub const EMPTY_CONTENT_BASE_CLASS: &str = "text-sm text-zinc-600";
pub const EMPTY_ACTIONS_BASE_CLASS: &str = "flex flex-wrap items-center justify-center gap-2";

pub fn empty_class(class: &str) -> String {
  classes([Some(EMPTY_BASE_CLASS), Some(class)])
}

pub fn empty_header_class(class: &str) -> String {
  classes([Some(EMPTY_HEADER_BASE_CLASS), Some(class)])
}

pub fn empty_title_class(class: &str) -> String {
  classes([Some(EMPTY_TITLE_BASE_CLASS), Some(class)])
}

pub fn empty_description_class(class: &str) -> String {
  classes([Some(EMPTY_DESCRIPTION_BASE_CLASS), Some(class)])
}

pub fn empty_content_class(class: &str) -> String {
  classes([Some(EMPTY_CONTENT_BASE_CLASS), Some(class)])
}

pub fn empty_actions_class(class: &str) -> String {
  classes([Some(EMPTY_ACTIONS_BASE_CLASS), Some(class)])
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

    assert!(actual.contains(EMPTY_BASE_CLASS));
    assert!(actual.ends_with("min-h-64"));
  }

  #[test]
  fn empty_actions_class_appends_user_class() {
    let actual = empty_actions_class("justify-start");

    assert!(actual.contains(EMPTY_ACTIONS_BASE_CLASS));
    assert!(actual.ends_with("justify-start"));
  }
}
