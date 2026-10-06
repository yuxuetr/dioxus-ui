use super::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use super::default_attribute::default_attribute;
use super::dialog_labels::{DialogLabelPart, use_dialog_label_part, use_dialog_labels};
pub use super::overlay::{DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig};
use super::overlay_root::{OverlayRoot, overlay_trigger, use_overlay_root};
use super::root_state::use_root_context;
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

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
