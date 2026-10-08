//! Drawer: a bottom modal for task flows on phones. It reuses the dialog
//! primitive's defaults with bottom-first sizing.
use super::default_attribute::default_attribute;
use super::dialog_labels::{DialogLabelPart, use_dialog_label_part, use_dialog_labels};
use super::modal_focus::use_modal_focus_scope;
pub use super::overlay::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};
use super::overlay_root::{OverlayRoot, overlay_trigger, use_overlay_root};
use super::root_state::use_root_context;
use super::utils::{classes, merge_classes};
use super::layer::use_layer;
use dioxus::prelude::*;

const DRAWER_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
const DRAWER_CONTENT_BASE_CLASS: &str = "fixed inset-x-0 bottom-0 z-50 grid max-h-[85vh] gap-4 rounded-t-md border border-border bg-background p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
const DRAWER_HEADER_BASE_CLASS: &str = "flex flex-col gap-2 text-center";
const DRAWER_FOOTER_BASE_CLASS: &str = "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end";
const DRAWER_TITLE_BASE_CLASS: &str = "text-lg font-semibold leading-none text-foreground";
const DRAWER_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
const DRAWER_CLOSE_BASE_CLASS: &str = "absolute right-4 top-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none";

/// Classes for the dimmed backdrop behind the drawer, with `class` merged over
/// them.
pub fn drawer_overlay_class(class: &str) -> String {
  merge_classes(classes([Some(DRAWER_OVERLAY_BASE_CLASS)]), class)
}

/// Classes for the panel fixed to the bottom of the screen, with `class` merged
/// over them.
pub fn drawer_content_class(class: &str) -> String {
  merge_classes(classes([Some(DRAWER_CONTENT_BASE_CLASS)]), class)
}

fn drawer_header_class(class: &str) -> String {
  merge_classes(classes([Some(DRAWER_HEADER_BASE_CLASS)]), class)
}

fn drawer_footer_class(class: &str) -> String {
  merge_classes(classes([Some(DRAWER_FOOTER_BASE_CLASS)]), class)
}

fn drawer_title_class(class: &str) -> String {
  merge_classes(classes([Some(DRAWER_TITLE_BASE_CLASS)]), class)
}

fn drawer_description_class(class: &str) -> String {
  merge_classes(classes([Some(DRAWER_DESCRIPTION_BASE_CLASS)]), class)
}

fn drawer_close_class(class: &str) -> String {
  merge_classes(classes([Some(DRAWER_CLOSE_BASE_CLASS)]), class)
}

/// What a `Drawer` shares with its parts.
#[derive(Clone, Copy)]
struct DrawerContext(OverlayRoot);

fn use_drawer(part: &str) -> OverlayRoot {
  use_root_context::<DrawerContext>(part, "Drawer").0
}

/// The root of a drawer: it owns whether it is open. Pass `open` to control
/// it, or `default_open` to start it; `on_open_change` hears every change the
/// user makes either way.
#[component]
pub fn Drawer(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let root = use_overlay_root("drawer", open, default_open, on_open_change);
  use_context_provider(|| DrawerContext(root));

  rsx! { {children} }
}

/// A button that opens a drawer. Style it with `class`, such as
/// `button_class(..)`.
#[component]
pub fn DrawerTrigger(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_drawer("DrawerTrigger");
  overlay_trigger(root, "dialog", class, disabled, attributes, children)
}

#[component]
pub fn DrawerOverlay(
  #[props(default)] class: String,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
) -> Element {
  let root = use_drawer("DrawerOverlay");
  let class = drawer_overlay_class(&class);
  let open = root.is_open();

  rsx! {
    div {
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      onclick: move |_| {
        if dismiss.outside_pointer {
          root.set_open.call(false);
        }
      },
    }
  }
}

#[component]
pub fn DrawerContent(
  #[props(default)] class: String,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_drawer("DrawerContent");
  let class = drawer_content_class(&class);
  let open = root.is_open();
  let id = default_attribute(&attributes, "id", root.content_id());
  let (labelledby, describedby) = use_dialog_labels().content_attributes(&attributes);
  let focus_scope = use_modal_focus_scope(open, true);
  let layer = use_layer(open);

  rsx! {
    div {
      role: "dialog",
      id,
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
        // An overlay opened inside, such as a popover, takes its Escape first.
        if event.key() == Key::Escape && dismiss.escape_key && layer.is_top() {
          root.set_open.call(false);
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
  children: Element,
) -> Element {
  let root = use_drawer("DrawerClose");
  let class = drawer_close_class(&class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      onclick: move |_| root.set_open.call(false),
      {children}
    }
  }
}
