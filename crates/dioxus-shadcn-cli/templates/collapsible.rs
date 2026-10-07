//! Collapsible: a trigger that shows or hides a region of content. The root owns whether it
//! is open (RFC 0077).
use super::default_attribute::default_attribute;
use super::overlay_root::{OverlayRoot, use_overlay_root};
use super::root_state::use_root_context;
use super::utils::{classes, merge_classes};
use super::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;

const COLLAPSIBLE_BASE_CLASS: &str = "grid gap-2 data-[disabled=true]:opacity-50";
const COLLAPSIBLE_TRIGGER_BASE_CLASS: &str = "inline-flex items-center justify-between gap-2 rounded-md text-sm font-medium text-foreground transition-colors hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const COLLAPSIBLE_CONTENT_BASE_CLASS: &str = "overflow-hidden text-sm text-muted-foreground";
const COLLAPSIBLE_CONTENT_OPEN_CLASS: &str = "block";
const COLLAPSIBLE_CONTENT_CLOSED_CLASS: &str = "hidden";

/// Classes for the root: base classes, plus no pointer events while `disabled`, with `class`
/// merged over them.
pub fn collapsible_class(disabled: bool, class: &str) -> String {
  merge_classes(
    classes([Some(COLLAPSIBLE_BASE_CLASS), disabled.then_some("pointer-events-none")]),
    class,
  )
}

/// Classes for the trigger button, with `class` merged over them.
pub fn collapsible_trigger_class(class: &str) -> String {
  merge_classes(classes([Some(COLLAPSIBLE_TRIGGER_BASE_CLASS)]), class)
}

/// Classes for the content: base classes, shown while `open` and hidden otherwise, with `class`
/// merged over them.
pub fn collapsible_content_class(open: bool, class: &str) -> String {
  let state_class =
    if open { COLLAPSIBLE_CONTENT_OPEN_CLASS } else { COLLAPSIBLE_CONTENT_CLOSED_CLASS };

  merge_classes(classes([Some(COLLAPSIBLE_CONTENT_BASE_CLASS), Some(state_class)]), class)
}

#[derive(Clone, Copy)]
struct CollapsibleContext {
  root: OverlayRoot,
  disabled: bool,
}

/// The root of a collapsible section: it owns whether the content shows
/// (RFC 0077). Pass `open` to control it, or `default_open` to start it;
/// `on_open_change` hears every change the user makes either way. `disabled`
/// disables the trigger.
#[component]
pub fn Collapsible(
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_overlay_root("collapsible", open, default_open, on_open_change);
  use_context_provider(|| CollapsibleContext { root, disabled });
  let class = collapsible_class(disabled, &class);
  let open = root.is_open();

  rsx! {
    div {
      class,
      "data-disabled": disabled.to_string(),
      "data-state": if open { "open" } else { "closed" },
      ..attributes,
      {children}
    }
  }
}

/// A click, Enter, or Space toggles the root. It points at the content while
/// the content shows. Other attributes are passed to the button.
#[component]
pub fn CollapsibleTrigger(
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let context = use_root_context::<CollapsibleContext>("CollapsibleTrigger", "Collapsible");
  let root = context.root;
  let class = collapsible_trigger_class(&with_density(density_control_class(use_density()), &class));
  let disabled = disabled || context.disabled;
  let open = root.is_open();
  let id = default_attribute(&attributes, "id", root.trigger_id());

  rsx! {
    button {
      r#type: "button",
      id,
      class,
      disabled,
      "aria-controls": open.then(|| root.content_id()),
      "aria-expanded": open.to_string(),
      "data-state": if open { "open" } else { "closed" },
      onclick: move |_| root.set_open.call(!root.is_open()),
      ..attributes,
      {children}
    }
  }
}

/// Renders while the root is open, or always with `force_mount`, hidden
/// while closed.
#[component]
pub fn CollapsibleContent(
  #[props(default)] force_mount: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let context = use_root_context::<CollapsibleContext>("CollapsibleContent", "Collapsible");
  let open = context.root.is_open();
  if !open && !force_mount {
    return rsx! {};
  }

  let class = collapsible_content_class(open, &class);

  rsx! {
    div {
      id: context.root.content_id(),
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      ..attributes,
      {children}
    }
  }
}
