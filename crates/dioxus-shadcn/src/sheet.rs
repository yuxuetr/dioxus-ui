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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SheetSide {
  Top,
  #[default]
  Right,
  Bottom,
  Left,
}

impl SheetSide {
  pub const fn class(self) -> &'static str {
    match self {
      Self::Top => "inset-x-0 top-0 h-auto border-b",
      Self::Right => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm",
      Self::Bottom => "inset-x-0 bottom-0 h-auto border-t",
      Self::Left => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm",
    }
  }

  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Top => "top",
      Self::Right => "right",
      Self::Bottom => "bottom",
      Self::Left => "left",
    }
  }
}

pub const SHEET_OVERLAY_BASE_CLASS: &str = "fixed inset-0 z-50 bg-black/50";
pub const SHEET_CONTENT_BASE_CLASS: &str = "fixed z-50 gap-4 border-border bg-background p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
pub const SHEET_HEADER_BASE_CLASS: &str = "flex flex-col gap-2 text-left";
pub const SHEET_FOOTER_BASE_CLASS: &str = "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end";
pub const SHEET_TITLE_BASE_CLASS: &str = "text-lg font-semibold leading-none text-foreground";
pub const SHEET_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";
pub const SHEET_CLOSE_BASE_CLASS: &str = "absolute right-4 top-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none";

pub fn sheet_overlay_class(class: &str) -> String {
  merge_classes(classes([Some(SHEET_OVERLAY_BASE_CLASS)]), class)
}

pub fn sheet_content_class(side: SheetSide, class: &str) -> String {
  merge_classes(classes([Some(SHEET_CONTENT_BASE_CLASS), Some(side.class())]), class)
}

pub fn sheet_header_class(class: &str) -> String {
  merge_classes(classes([Some(SHEET_HEADER_BASE_CLASS)]), class)
}

pub fn sheet_footer_class(class: &str) -> String {
  merge_classes(classes([Some(SHEET_FOOTER_BASE_CLASS)]), class)
}

pub fn sheet_title_class(class: &str) -> String {
  merge_classes(classes([Some(SHEET_TITLE_BASE_CLASS)]), class)
}

pub fn sheet_description_class(class: &str) -> String {
  merge_classes(classes([Some(SHEET_DESCRIPTION_BASE_CLASS)]), class)
}

pub fn sheet_close_class(class: &str) -> String {
  merge_classes(classes([Some(SHEET_CLOSE_BASE_CLASS)]), class)
}

/// What a `Sheet` shares with its parts.
#[derive(Clone, Copy)]
struct SheetContext(OverlayRoot);

fn use_sheet(part: &str) -> OverlayRoot {
  use_root_context::<SheetContext>(part, "Sheet").0
}

/// The root of a sheet: it owns whether it is open. Pass `open` to control
/// it, or `default_open` to start it; `on_open_change` hears every change the
/// user makes either way.
#[component]
pub fn Sheet(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let root = use_overlay_root("sheet", open, default_open, on_open_change);
  use_context_provider(|| SheetContext(root));

  rsx! { {children} }
}

/// A button that opens a sheet. Style it with `class`, such as
/// `button_class(..)`.
#[component]
pub fn SheetTrigger(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_sheet("SheetTrigger");
  overlay_trigger(root, "dialog", class, disabled, attributes, children)
}

#[component]
pub fn SheetOverlay(
  #[props(default)] class: String,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
) -> Element {
  let root = use_sheet("SheetOverlay");
  let class = sheet_overlay_class(&class);
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
pub fn SheetContent(
  #[props(default)] side: SheetSide,
  #[props(default)] class: String,
  #[props(default = DismissBehavior::dialog_default())] dismiss: DismissBehavior,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_sheet("SheetContent");
  let class = sheet_content_class(side, &class);
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
      "data-side": side.attribute(),
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
pub fn SheetHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = sheet_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn SheetFooter(#[props(default)] class: String, children: Element) -> Element {
  let class = sheet_footer_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn SheetTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = sheet_title_class(&class);
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
pub fn SheetDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = sheet_description_class(&class);
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
pub fn SheetClose(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  children: Element,
) -> Element {
  let root = use_sheet("SheetClose");
  let class = sheet_close_class(&with_density(density_control_class(use_density()), &class));

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
  fn ssr_root_links_the_trigger_to_the_content() {
    fn app() -> Element {
      rsx! {
        Sheet { default_open: true,
          SheetTrigger { "Open" }
          SheetContent { "Body" }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"id="dxui-sheet-0-trigger""#), "{html}");
    assert!(html.contains(r#"aria-expanded="true" aria-controls="dxui-sheet-0-content""#));
    assert!(html.contains(r#"role="dialog" id="dxui-sheet-0-content""#), "{html}");
    assert!(!html.contains(" hidden"), "{html}");
  }

  #[test]
  fn ssr_part_outside_its_root_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        SheetContent { "Body" }
      }
    }
    let html = render(app);

    assert!(html.contains("before"));
    assert!(!html.contains("Body"), "{html}");
  }

  #[test]
  fn sheet_content_class_includes_side_and_user_class() {
    let actual = sheet_content_class(SheetSide::Left, "w-80");

    assert!(actual.contains(SHEET_CONTENT_BASE_CLASS));
    assert_eq!(
      actual,
      "fixed z-50 gap-4 border-border bg-background p-6 shadow-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring inset-y-0 left-0 h-full border-r sm:max-w-sm w-80"
    );
    assert!(actual.ends_with("w-80"));
  }

  #[test]
  fn sheet_side_attribute_matches_side() {
    assert_eq!(SheetSide::Top.attribute(), "top");
    assert_eq!(SheetSide::Right.attribute(), "right");
    assert_eq!(SheetSide::Bottom.attribute(), "bottom");
    assert_eq!(SheetSide::Left.attribute(), "left");
  }

  #[test]
  fn sheet_uses_dialog_primitive_defaults() {
    let config = DialogPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert!(config.dismiss.escape_key);
    assert!(!config.dismiss.outside_pointer);
    assert_eq!(config.focus_return, FocusReturn::Trigger);
  }
}
