//! Switch: a controlled on/off control with a styled track and thumb.
use super::utils::{classes, merge_classes};
use super::density::{density_hit_area_class, use_density, with_density};
use dioxus::prelude::*;

const SWITCH_BASE_CLASS: &str = "inline-flex h-6 w-11 shrink-0 items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
const SWITCH_THUMB_BASE_CLASS: &str =
  "pointer-events-none block h-5 w-5 rounded-full bg-background shadow transition-transform";

/// Classes for the track: base classes, the primary fill when `checked`, then `class`
/// merged over them.
pub fn switch_class(checked: bool, class: &str) -> String {
  let checked_class = if checked { "bg-primary" } else { "bg-input" };

  merge_classes(classes([Some(SWITCH_BASE_CLASS), Some(checked_class)]), class)
}

/// Classes for the thumb, moved to the end of the track when `checked`.
pub fn switch_thumb_class(checked: bool) -> String {
  let checked_class = if checked { "translate-x-5" } else { "translate-x-0" };

  classes([Some(SWITCH_THUMB_BASE_CLASS), Some(checked_class)])
}

/// Renders `data-state` for `data-[state=checked]:` style variants.
fn switch_state(checked: bool) -> &'static str {
  if checked { "checked" } else { "unchecked" }
}

/// A controlled switch. A click, Space, Enter, or a click on its `Label` calls
/// `on_checked_change` with the requested state, `!checked`; the app passes it
/// back as `checked`. Other attributes, such as `id` and `aria-label`, are
/// passed to the button.
#[component]
pub fn Switch(
  #[props(default)] checked: bool,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] on_checked_change: Option<EventHandler<bool>>,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
  let class = switch_class(checked, &with_density(density_hit_area_class(use_density()), &class));
  let thumb_class = switch_thumb_class(checked);
  let state = switch_state(checked);

  rsx! {
    button {
      r#type: "button",
      role: "switch",
      class,
      disabled,
      "aria-checked": checked.to_string(),
      "data-state": state,
      onclick: move |_| {
        if let Some(handler) = on_checked_change {
          handler.call(!checked);
        }
      },
      ..attributes,
      span {
        class: thumb_class,
        "data-state": state,
      }
    }
  }
}
