use super::dialog_labels::{DialogLabelPart, use_dialog_label_part, use_dialog_labels};
use super::modal_focus::use_modal_focus_scope;
pub use super::overlay::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

pub const DIALOG_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
pub const DIALOG_CONTENT_BASE_CLASS: &str = "fixed left-1/2 top-1/2 z-50 grid w-full max-w-lg -translate-x-1/2 -translate-y-1/2 gap-4 rounded-md border border-border bg-background p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
pub const DIALOG_TITLE_BASE_CLASS: &str = "text-lg font-semibold leading-none text-foreground";
pub const DIALOG_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
pub const DIALOG_CLOSE_BASE_CLASS: &str = "absolute right-4 top-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none";

pub fn dialog_overlay_class(class: &str) -> String {
  merge_classes(classes([Some(DIALOG_OVERLAY_BASE_CLASS)]), class)
}

pub fn dialog_content_class(class: &str) -> String {
  merge_classes(classes([Some(DIALOG_CONTENT_BASE_CLASS)]), class)
}

pub fn dialog_title_class(class: &str) -> String {
  merge_classes(classes([Some(DIALOG_TITLE_BASE_CLASS)]), class)
}

pub fn dialog_description_class(class: &str) -> String {
  merge_classes(classes([Some(DIALOG_DESCRIPTION_BASE_CLASS)]), class)
}

pub fn dialog_close_class(class: &str) -> String {
  merge_classes(classes([Some(DIALOG_CLOSE_BASE_CLASS)]), class)
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
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = dialog_content_class(&class);
  let (labelledby, describedby) = use_dialog_labels().content_attributes(&attributes);
  let focus_scope = use_modal_focus_scope(open, true);

  rsx! {
    div {
      role: "dialog",
      class,
      "aria-labelledby": labelledby,
      "aria-describedby": describedby,
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
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn DialogTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = dialog_title_class(&class);
  let id = use_dialog_label_part(DialogLabelPart::Title);

  rsx! {
    h2 {
      class,
      id,
      {children}
    }
  }
}

#[component]
pub fn DialogDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = dialog_description_class(&class);
  let id = use_dialog_label_part(DialogLabelPart::Description);

  rsx! {
    p {
      class,
      id,
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
