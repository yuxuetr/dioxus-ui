use super::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
use super::dialog_labels::{DialogLabelPart, use_dialog_label_part, use_dialog_labels};
pub use super::overlay::{DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig};
use super::utils::classes;
use dioxus::prelude::*;

pub const POPOVER_CONTENT_BASE_CLASS: &str = "z-50 w-72 rounded-md border border-border bg-popover p-4 text-popover-foreground shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
pub const POPOVER_HEADER_BASE_CLASS: &str = "grid gap-1";
pub const POPOVER_TITLE_BASE_CLASS: &str = "font-medium leading-none text-foreground";
pub const POPOVER_DESCRIPTION_BASE_CLASS: &str = "text-sm text-muted-foreground";

pub fn popover_content_class(class: &str) -> String {
  classes([Some(POPOVER_CONTENT_BASE_CLASS), Some(class)])
}

pub fn popover_header_class(class: &str) -> String {
  classes([Some(POPOVER_HEADER_BASE_CLASS), Some(class)])
}

pub fn popover_title_class(class: &str) -> String {
  classes([Some(POPOVER_TITLE_BASE_CLASS), Some(class)])
}

pub fn popover_description_class(class: &str) -> String {
  classes([Some(POPOVER_DESCRIPTION_BASE_CLASS), Some(class)])
}

/// Non-modal content. With `anchor_id` it is placed next to that element with
/// fixed positioning, flipping and shifting to stay in the viewport. Escape and
/// outside interactions request close through `on_open_change` per `dismiss`.
#[component]
pub fn PopoverContent(
  #[props(default)] open: bool,
  #[props(default)] class: String,
  #[props(default)] anchor_id: Option<String>,
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = popover_content_class(&class);
  let (labelledby, describedby) = use_dialog_labels().content_attributes(&attributes);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    on_open_change,
  );

  rsx! {
    div {
      role: "dialog",
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
