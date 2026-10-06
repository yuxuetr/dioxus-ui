use super::dialog_labels::{DialogLabelPart, use_dialog_label_part, use_dialog_labels};
use super::modal_focus::use_modal_focus_scope;
pub use super::overlay::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};
use super::utils::classes;
use dioxus::prelude::*;

pub const DRAWER_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
pub const DRAWER_CONTENT_BASE_CLASS: &str = "fixed inset-x-0 bottom-0 z-50 grid max-h-[85vh] gap-4 rounded-t-md border border-border bg-background p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
pub const DRAWER_HEADER_BASE_CLASS: &str = "flex flex-col gap-2 text-center";
pub const DRAWER_FOOTER_BASE_CLASS: &str = "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end";
pub const DRAWER_TITLE_BASE_CLASS: &str = "text-lg font-semibold leading-none text-foreground";
pub const DRAWER_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
pub const DRAWER_CLOSE_BASE_CLASS: &str = "absolute right-4 top-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none";

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
pub fn DrawerOverlay(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
) -> Element {
  let class = drawer_overlay_class(&class);

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
pub fn DrawerContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = drawer_content_class(&class);
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
      "data-side": "bottom",
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
pub fn DrawerDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = drawer_description_class(&class);
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
pub fn DrawerClose(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let class = drawer_close_class(&class);

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
