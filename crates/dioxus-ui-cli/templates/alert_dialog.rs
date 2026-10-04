pub use super::utils::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};
use super::utils::{classes, use_modal_focus_scope};
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlertDialogActionVariant {
  Default,
  Destructive,
}

impl AlertDialogActionVariant {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => "bg-blue-600 text-white hover:bg-blue-700",
      Self::Destructive => "bg-red-600 text-white hover:bg-red-700",
    }
  }
}

pub const ALERT_DIALOG_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
pub const ALERT_DIALOG_CONTENT_BASE_CLASS: &str = "fixed left-1/2 top-1/2 z-50 grid w-full max-w-lg -translate-x-1/2 -translate-y-1/2 gap-4 rounded-md border border-zinc-200 bg-white p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600";
pub const ALERT_DIALOG_HEADER_BASE_CLASS: &str = "flex flex-col gap-2 text-center sm:text-left";
pub const ALERT_DIALOG_FOOTER_BASE_CLASS: &str =
  "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end";
pub const ALERT_DIALOG_TITLE_BASE_CLASS: &str = "text-lg font-semibold leading-none text-zinc-950";
pub const ALERT_DIALOG_DESCRIPTION_BASE_CLASS: &str = "text-sm text-zinc-600";
pub const ALERT_DIALOG_ACTION_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md px-4 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const ALERT_DIALOG_CANCEL_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md border border-zinc-200 bg-white px-4 text-sm font-medium transition-colors hover:bg-zinc-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";

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
  children: Element,
) -> Element {
  let class = alert_dialog_content_class(&class);
  let focus_scope = use_modal_focus_scope(open);

  rsx! {
    div {
      role: "alertdialog",
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

  rsx! {
    h2 {
      class,
      {children}
    }
  }
}

#[component]
pub fn AlertDialogDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = alert_dialog_description_class(&class);

  rsx! {
    p {
      class,
      {children}
    }
  }
}

#[component]
pub fn AlertDialogAction(
  #[props(default = AlertDialogActionVariant::Default)] variant: AlertDialogActionVariant,
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
