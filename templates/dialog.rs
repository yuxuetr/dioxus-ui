use dioxus::prelude::*;
use super::utils::classes;
pub use super::utils::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};

pub const DIALOG_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
pub const DIALOG_CONTENT_BASE_CLASS: &str = "fixed left-1/2 top-1/2 z-50 grid w-full max-w-lg -translate-x-1/2 -translate-y-1/2 gap-4 rounded-md border border-zinc-200 bg-white p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";
pub const DIALOG_TITLE_BASE_CLASS: &str = "text-lg font-semibold leading-none text-zinc-950";
pub const DIALOG_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";
pub const DIALOG_CLOSE_BASE_CLASS: &str = "absolute right-4 top-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none";

pub fn dialog_overlay_class(class: &str) -> String {
  classes([Some(DIALOG_OVERLAY_BASE_CLASS), Some(class)])
}

pub fn dialog_content_class(class: &str) -> String {
  classes([Some(DIALOG_CONTENT_BASE_CLASS), Some(class)])
}

pub fn dialog_title_class(class: &str) -> String {
  classes([Some(DIALOG_TITLE_BASE_CLASS), Some(class)])
}

pub fn dialog_description_class(class: &str) -> String {
  classes([Some(DIALOG_DESCRIPTION_BASE_CLASS), Some(class)])
}

pub fn dialog_close_class(class: &str) -> String {
  classes([Some(DIALOG_CLOSE_BASE_CLASS), Some(class)])
}

#[component]
pub fn DialogOverlay(
  #[props(default)] open: bool,
  #[props(default)] class: String,
) -> Element {
  let class = dialog_overlay_class(&class);

  rsx! {
    div {
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
    }
  }
}

#[component]
pub fn DialogContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = dialog_content_class(&class);

  rsx! {
    div {
      role: "dialog",
      class,
      hidden: !open,
      "aria-modal": "true",
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

#[component]
pub fn DialogTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = dialog_title_class(&class);

  rsx! {
    h2 {
      class,
      {children}
    }
  }
}

#[component]
pub fn DialogDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = dialog_description_class(&class);

  rsx! {
    p {
      class,
      {children}
    }
  }
}

#[component]
pub fn DialogClose(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  children: Element,
) -> Element {
  let class = dialog_close_class(&class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      {children}
    }
  }
}
