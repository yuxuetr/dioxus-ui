use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

pub const CARD_BASE_CLASS: &str =
  "rounded-md border border-border bg-card text-card-foreground shadow-sm";
pub const CARD_HEADER_BASE_CLASS: &str = "flex flex-col gap-1.5 p-6";
pub const CARD_TITLE_BASE_CLASS: &str = "text-2xl font-semibold leading-none tracking-normal";
pub const CARD_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
pub const CARD_CONTENT_BASE_CLASS: &str = "p-6 pt-0";
pub const CARD_FOOTER_BASE_CLASS: &str = "flex items-center p-6 pt-0";

pub fn card_class(class: &str) -> String {
  merge_classes(classes([Some(CARD_BASE_CLASS)]), class)
}

pub fn card_header_class(class: &str) -> String {
  merge_classes(classes([Some(CARD_HEADER_BASE_CLASS)]), class)
}

pub fn card_title_class(class: &str) -> String {
  merge_classes(classes([Some(CARD_TITLE_BASE_CLASS)]), class)
}

pub fn card_description_class(class: &str) -> String {
  merge_classes(classes([Some(CARD_DESCRIPTION_BASE_CLASS)]), class)
}

pub fn card_content_class(class: &str) -> String {
  merge_classes(classes([Some(CARD_CONTENT_BASE_CLASS)]), class)
}

pub fn card_footer_class(class: &str) -> String {
  merge_classes(classes([Some(CARD_FOOTER_BASE_CLASS)]), class)
}

#[component]
pub fn Card(#[props(default)] class: String, children: Element) -> Element {
  let class = card_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CardHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = card_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// A `div`, as in shadcn/ui v4; wrap the text in a heading at the level the
/// page outline needs.
#[component]
pub fn CardTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = card_title_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CardDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = card_description_class(&class);

  rsx! {
    p {
      class,
      {children}
    }
  }
}

#[component]
pub fn CardContent(#[props(default)] class: String, children: Element) -> Element {
  let class = card_content_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CardFooter(#[props(default)] class: String, children: Element) -> Element {
  let class = card_footer_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}
