use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};
pub use dioxus_shadcn_primitives::{
  DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig,
};

use crate::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use crate::default_attribute::default_attribute;
use crate::dialog_labels::{DialogLabelPart, use_dialog_label_part, use_dialog_labels};
use crate::overlay_root::{OverlayRoot, overlay_trigger, use_overlay_root};
use crate::root_state::use_root_context;

pub const POPOVER_CONTENT_BASE_CLASS: &str = "z-50 w-72 rounded-md border border-border bg-popover p-4 text-popover-foreground shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
pub const POPOVER_HEADER_BASE_CLASS: &str = "grid gap-1";
pub const POPOVER_TITLE_BASE_CLASS: &str = "font-medium leading-none text-foreground";
pub const POPOVER_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";

pub fn popover_content_class(class: &str) -> String {
  merge_classes(classes([Some(POPOVER_CONTENT_BASE_CLASS)]), class)
}

pub fn popover_header_class(class: &str) -> String {
  merge_classes(classes([Some(POPOVER_HEADER_BASE_CLASS)]), class)
}

pub fn popover_title_class(class: &str) -> String {
  merge_classes(classes([Some(POPOVER_TITLE_BASE_CLASS)]), class)
}

pub fn popover_description_class(class: &str) -> String {
  merge_classes(classes([Some(POPOVER_DESCRIPTION_BASE_CLASS)]), class)
}

/// What a `Popover` shares with its parts.
#[derive(Clone, Copy)]
struct PopoverContext(OverlayRoot);

fn use_popover(part: &str) -> OverlayRoot {
  use_root_context::<PopoverContext>(part, "Popover").0
}

/// The root of a popover: it owns whether it is open. Pass `open` to control
/// it, or `default_open` to start it; `on_open_change` hears every change the
/// user makes either way.
#[component]
pub fn Popover(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let root = use_overlay_root("popover", open, default_open, on_open_change);
  use_context_provider(|| PopoverContext(root));

  rsx! { {children} }
}

/// A button that toggles the popover and anchors its content. Style it with
/// `class`, such as `button_class(..)`.
#[component]
pub fn PopoverTrigger(
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_popover("PopoverTrigger");
  overlay_trigger(root, "dialog", class, disabled, attributes, children)
}

/// Non-modal content, placed next to the `PopoverTrigger` with fixed
/// positioning, flipping and shifting to stay in the viewport. Escape and
/// outside interactions close it per `dismiss`.
#[component]
pub fn PopoverContent(
  #[props(default)] class: String,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_popover("PopoverContent");
  let class = popover_content_class(&class);
  let open = root.is_open();
  let id = default_attribute(&attributes, "id", root.content_id());
  let (labelledby, describedby) = use_dialog_labels().content_attributes(&attributes);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement {
      anchor_id: Some(root.trigger_id()),
      anchor_point: None,
      side,
      align,
      side_offset,
    },
    dismiss,
    Some(root.set_open),
  );

  rsx! {
    div {
      role: "dialog",
      id,
      class,
      "aria-labelledby": labelledby,
      "aria-describedby": describedby,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      "data-dxui-anchored": anchored,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn PopoverHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = popover_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn PopoverTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = popover_title_class(&class);
  let id = use_dialog_label_part(DialogLabelPart::Title);

  rsx! {
    h3 {
      class,
      id,
      {children}
    }
  }
}

#[component]
pub fn PopoverDescription(#[props(default)] class: String, children: Element) -> Element {
  let class = popover_description_class(&class);
  let id = use_dialog_label_part(DialogLabelPart::Description);

  rsx! {
    p {
      class,
      id,
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
  fn ssr_popover_anchors_its_content_to_the_trigger() {
    fn app() -> Element {
      rsx! {
        Popover { default_open: true,
          PopoverTrigger { "Share" }
          PopoverContent { PopoverTitle { "Share link" } }
        }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"id="dxui-popover-0-trigger""#), "{html}");
    assert!(html.contains(r#"aria-expanded="true" aria-controls="dxui-popover-0-content""#));
    assert!(html.contains(r#"role="dialog" id="dxui-popover-0-content""#), "{html}");
    assert!(!html.contains(" hidden"), "{html}");
  }

  #[test]
  fn ssr_popover_part_outside_its_root_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        PopoverContent { "Body" }
      }
    }
    let html = render(app);

    assert!(html.contains("before"));
    assert!(!html.contains("Body"), "{html}");
  }

  #[test]
  fn popover_content_class_appends_user_class() {
    let actual = popover_content_class("w-80");

    assert_eq!(
      actual,
      "z-50 rounded-md border border-border bg-popover p-4 text-popover-foreground shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring w-80"
    );
    assert!(actual.ends_with("w-80"));
  }

  #[test]
  fn popover_primitive_config_is_reexported() {
    let config = PopoverPrimitiveConfig::controlled(true);

    assert!(config.open);
    assert_eq!(config.side, OverlaySide::Bottom);
    assert_eq!(config.align, OverlayAlign::Center);
  }
}
