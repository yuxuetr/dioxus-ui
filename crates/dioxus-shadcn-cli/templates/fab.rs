use std::rc::Rc;

use super::overlay_root::{OverlayRoot, use_overlay_root};
use super::root_state::use_root_context;
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

// The trigger comes first in source, so Tab reaches it before the actions,
// and `flex-col-reverse` stacks the actions above it.
pub const FAB_BASE_CLASS: &str = "z-40 flex flex-col-reverse items-end gap-3";
pub const FAB_FIXED_CLASS: &str = "fixed end-6 bottom-[calc(1.5rem+env(safe-area-inset-bottom))]";
pub const FAB_STATIC_CLASS: &str = "relative";
pub const FAB_TRIGGER_CLASS: &str = "inline-flex size-14 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-lg transition-colors hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background [&_svg]:size-6";
pub const FAB_ACTIONS_CLASS: &str = "flex flex-col items-end gap-3";
pub const FAB_ACTION_CLASS: &str =
  "group/fab-action inline-flex items-center gap-3 text-sm font-medium focus-visible:outline-none";
pub const FAB_ACTION_LABEL_CLASS: &str =
  "rounded-md border border-border bg-popover px-2 py-1 text-popover-foreground shadow-md";
pub const FAB_ACTION_ICON_CLASS: &str = "inline-flex size-11 items-center justify-center rounded-full bg-secondary text-secondary-foreground shadow-md group-focus-visible/fab-action:ring-2 group-focus-visible/fab-action:ring-ring [&_svg]:size-5";

pub fn fab_class(fixed: bool, class: &str) -> String {
  let position = if fixed { FAB_FIXED_CLASS } else { FAB_STATIC_CLASS };
  merge_classes(classes([Some(FAB_BASE_CLASS), Some(position)]), class)
}

pub fn fab_action_class(class: &str) -> String {
  merge_classes(classes([Some(FAB_ACTION_CLASS)]), class)
}

/// What a speed dial `Fab` shares with its actions.
#[derive(Clone, Copy)]
struct FabContext(OverlayRoot);

/// A floating action button, fixed to the bottom inline-end corner;
/// `fixed: false` keeps it in the flow. Without children it is a plain button
/// whose `onclick` runs the action. With `FabAction` children it is a speed
/// dial that owns whether it is open: a press toggles it, the actions show
/// above it while open, and Escape closes it and returns focus to the
/// trigger. Pass `open` to control it, or `default_open` to start it;
/// `on_open_change` hears every change either way. `icon` is the trigger's
/// content; name the trigger with `aria-label`.
#[component]
pub fn Fab(
  icon: Element,
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default = true)] fixed: bool,
  #[props(default)] class: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = fab_class(fixed, &class);
  let root = use_overlay_root("fab", open, default_open, on_open_change);
  use_context_provider(|| FabContext(root));
  let mut trigger = use_signal(|| None::<Rc<MountedData>>);
  // A Fab without children gets Dioxus's shared empty node.
  let dial = children.as_ref().is_ok_and(|node| *node != VNode::placeholder());
  let open = dial && root.is_open();
  let expanded = dial.then(|| open.to_string());
  let controls = dial.then(|| root.content_id());

  rsx! {
    div {
      class,
      onkeydown: move |event| {
        if open && event.key() == Key::Escape {
          event.prevent_default();
          root.set_open.call(false);
          if let Some(element) = trigger() {
            spawn(async move {
              // A failed focus leaves focus where it was; nothing to undo.
              let _ = element.set_focus(true).await;
            });
          }
        }
      },
      button {
        id: root.trigger_id(),
        class: FAB_TRIGGER_CLASS,
        r#type: "button",
        "aria-expanded": expanded,
        "aria-controls": controls,
        onmounted: move |event| trigger.set(Some(event.data())),
        onclick: move |event| {
          if dial {
            root.set_open.call(!open);
          } else if let Some(handler) = onclick {
            handler.call(event);
          }
        },
        ..attributes,
        {icon}
      }
      // The container stays, hidden, so `aria-controls` always resolves;
      // the actions render only while open.
      if dial {
        div { id: root.content_id(), class: FAB_ACTIONS_CLASS, role: "group", hidden: !open,
          if open {
            {children}
          }
        }
      }
    }
  }
}

/// One speed dial action: a button with a visible `label` beside a round
/// icon, so it needs no `aria-label`. A press runs `onclick` and closes the
/// dial.
#[component]
pub fn FabAction(
  label: String,
  #[props(default)] class: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let root = use_root_context::<FabContext>("FabAction", "Fab").0;
  let class = fab_action_class(&class);

  rsx! {
    button {
      class,
      r#type: "button",
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
        root.set_open.call(false);
      },
      ..attributes,
      span { class: FAB_ACTION_LABEL_CLASS, "{label}" }
      span { class: FAB_ACTION_ICON_CLASS, "aria-hidden": "true", {children} }
    }
  }
}
