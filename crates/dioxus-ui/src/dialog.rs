use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};

use crate::modal_focus::use_modal_focus_scope;

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

/// Backdrop that requests close on click when `dismiss.outside_pointer` is set.
#[component]
pub fn DialogOverlay(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
) -> Element {
  let class = dialog_overlay_class(&class);

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

/// Modal content that takes focus on open, keeps Tab inside, restores focus on
/// close, and requests close on Escape when `dismiss.escape_key` is set.
#[component]
pub fn DialogContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
  children: Element,
) -> Element {
  let class = dialog_content_class(&class);
  let focus_scope = use_modal_focus_scope(open);

  rsx! {
    div {
      role: "dialog",
      class,
      hidden: !open,
      tabindex: "-1",
      "aria-modal": "true",
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
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let class = dialog_close_class(&class);

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
  fn dialog_content_class_appends_user_class() {
    let actual = dialog_content_class("max-w-xl");

    assert!(actual.contains(DIALOG_CONTENT_BASE_CLASS));
    assert!(actual.ends_with("max-w-xl"));
  }

  #[test]
  fn dialog_primitive_config_is_reexported() {
    let config = DialogPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
