//! Mockup: frames content as a browser window, an application window, a terminal, or
//! a phone, for product and documentation pages.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

const MOCKUP_FRAME_BASE_CLASS: &str =
  "overflow-hidden rounded-lg border border-border bg-muted shadow-sm";
const MOCKUP_TOOLBAR_BASE_CLASS: &str = "flex items-center gap-3 px-3 py-2";
const MOCKUP_DOT_CLASS: &str = "size-3 rounded-full bg-muted-foreground/40";
const MOCKUP_ADDRESS_BASE_CLASS: &str = "mx-auto w-1/2 min-w-0 truncate rounded-md bg-background px-3 py-1 text-center text-xs text-muted-foreground";
const MOCKUP_CONTENT_BASE_CLASS: &str = "border-t border-border bg-background";
const MOCKUP_CODE_BASE_CLASS: &str =
  "overflow-x-auto rounded-lg bg-foreground py-4 font-mono text-sm text-background shadow-sm";
const MOCKUP_CODE_LINE_BASE_CLASS: &str = "flex gap-4 px-5 leading-6";
const MOCKUP_CODE_LINE_HIGHLIGHT_CLASS: &str = "bg-warning text-warning-foreground";
const MOCKUP_CODE_PREFIX_CLASS: &str = "w-4 shrink-0 select-none text-end opacity-60";
const MOCKUP_PHONE_BASE_CLASS: &str =
  "relative mx-auto w-72 rounded-[2.5rem] border-[10px] border-foreground bg-foreground shadow-lg";
const MOCKUP_PHONE_NOTCH_CLASS: &str =
  "absolute left-1/2 top-0 z-10 h-6 w-28 -translate-x-1/2 rounded-b-xl bg-foreground";
const MOCKUP_PHONE_DISPLAY_BASE_CLASS: &str =
  "aspect-[9/19.5] overflow-hidden rounded-[1.75rem] bg-background";

/// Classes for a window frame: rounded border on a muted background, then `class`
/// merged over them.
pub fn mockup_frame_class(class: &str) -> String {
  merge_classes(classes([Some(MOCKUP_FRAME_BASE_CLASS)]), class)
}

fn mockup_code_class(class: &str) -> String {
  merge_classes(classes([Some(MOCKUP_CODE_BASE_CLASS)]), class)
}

/// Classes for a terminal line: base classes, the warning colors when `highlight`,
/// then `class` merged over them.
pub fn mockup_code_line_class(highlight: bool, class: &str) -> String {
  merge_classes(classes([Some(MOCKUP_CODE_LINE_BASE_CLASS), highlight.then_some(MOCKUP_CODE_LINE_HIGHLIGHT_CLASS)]), class)
}

fn mockup_phone_class(class: &str) -> String {
  merge_classes(classes([Some(MOCKUP_PHONE_BASE_CLASS)]), class)
}

/// The three window dots, decorative.
#[component]
fn MockupDots() -> Element {
  rsx! {
    div { class: "flex gap-1.5", "aria-hidden": "true",
      span { class: MOCKUP_DOT_CLASS }
      span { class: MOCKUP_DOT_CLASS }
      span { class: MOCKUP_DOT_CLASS }
    }
  }
}

/// Frames content as a browser window with an address field showing `url`.
#[component]
pub fn MockupBrowser(
  #[props(default)] url: String,
  #[props(default)] class: String,
  #[props(default)] content_class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = mockup_frame_class(&class);
  let content_class = merge_classes(classes([Some(MOCKUP_CONTENT_BASE_CLASS)]), content_class.as_str());

  rsx! {
    div {
      class,
      ..attributes,
      div { class: MOCKUP_TOOLBAR_BASE_CLASS,
        MockupDots {}
        if !url.is_empty() {
          div { class: MOCKUP_ADDRESS_BASE_CLASS, "{url}" }
        }
      }
      div { class: content_class, {children} }
    }
  }
}

/// Frames content as an application window.
#[component]
pub fn MockupWindow(
  #[props(default)] class: String,
  #[props(default)] content_class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = mockup_frame_class(&class);
  let content_class = merge_classes(classes([Some(MOCKUP_CONTENT_BASE_CLASS)]), content_class.as_str());

  rsx! {
    div {
      class,
      ..attributes,
      div { class: MOCKUP_TOOLBAR_BASE_CLASS, MockupDots {} }
      div { class: content_class, {children} }
    }
  }
}

/// A terminal-styled block of `MockupCodeLine`s.
#[component]
pub fn MockupCode(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = mockup_code_class(&class);

  rsx! {
    div {
      class,
      ..attributes,
      {children}
    }
  }
}

/// One line of a `MockupCode`. `prefix`, such as `$` or `>`, is shown before
/// the line but not copied or read; `highlight` marks the line.
#[component]
pub fn MockupCodeLine(
  #[props(default)] prefix: String,
  #[props(default)] highlight: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = mockup_code_line_class(highlight, &class);

  rsx! {
    pre {
      class,
      "data-highlight": highlight.to_string(),
      span { class: MOCKUP_CODE_PREFIX_CLASS, "aria-hidden": "true", "{prefix}" }
      code { {children} }
    }
  }
}

/// Frames content as a phone, with a camera notch and a screen-shaped
/// display area.
#[component]
pub fn MockupPhone(
  #[props(default)] class: String,
  #[props(default)] display_class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = mockup_phone_class(&class);
  let display_class =
    merge_classes(classes([Some(MOCKUP_PHONE_DISPLAY_BASE_CLASS)]), display_class.as_str());

  rsx! {
    div {
      class,
      ..attributes,
      div { class: MOCKUP_PHONE_NOTCH_CLASS, "aria-hidden": "true" }
      div { class: display_class, {children} }
    }
  }
}
