use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};

pub const DRAWER_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
pub const DRAWER_CONTENT_BASE_CLASS: &str = "fixed inset-x-0 bottom-0 z-50 grid max-h-[85vh] gap-4 rounded-t-md border border-zinc-200 bg-white p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";
pub const DRAWER_HEADER_BASE_CLASS: &str = "flex flex-col gap-2 text-center";
pub const DRAWER_FOOTER_BASE_CLASS: &str = "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end";
pub const DRAWER_TITLE_BASE_CLASS: &str = "text-lg font-semibold leading-none text-zinc-950";
pub const DRAWER_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";
pub const DRAWER_CLOSE_BASE_CLASS: &str = "absolute right-4 top-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none";

pub fn drawer_overlay_class(class: &str) -> String {
  classes([Some(DRAWER_OVERLAY_BASE_CLASS), Some(class)])
}

pub fn drawer_content_class(class: &str) -> String {
  classes([Some(DRAWER_CONTENT_BASE_CLASS), Some(class)])
}

pub fn drawer_header_class(class: &str) -> String {
  classes([Some(DRAWER_HEADER_BASE_CLASS), Some(class)])
}

pub fn drawer_footer_class(class: &str) -> String {
  classes([Some(DRAWER_FOOTER_BASE_CLASS), Some(class)])
}

pub fn drawer_title_class(class: &str) -> String {
  classes([Some(DRAWER_TITLE_BASE_CLASS), Some(class)])
}

pub fn drawer_description_class(class: &str) -> String {
  classes([Some(DRAWER_DESCRIPTION_BASE_CLASS), Some(class)])
}

pub fn drawer_close_class(class: &str) -> String {
  classes([Some(DRAWER_CLOSE_BASE_CLASS), Some(class)])
}

#[component]
pub fn DrawerOverlay(#[props(default)] open: bool, #[props(default)] class: String) -> Element {
  let class = drawer_overlay_class(&class);

  rsx! {
    div {
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
    }
  }
}

#[component]
pub fn DrawerContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = drawer_content_class(&class);

  rsx! {
    div {
      role: "dialog",
      class,
      hidden: !open,
      "aria-modal": "true",
      "data-side": "bottom",
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[component]
pub fn DrawerHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = drawer_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn DrawerFooter(#[props(default)] class: String, children: Element) -> Element {
  let class = drawer_footer_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn DrawerTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = drawer_title_class(&class);

  rsx! {
    h2 {
      class,
      {children}
    }
  }
}

#[component]
pub fn DrawerDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = drawer_description_class(&class);

  rsx! {
    p {
      class,
      {children}
    }
  }
}

#[component]
pub fn DrawerClose(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  children: Element,
) -> Element {
  let class = drawer_close_class(&class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn drawer_content_class_appends_user_class() {
    let actual = drawer_content_class("max-h-[70vh]");

    assert!(actual.contains(DRAWER_CONTENT_BASE_CLASS));
    assert!(actual.contains("bottom-0"));
    assert!(actual.ends_with("max-h-[70vh]"));
  }

  #[test]
  fn drawer_uses_dialog_primitive_defaults() {
    let config = DialogPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert!(config.dismiss.escape_key);
    assert!(!config.dismiss.outside_pointer);
    assert_eq!(config.focus_return, FocusReturn::Trigger);
  }
}
