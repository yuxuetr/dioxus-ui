//! Checkbox: a controlled native checkbox with styled checked, unchecked, and mixed
//! states.

use std::cell::Cell;
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

use crate::density::{density_hit_area_class, use_density, with_density};
use crate::element_id::next_element_id;

// Sets the native `indeterminate` property, which has no HTML attribute.
// Keep in sync with `CHECKBOX_INDETERMINATE_SCRIPT` in the CLI `checkbox.rs` template.
pub(crate) const CHECKBOX_INDETERMINATE_SCRIPT: &str = r#"
const [scopeId, indeterminate] = await dioxus.recv();
const input = document.querySelector(`[data-dxui-checkbox="${scopeId}"]`);
if (input) input.indeterminate = indeterminate;
"#;

// The input draws its own box, so `appearance-none` drops the native control.
// The checked and mixed marks are a `::before` masked to the mark's shape and
// filled with `--primary-foreground`, so they follow any theme (RFC 0057).
const CHECKBOX_BASE_CLASS: &str = "inline-flex h-4 w-4 shrink-0 appearance-none items-center justify-center rounded border transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 data-[state=indeterminate]:border-primary data-[state=indeterminate]:bg-primary before:block before:size-full before:bg-primary-foreground before:opacity-0 before:[mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27black%27%20stroke-width=%272.5%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Cpath%20d=%27M3.5%208.5l3%203%206-7%27/%3E%3C/svg%3E)_center/100%_no-repeat] data-[state=checked]:before:opacity-100 data-[state=indeterminate]:before:opacity-100 data-[state=indeterminate]:before:[mask-image:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27black%27%20stroke-width=%272.5%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Cpath%20d=%27M4%208h8%27/%3E%3C/svg%3E)]";

/// Classes for the checkbox: base classes, the checked or unchecked colors, then
/// `class` merged over them.
pub fn checkbox_class(checked: bool, class: &str) -> String {
  let checked_class = if checked {
    "border-primary bg-primary text-primary-foreground"
  } else {
    "border-input bg-background text-transparent"
  };

  merge_classes(classes([Some(CHECKBOX_BASE_CLASS), Some(checked_class)]), class)
}

fn checkbox_state(checked: bool, indeterminate: bool) -> &'static str {
  if indeterminate {
    "indeterminate"
  } else if checked {
    "checked"
  } else {
    "unchecked"
  }
}

/// The state a change requests: a mixed checkbox becomes checked.
fn checkbox_requested_state(checked: bool, indeterminate: bool) -> bool {
  indeterminate || !checked
}

/// A controlled native checkbox. A change calls `on_checked_change` with the
/// requested state, `!checked`, or `true` while `indeterminate`; the app
/// passes it back as `checked`. Other attributes, such as `id` and `name`,
/// are passed to the input.
#[component]
pub fn Checkbox(
  #[props(default)] checked: bool,
  #[props(default)] indeterminate: bool,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] on_checked_change: Option<EventHandler<bool>>,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let class = checkbox_class(
    checked || indeterminate,
    &with_density(density_hit_area_class(use_density()), &class),
  );
  let scope_id = use_hook(|| format!("dxui-checkbox-{}", next_element_id()));
  // A click clears the native property before any handler runs; bumping this
  // re-runs the sync after the app's next render, in case it stays mixed.
  let mut changes = use_signal(|| 0_u32);
  let ever_mixed = use_hook(|| Rc::new(Cell::new(false)));
  let effect_scope_id = scope_id.clone();

  use_effect(use_reactive((&indeterminate,), move |(indeterminate,)| {
    changes();
    if !indeterminate && !ever_mixed.get() {
      return;
    }
    ever_mixed.set(true);
    let eval = document::eval(CHECKBOX_INDETERMINATE_SCRIPT);
    // A send error means the page already finished the script; nothing to set.
    let _ = eval.send((effect_scope_id.as_str(), indeterminate));
  }));

  rsx! {
    input {
      r#type: "checkbox",
      class,
      checked,
      disabled,
      "data-state": checkbox_state(checked, indeterminate),
      "data-dxui-checkbox": scope_id,
      onchange: move |_| {
        if indeterminate {
          changes += 1;
        }
        if let Some(handler) = on_checked_change {
          handler.call(checkbox_requested_state(checked, indeterminate));
        }
      },
      ..attributes,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::density::DensityProvider;
  use dioxus_shadcn_core::UiDensity;

  #[test]
  fn touch_gives_the_checkbox_a_hit_area_without_resizing_it() {
    fn app() -> Element {
      rsx! { DensityProvider { density: UiDensity::Touch, Checkbox { "aria-label": "Accept" } } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains("after:size-11") && html.contains("h-4 w-4"), "{html}");
  }

  #[test]
  fn a_mixed_checkbox_requests_checked() {
    assert!(checkbox_requested_state(false, true));
    assert!(checkbox_requested_state(true, true));
    assert!(checkbox_requested_state(false, false));
    assert!(!checkbox_requested_state(true, false));
    assert_eq!(checkbox_state(true, true), "indeterminate");
    assert_eq!(checkbox_state(true, false), "checked");
    assert_eq!(checkbox_state(false, false), "unchecked");
  }

  #[test]
  fn checkbox_class_reflects_checked_state() {
    let actual = checkbox_class(true, "mt-1");

    assert!(actual.contains(CHECKBOX_BASE_CLASS));
    assert!(actual.contains("border-primary bg-primary text-primary-foreground"));
    assert!(actual.ends_with("mt-1"));
  }

  #[test]
  fn checkbox_marks_follow_the_primary_foreground_token() {
    assert!(CHECKBOX_BASE_CLASS.contains("before:bg-primary-foreground"));
    assert!(CHECKBOX_BASE_CLASS.contains("data-[state=checked]:before:opacity-100"));
    assert!(!CHECKBOX_BASE_CLASS.contains("dark:"));
  }
}
