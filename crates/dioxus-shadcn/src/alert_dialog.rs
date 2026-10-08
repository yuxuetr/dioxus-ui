//! Alert dialog: a modal that asks the user to confirm a destructive or
//! high-impact action before it happens.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};

use crate::default_attribute::default_attribute;
use crate::dialog_labels::{DialogLabelPart, use_dialog_label_part, use_dialog_labels};
use crate::layer::use_layer;
use crate::modal_focus::use_modal_focus_scope;
use crate::overlay_root::{OverlayRoot, overlay_trigger, use_overlay_root};
use crate::root_state::use_root_context;

/// The look of `AlertDialogAction`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AlertDialogActionVariant {
  /// The primary button look.
  #[default]
  Default,
  /// The destructive color, for an action that deletes or cannot be undone.
  Destructive,
}

impl AlertDialogActionVariant {
  /// The background, text, and hover colors of this variant.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
      Self::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
    }
  }
}

const ALERT_DIALOG_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
const ALERT_DIALOG_CONTENT_BASE_CLASS: &str = "fixed left-1/2 top-1/2 z-50 grid w-full max-w-lg -translate-x-1/2 -translate-y-1/2 gap-4 rounded-md border border-border bg-background p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
const ALERT_DIALOG_HEADER_BASE_CLASS: &str = "flex flex-col gap-2 text-center sm:text-left";
const ALERT_DIALOG_FOOTER_BASE_CLASS: &str =
  "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end";
const ALERT_DIALOG_TITLE_BASE_CLASS: &str = "text-lg font-semibold leading-none text-foreground";
const ALERT_DIALOG_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
const ALERT_DIALOG_ACTION_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md px-4 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const ALERT_DIALOG_CANCEL_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md border border-border bg-background px-4 text-sm font-medium transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

/// Classes for the overlay: the dimmed full-screen backdrop, then `class` merged over it.
pub fn alert_dialog_overlay_class(class: &str) -> String {
  merge_classes(classes([Some(ALERT_DIALOG_OVERLAY_BASE_CLASS)]), class)
}

/// Classes for the content: the centered panel, then `class` merged over it.
pub fn alert_dialog_content_class(class: &str) -> String {
  merge_classes(classes([Some(ALERT_DIALOG_CONTENT_BASE_CLASS)]), class)
}

fn alert_dialog_header_class(class: &str) -> String {
  merge_classes(classes([Some(ALERT_DIALOG_HEADER_BASE_CLASS)]), class)
}

fn alert_dialog_footer_class(class: &str) -> String {
  merge_classes(classes([Some(ALERT_DIALOG_FOOTER_BASE_CLASS)]), class)
}

fn alert_dialog_title_class(class: &str) -> String {
  merge_classes(classes([Some(ALERT_DIALOG_TITLE_BASE_CLASS)]), class)
}

fn alert_dialog_description_class(class: &str) -> String {
  merge_classes(classes([Some(ALERT_DIALOG_DESCRIPTION_BASE_CLASS)]), class)
}

/// Classes for the action button: base classes, the variant's colors, then `class`
/// merged over them.
pub fn alert_dialog_action_class(variant: AlertDialogActionVariant, class: &str) -> String {
  merge_classes(classes([Some(ALERT_DIALOG_ACTION_BASE_CLASS), Some(variant.class())]), class)
}

fn alert_dialog_cancel_class(class: &str) -> String {
  merge_classes(classes([Some(ALERT_DIALOG_CANCEL_BASE_CLASS)]), class)
}

/// What an `AlertDialog` shares with its parts.
#[derive(Clone, Copy)]
struct AlertDialogContext(OverlayRoot);

fn use_alert_dialog(part: &str) -> OverlayRoot {
  use_root_context::<AlertDialogContext>(part, "AlertDialog").0
}

/// The root of an alert dialog: it owns whether it is open. Pass `open` to
/// control it, or `default_open` to start it; `on_open_change` hears every
/// change the user makes either way.
#[component]
pub fn AlertDialog(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let root = use_overlay_root("alert-dialog", open, default_open, on_open_change);
  use_context_provider(|| AlertDialogContext(root));

  rsx! { {children} }
}

/// A button that opens an alert dialog. Style it with `class`, such as
/// `button_class(..)`.
#[component]
pub fn AlertDialogTrigger(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_alert_dialog("AlertDialogTrigger");
  overlay_trigger(root, "dialog", class, disabled, attributes, children)
}

#[component]
pub fn AlertDialogOverlay(#[props(default)] class: String) -> Element {
  let root = use_alert_dialog("AlertDialogOverlay");
  let class = alert_dialog_overlay_class(&class);
  let open = root.is_open();

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
  #[props(default)] class: String,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_alert_dialog("AlertDialogContent");
  let class = alert_dialog_content_class(&class);
  let open = root.is_open();
  let id = default_attribute(&attributes, "id", root.content_id());
  let (labelledby, describedby) = use_dialog_labels().content_attributes(&attributes);
  let focus_scope = use_modal_focus_scope(open, true);
  let layer = use_layer(open);

  rsx! {
    div {
      role: "alertdialog",
      id,
      class,
      "aria-labelledby": labelledby,
      "aria-describedby": describedby,
      hidden: !open,
      tabindex: "-1",
      "aria-modal": "true",
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
  children: Element,
) -> Element {
  let root = use_alert_dialog("AlertDialogAction");
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
        root.set_open.call(false);
      },
      {children}
    }
  }
}

#[component]
pub fn AlertDialogCancel(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  children: Element,
) -> Element {
  let root = use_alert_dialog("AlertDialogCancel");
  let class = alert_dialog_cancel_class(&class);

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
        root.set_open.call(false);
      },
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_root_links_the_trigger_to_the_content() {
    fn app() -> Element {
      rsx! {
        AlertDialog { default_open: true,
          AlertDialogTrigger { "Open" }
          AlertDialogContent { "Body" }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"id="dxui-alert-dialog-0-trigger""#), "{html}");
    assert!(html.contains(r#"aria-expanded="true" aria-controls="dxui-alert-dialog-0-content""#));
    assert!(html.contains(r#"role="alertdialog" id="dxui-alert-dialog-0-content""#), "{html}");
    assert!(!html.contains(" hidden"), "{html}");
  }

  #[test]
  fn ssr_part_outside_its_root_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        AlertDialogContent { "Body" }
      }
    }
    let html = render(app);

    assert!(html.contains("before"));
    assert!(!html.contains("Body"), "{html}");
  }

  #[test]
  fn alert_dialog_content_class_appends_user_class() {
    let actual = alert_dialog_content_class("max-w-md");

    assert_eq!(
      actual,
      "fixed left-1/2 top-1/2 z-50 grid w-full -translate-x-1/2 -translate-y-1/2 gap-4 rounded-md border border-border bg-background p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring max-w-md"
    );
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
