use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use dioxus_ui_core::classes;

static NEXT_CHECKBOX_ID: AtomicUsize = AtomicUsize::new(0);

// Sets the native `indeterminate` property, which has no HTML attribute.
// Keep in sync with `CHECKBOX_INDETERMINATE_SCRIPT` in the CLI `checkbox.rs` template.
pub(crate) const CHECKBOX_INDETERMINATE_SCRIPT: &str = r#"
const [scopeId, indeterminate] = await dioxus.recv();
const input = document.querySelector(`[data-dxui-checkbox="${scopeId}"]`);
if (input) input.indeterminate = indeterminate;
"#;

pub const CHECKBOX_BASE_CLASS: &str = "inline-flex h-4 w-4 shrink-0 items-center justify-center rounded border border-zinc-300 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:cursor-not-allowed disabled:opacity-50";

pub fn checkbox_class(checked: bool, class: &str) -> String {
  let checked_class =
    if checked { "border-blue-600 bg-blue-600 text-white" } else { "bg-white text-transparent" };

  classes([Some(CHECKBOX_BASE_CLASS), Some(checked_class), Some(class)])
}

pub fn checkbox_state(checked: bool, indeterminate: bool) -> &'static str {
  if indeterminate {
    "indeterminate"
  } else if checked {
    "checked"
  } else {
    "unchecked"
  }
}

/// The state a change requests: a mixed checkbox becomes checked.
pub fn checkbox_requested_state(checked: bool, indeterminate: bool) -> bool {
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
  let class = checkbox_class(checked || indeterminate, &class);
  let scope_id =
    use_hook(|| format!("dxui-checkbox-{}", NEXT_CHECKBOX_ID.fetch_add(1, Ordering::Relaxed)));
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
    assert!(actual.contains("border-blue-600 bg-blue-600 text-white"));
    assert!(actual.ends_with("mt-1"));
  }
}
