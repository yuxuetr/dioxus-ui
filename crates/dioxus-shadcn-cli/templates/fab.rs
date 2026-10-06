use std::rc::Rc;

use super::element_id::next_element_id;
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

/// A floating action button, fixed to the bottom inline-end corner;
/// `fixed: false` keeps it in the flow. Without `on_open_change` it is a
/// plain button whose `onclick` runs the action. With it, it is a speed
/// dial: a press calls `on_open_change`, `open` shows the `FabAction`
/// children above it, and Escape closes it and returns focus to the trigger.
/// `icon` is the trigger's content; name the trigger with `aria-label`.
#[component]
pub fn Fab(
  icon: Element,
  #[props(default)] open: bool,
  #[props(default = true)] fixed: bool,
  #[props(default)] class: String,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = fab_class(fixed, &class);
  let actions_id = use_hook(|| format!("dxui-fab-{}", next_element_id()));
  let mut trigger = use_signal(|| None::<Rc<MountedData>>);
  let dial = on_open_change.is_some();
  let expanded = dial.then(|| open.to_string());
  let controls = dial.then(|| actions_id.clone());

  rsx! {
    div {
      class,
      onkeydown: move |event| {
        if dial && open && event.key() == Key::Escape {
          event.prevent_default();
          if let Some(handler) = on_open_change {
            handler.call(false);
          }
          if let Some(element) = trigger() {
            spawn(async move {
              // A failed focus leaves focus where it was; nothing to undo.
              let _ = element.set_focus(true).await;
            });
          }
        }
      },
      button {
        class: FAB_TRIGGER_CLASS,
        r#type: "button",
        "aria-expanded": expanded,
        "aria-controls": controls,
        onmounted: move |event| trigger.set(Some(event.data())),
        onclick: move |event| {
          if let Some(handler) = on_open_change {
            handler.call(!open);
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
        div { id: actions_id, class: FAB_ACTIONS_CLASS, role: "group", hidden: !open,
          if open {
            {children}
          }
        }
      }
    }
  }
}

/// One speed dial action: a button with a visible `label` beside a round
/// icon, so it needs no `aria-label`. Close the dial in its `onclick`.
#[component]
pub fn FabAction(
  label: String,
  #[props(default)] class: String,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = fab_action_class(&class);

  rsx! {
    button {
      class,
      r#type: "button",
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
      },
      ..attributes,
      span { class: FAB_ACTION_LABEL_CLASS, "{label}" }
      span { class: FAB_ACTION_ICON_CLASS, "aria-hidden": "true", {children} }
    }
  }
}
