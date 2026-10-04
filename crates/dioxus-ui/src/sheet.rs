use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};

use crate::overlay_behavior::use_modal_focus_scope;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SheetSide {
  Top,
  #[default]
  Right,
  Bottom,
  Left,
}

impl SheetSide {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Top => "inset-x-0 top-0 h-auto border-b",
      Self::Right => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm",
      Self::Bottom => "inset-x-0 bottom-0 h-auto border-t",
      Self::Left => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm",
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Top => "top",
      Self::Right => "right",
      Self::Bottom => "bottom",
      Self::Left => "left",
    }
  }
}

pub const SHEET_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
pub const SHEET_CONTENT_BASE_CLASS: &str = "fixed z-50 gap-4 border-zinc-200 bg-white p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";
pub const SHEET_HEADER_BASE_CLASS: &str = "flex flex-col gap-2 text-left";
pub const SHEET_FOOTER_BASE_CLASS: &str = "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end";
pub const SHEET_TITLE_BASE_CLASS: &str = "text-lg font-semibold leading-none text-zinc-950";
pub const SHEET_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";
pub const SHEET_CLOSE_BASE_CLASS: &str = "absolute right-4 top-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none";

pub fn sheet_overlay_class(class: &str) -> String {
  classes([Some(SHEET_OVERLAY_BASE_CLASS), Some(class)])
}

pub fn sheet_content_class(side: SheetSide, class: &str) -> String {
  classes([Some(SHEET_CONTENT_BASE_CLASS), Some(side.class()), Some(class)])
}

pub fn sheet_header_class(class: &str) -> String {
  classes([Some(SHEET_HEADER_BASE_CLASS), Some(class)])
}

pub fn sheet_footer_class(class: &str) -> String {
  classes([Some(SHEET_FOOTER_BASE_CLASS), Some(class)])
}

pub fn sheet_title_class(class: &str) -> String {
  classes([Some(SHEET_TITLE_BASE_CLASS), Some(class)])
}

pub fn sheet_description_class(class: &str) -> String {
  classes([Some(SHEET_DESCRIPTION_BASE_CLASS), Some(class)])
}

pub fn sheet_close_class(class: &str) -> String {
  classes([Some(SHEET_CLOSE_BASE_CLASS), Some(class)])
}

#[component]
pub fn SheetOverlay(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
) -> Element {
  let class = sheet_overlay_class(&class);

  rsx! {
    div {
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      onclick: move |_| {
        if let Some(handler) = on_open_change.filter(|_| dismiss.outside_pointer) {
          handler.call(false);
        }
      },
    }
  }
}

#[component]
pub fn SheetContent(
  #[props(default)] open: bool,
  #[props(default)] side: SheetSide,
  #[props(default)] class: String,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let class = sheet_content_class(side, &class);
  let focus_scope = use_modal_focus_scope(open);

  rsx! {
    div {
      role: "dialog",
      class,
      hidden: !open,
      tabindex: "-1",
      "aria-modal": "true",
      "data-side": side.attribute(),
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-focus-scope": focus_scope,
      onkeydown: move |event| {
        let wants_close = event.key() == Key::Escape && dismiss.escape_key;
        if let Some(handler) = on_open_change.filter(|_| wants_close) {
          handler.call(false);
        }
      },
      {children}
    }
  }
}

#[component]
pub fn SheetHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = sheet_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn SheetFooter(#[props(default)] class: String, children: Element) -> Element {
  let class = sheet_footer_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn SheetTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = sheet_title_class(&class);

  rsx! {
    h2 {
      class,
      {children}
    }
  }
}

#[component]
pub fn SheetDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = sheet_description_class(&class);

  rsx! {
    p {
      class,
      {children}
    }
  }
}

#[component]
pub fn SheetClose(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let class = sheet_close_class(&class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      onclick: move |_| {
        if let Some(handler) = on_open_change {
          handler.call(false);
        }
      },
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sheet_content_class_includes_side_and_user_class() {
    let actual = sheet_content_class(SheetSide::Left, "w-80");

    assert!(actual.contains(SHEET_CONTENT_BASE_CLASS));
    assert!(actual.contains("inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm"));
    assert!(actual.ends_with("w-80"));
  }

  #[test]
  fn sheet_side_attribute_matches_side() {
    assert_eq!(SheetSide::Top.attribute(), "top");
    assert_eq!(SheetSide::Right.attribute(), "right");
    assert_eq!(SheetSide::Bottom.attribute(), "bottom");
    assert_eq!(SheetSide::Left.attribute(), "left");
  }

  #[test]
  fn sheet_uses_dialog_primitive_defaults() {
    let config = DialogPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert!(config.dismiss.escape_key);
    assert!(!config.dismiss.outside_pointer);
    assert_eq!(config.focus_return, FocusReturn::Trigger);
  }
}
