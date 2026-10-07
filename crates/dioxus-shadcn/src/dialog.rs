use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
};

use crate::default_attribute::default_attribute;
use crate::density::{density_control_class, use_density, with_density};
use crate::dialog_labels::{DialogLabelPart, use_dialog_label_part, use_dialog_labels};
use crate::modal_focus::use_modal_focus_scope;
use crate::overlay_root::{OverlayRoot, overlay_trigger, use_overlay_root};
use crate::root_state::use_root_context;

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

/// What a `Dialog` shares with its parts.
#[derive(Clone, Copy)]
struct DialogContext(OverlayRoot);

fn use_dialog(part: &str) -> OverlayRoot {
  use_root_context::<DialogContext>(part, "Dialog").0
}

/// The root of a dialog: it owns whether the dialog is open. Pass `open` to
/// control it, or `default_open` to start it; `on_open_change` hears every
/// change the user makes either way.
#[component]
pub fn Dialog(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let root = use_overlay_root("dialog", open, default_open, on_open_change);
  use_context_provider(|| DialogContext(root));

  rsx! { {children} }
}

/// A button that opens the dialog. Style it with `class`, such as
/// `button_class(..)`.
#[component]
pub fn DialogTrigger(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_dialog("DialogTrigger");
  overlay_trigger(root, "dialog", class, disabled, attributes, children)
}

/// Backdrop that closes the dialog on click when `dismiss.outside_pointer` is
/// set.
#[component]
pub fn DialogOverlay(
  #[props(default)] class: String,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
) -> Element {
  let root = use_dialog("DialogOverlay");
  let class = dialog_overlay_class(&class);
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

/// Modal content that takes focus on open, keeps Tab inside, restores focus on
/// close, and closes on Escape when `dismiss.escape_key` is set.
#[component]
pub fn DialogContent(
  #[props(default)] class: String,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_dialog("DialogContent");
  let class = dialog_content_class(&class);
  let open = root.is_open();
  let id = default_attribute(&attributes, "id", root.content_id());
  let (labelledby, describedby) = use_dialog_labels().content_attributes(&attributes);
  let focus_scope = use_modal_focus_scope(open, true);

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
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-focus-scope": focus_scope,
      onkeydown: move |event| {
        if event.key() == Key::Escape && dismiss.escape_key {
          root.set_open.call(false);
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
  children: Element,
) -> Element {
  let root = use_dialog("DialogClose");
  let class = dialog_close_class(&with_density(density_control_class(use_density()), &class));

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

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_dialog_links_the_trigger_to_the_content() {
    fn app() -> Element {
      rsx! {
        Dialog { default_open: true,
          DialogTrigger { class: "btn", "Open" }
          DialogOverlay {}
          DialogContent {
            DialogTitle { "Rename" }
            DialogClose { "Close" }
          }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(
      r#"type="button" id="dxui-dialog-0-trigger" class="btn" aria-haspopup="dialog" aria-expanded="true" aria-controls="dxui-dialog-0-content" data-state="open""#
    ), "{html}");
    assert!(html.contains(r#"role="dialog" id="dxui-dialog-0-content""#), "{html}");
    assert!(!html.contains(" hidden"), "{html}");
  }

  #[test]
  fn ssr_controlled_open_wins_over_the_default() {
    fn app() -> Element {
      rsx! {
        Dialog { open: false, default_open: true,
          DialogTrigger { "Open" }
          DialogContent { "Body" }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"aria-expanded="false""#), "{html}");
    assert!(html.contains(r#"role="dialog" id="dxui-dialog-0-content" class=""#));
    assert!(html.contains(" hidden"), "{html}");
  }

  #[test]
  fn ssr_dialog_part_outside_its_root_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        DialogContent { "Body" }
      }
    }
    let html = render(app);

    assert!(html.contains("before"));
    assert!(!html.contains("Body"), "{html}");
  }

  #[test]
  fn dialog_content_class_appends_user_class() {
    let actual = dialog_content_class("max-w-xl");

    assert_eq!(
      actual,
      "fixed left-1/2 top-1/2 z-50 grid w-full -translate-x-1/2 -translate-y-1/2 gap-4 rounded-md border border-border bg-background p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring max-w-xl"
    );
    assert!(actual.ends_with("max-w-xl"));
  }

  #[test]
  fn dialog_primitive_config_is_reexported() {
    let config = DialogPrimitiveConfig::controlled(true);

    assert!(config.open);
  }
}
