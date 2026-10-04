use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const COLLAPSIBLE_BASE_CLASS: &str = "grid gap-2 data-disabled:opacity-50";
pub const COLLAPSIBLE_TRIGGER_BASE_CLASS: &str = "inline-flex items-center justify-between gap-2 rounded-md text-sm font-medium transition-colors hover:text-zinc-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:pointer-events-none disabled:opacity-50";
pub const COLLAPSIBLE_TRIGGER_OPEN_CLASS: &str = "text-zinc-950";
pub const COLLAPSIBLE_TRIGGER_CLOSED_CLASS: &str = "text-zinc-900";
pub const COLLAPSIBLE_CONTENT_BASE_CLASS: &str = "overflow-hidden text-sm text-zinc-600";
pub const COLLAPSIBLE_CONTENT_OPEN_CLASS: &str = "block";
pub const COLLAPSIBLE_CONTENT_CLOSED_CLASS: &str = "hidden";

pub fn collapsible_class(disabled: bool, class: &str) -> String {
  classes([Some(COLLAPSIBLE_BASE_CLASS), disabled.then_some("pointer-events-none"), Some(class)])
}

pub fn collapsible_trigger_class(open: bool, class: &str) -> String {
  let state_class =
    if open { COLLAPSIBLE_TRIGGER_OPEN_CLASS } else { COLLAPSIBLE_TRIGGER_CLOSED_CLASS };

  classes([Some(COLLAPSIBLE_TRIGGER_BASE_CLASS), Some(state_class), Some(class)])
}

pub fn collapsible_content_class(open: bool, class: &str) -> String {
  let state_class =
    if open { COLLAPSIBLE_CONTENT_OPEN_CLASS } else { COLLAPSIBLE_CONTENT_CLOSED_CLASS };

  classes([Some(COLLAPSIBLE_CONTENT_BASE_CLASS), Some(state_class), Some(class)])
}

#[component]
pub fn Collapsible(
  #[props(default)] open: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = collapsible_class(disabled, &class);

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

/// A click, Enter, or Space calls `on_open_change` with the requested state,
/// `!open`; the app passes it back as `open` to every part. Other attributes
/// are passed to the button.
#[component]
pub fn CollapsibleTrigger(
  #[props(default)] open: bool,
  #[props(default)] disabled: bool,
  #[props(default)] controls: Option<String>,
  #[props(default)] class: String,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = collapsible_trigger_class(open, &class);
  let controls = controls.unwrap_or_default();

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-controls": controls,
      "aria-expanded": open.to_string(),
      "data-state": if open { "open" } else { "closed" },
      onclick: move |_| {
        if let Some(handler) = on_open_change {
          handler.call(!open);
        }
      },
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn CollapsibleContent(
  #[props(default)] open: bool,
  #[props(default)] id: Option<String>,
  #[props(default)] force_mount: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  if !open && !force_mount {
    return rsx! {};
  }

  let class = collapsible_content_class(open, &class);
  let id = id.unwrap_or_default();

  rsx! {
    div {
      id,
      class,
      hidden: !open,
      "data-state": if open { "open" } else { "closed" },
      ..attributes,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn collapsible_class_reflects_disabled_state() {
    let actual = collapsible_class(true, "max-w-sm");

    assert!(actual.contains(COLLAPSIBLE_BASE_CLASS));
    assert!(actual.contains("pointer-events-none"));
    assert!(actual.ends_with("max-w-sm"));
  }

  #[test]
  fn collapsible_trigger_class_reflects_open_state() {
    let actual = collapsible_trigger_class(true, "w-full");

    assert!(actual.contains(COLLAPSIBLE_TRIGGER_BASE_CLASS));
    assert!(actual.contains(COLLAPSIBLE_TRIGGER_OPEN_CLASS));
    assert!(actual.ends_with("w-full"));
  }

  #[test]
  fn collapsible_content_class_reflects_closed_state() {
    let actual = collapsible_content_class(false, "pt-2");

    assert!(actual.contains(COLLAPSIBLE_CONTENT_BASE_CLASS));
    assert!(actual.contains(COLLAPSIBLE_CONTENT_CLOSED_CLASS));
    assert!(actual.ends_with("pt-2"));
  }
}
