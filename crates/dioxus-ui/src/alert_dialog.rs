use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};

use crate::dialog_labels::{DialogLabelPart, use_dialog_label_part, use_dialog_labels};
use crate::modal_focus::use_modal_focus_scope;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AlertDialogActionVariant {
  #[default]
  Default,
  Destructive,
}

impl AlertDialogActionVariant {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
      Self::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
    }
  }
}

pub const ALERT_DIALOG_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
pub const ALERT_DIALOG_CONTENT_BASE_CLASS: &str = "fixed left-1/2 top-1/2 z-50 grid w-full max-w-lg -translate-x-1/2 -translate-y-1/2 gap-4 rounded-md border border-border bg-background p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
pub const ALERT_DIALOG_HEADER_BASE_CLASS: &str = "flex flex-col gap-2 text-center sm:text-left";
pub const ALERT_DIALOG_FOOTER_BASE_CLASS: &str =
  "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end";
pub const ALERT_DIALOG_TITLE_BASE_CLASS: &str =
  "text-lg font-semibold leading-none text-foreground";
pub const ALERT_DIALOG_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
pub const ALERT_DIALOG_ACTION_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md px-4 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
pub const ALERT_DIALOG_CANCEL_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md border border-border bg-background px-4 text-sm font-medium transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

pub fn alert_dialog_overlay_class(class: &str) -> String {
  classes([Some(ALERT_DIALOG_OVERLAY_BASE_CLASS), Some(class)])
}

pub fn alert_dialog_content_class(class: &str) -> String {
  classes([Some(ALERT_DIALOG_CONTENT_BASE_CLASS), Some(class)])
}

pub fn alert_dialog_header_class(class: &str) -> String {
  classes([Some(ALERT_DIALOG_HEADER_BASE_CLASS), Some(class)])
}

pub fn alert_dialog_footer_class(class: &str) -> String {
  classes([Some(ALERT_DIALOG_FOOTER_BASE_CLASS), Some(class)])
}

pub fn alert_dialog_title_class(class: &str) -> String {
  classes([Some(ALERT_DIALOG_TITLE_BASE_CLASS), Some(class)])
}

pub fn alert_dialog_description_class(class: &str) -> String {
  classes([Some(ALERT_DIALOG_DESCRIPTION_BASE_CLASS), Some(class)])
}

pub fn alert_dialog_action_class(variant: AlertDialogActionVariant, class: &str) -> String {
  classes([Some(ALERT_DIALOG_ACTION_BASE_CLASS), Some(variant.class()), Some(class)])
}

pub fn alert_dialog_cancel_class(class: &str) -> String {
  classes([Some(ALERT_DIALOG_CANCEL_BASE_CLASS), Some(class)])
}

#[component]
pub fn AlertDialogOverlay(
  #[props(default)] open: bool,
  #[props(default)] class: String,
) -> Element {
  let class = alert_dialog_overlay_class(&class);

  rsx! {
    div {
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
    }
  }
}

#[component]
pub fn AlertDialogContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = alert_dialog_content_class(&class);
  let (labelledby, describedby) = use_dialog_labels().content_attributes(&attributes);
  let focus_scope = use_modal_focus_scope(open);

  rsx! {
    div {
      role: "alertdialog",
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
pub fn AlertDialogHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = alert_dialog_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn AlertDialogFooter(#[props(default)] class: String, children: Element) -> Element {
  let class = alert_dialog_footer_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn AlertDialogTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = alert_dialog_title_class(&class);
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
pub fn AlertDialogDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = alert_dialog_description_class(&class);
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
pub fn AlertDialogAction(
  #[props(default)] variant: AlertDialogActionVariant,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let class = alert_dialog_action_class(variant, &class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
        if let Some(handler) = on_open_change {
          handler.call(false);
        }
      },
      {children}
    }
  }
}

#[component]
pub fn AlertDialogCancel(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let class = alert_dialog_cancel_class(&class);

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
  fn alert_dialog_content_class_appends_user_class() {
    let actual = alert_dialog_content_class("max-w-md");

    assert!(actual.contains(ALERT_DIALOG_CONTENT_BASE_CLASS));
    assert!(actual.ends_with("max-w-md"));
  }

  #[test]
  fn alert_dialog_action_class_reflects_destructive_variant() {
    let actual = alert_dialog_action_class(AlertDialogActionVariant::Destructive, "w-full");

    assert!(actual.contains(ALERT_DIALOG_ACTION_BASE_CLASS));
    assert!(actual.contains("bg-destructive text-destructive-foreground hover:bg-destructive/90"));
    assert!(actual.ends_with("w-full"));
  }

  #[test]
  fn alert_dialog_uses_dialog_primitive_defaults() {
    let config = DialogPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert!(config.dismiss.escape_key);
    assert!(!config.dismiss.outside_pointer);
    assert_eq!(config.focus_return, FocusReturn::Trigger);
  }
}
