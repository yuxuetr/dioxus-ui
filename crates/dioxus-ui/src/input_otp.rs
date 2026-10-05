use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  OtpSlotState, otp_apply_paste, otp_apply_paste_filtered, otp_clamp_value, otp_delete_char,
  otp_insert_char, otp_insert_char_filtered, otp_is_complete, otp_next_index, otp_previous_index,
  otp_slots, otp_slots_with_disabled,
};

static NEXT_INPUT_OTP_ID: AtomicUsize = AtomicUsize::new(0);

// Keep in sync with `INPUT_OTP_FILTER_SCRIPT` in the CLI `input_otp.rs` template.
// Runs for the input's lifetime. Its listener on the input itself runs before
// the delegated Dioxus handler, so it can drop rejected characters from the
// native value first. Without it the input would keep characters the app
// never saw, and Backspace would remove those instead of the last digit.
pub(crate) const INPUT_OTP_FILTER_SCRIPT: &str = r#"
const scopeId = await dioxus.recv();
const input = document.querySelector(`[data-dxui-otp-input="${scopeId}"]`);
if (!input) return;
// Mirrors `InputOtpInputMode::allows`.
const allowed = { numeric: /[0-9]/, text: /[\p{Alphabetic}\p{N}]/u };
const onInput = () => {
  const pattern = allowed[input.inputMode] || allowed.text;
  const length = Number(input.dataset.length) || 0;
  const next = Array.from(input.value)
    .filter((ch) => pattern.test(ch))
    .slice(0, length)
    .join("");
  if (next !== input.value) input.value = next;
};
let finish;
const ended = new Promise((resolve) => {
  finish = resolve;
});
const observer = new MutationObserver(() => {
  if (!input.isConnected) finish();
});
observer.observe(document.documentElement, { subtree: true, childList: true });
input.addEventListener("input", onInput);
await ended;
observer.disconnect();
"#;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InputOtpInputMode {
  #[default]
  Numeric,
  Text,
}

pub const INPUT_OTP_BASE_CLASS: &str = "relative flex items-center gap-2";
pub const INPUT_OTP_DISABLED_CLASS: &str = "opacity-50";
pub const INPUT_OTP_GROUP_BASE_CLASS: &str = "flex items-center gap-1";
pub const INPUT_OTP_SLOT_BASE_CLASS: &str = "relative flex h-10 w-10 items-center justify-center rounded-md border text-sm font-medium transition-colors";
pub const INPUT_OTP_SLOT_ACTIVE_CLASS: &str = "border-ring ring-2 ring-ring";
pub const INPUT_OTP_SLOT_INVALID_CLASS: &str = "border-destructive ring-2 ring-destructive";
pub const INPUT_OTP_SLOT_DISABLED_CLASS: &str = "cursor-not-allowed bg-muted text-muted-foreground";
pub const INPUT_OTP_SLOT_EMPTY_CLASS: &str = "text-muted-foreground";
pub const INPUT_OTP_SEPARATOR_BASE_CLASS: &str = "flex items-center px-1 text-muted-foreground";
pub const INPUT_OTP_HIDDEN_INPUT_BASE_CLASS: &str =
  "absolute inset-0 h-full w-full cursor-text opacity-0 disabled:cursor-not-allowed";

impl InputOtpInputMode {
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Numeric => "numeric",
      Self::Text => "text",
    }
  }

  /// Numeric codes keep ASCII digits; text codes keep letters and digits.
  pub fn allows(self, ch: char) -> bool {
    match self {
      Self::Numeric => ch.is_ascii_digit(),
      Self::Text => ch.is_alphanumeric(),
    }
  }
}

/// Keeps the characters `input_mode` allows, up to `length` of them, so typed
/// or pasted text such as `"123-456"` becomes a code.
pub fn input_otp_sanitize(value: &str, length: usize, input_mode: InputOtpInputMode) -> String {
  value.chars().filter(|ch| input_mode.allows(*ch)).take(length).collect()
}

pub fn input_otp_class(disabled: bool, class: &str) -> String {
  classes([Some(INPUT_OTP_BASE_CLASS), disabled.then_some(INPUT_OTP_DISABLED_CLASS), Some(class)])
}

pub fn input_otp_group_class(class: &str) -> String {
  classes([Some(INPUT_OTP_GROUP_BASE_CLASS), Some(class)])
}

pub fn input_otp_slot_class(active: bool, invalid: bool, disabled: bool, class: &str) -> String {
  classes([
    Some(INPUT_OTP_SLOT_BASE_CLASS),
    Some(if invalid {
      INPUT_OTP_SLOT_INVALID_CLASS
    } else if active {
      INPUT_OTP_SLOT_ACTIVE_CLASS
    } else {
      "border-input"
    }),
    Some(if disabled { INPUT_OTP_SLOT_DISABLED_CLASS } else { "bg-background text-foreground" }),
    Some(class),
  ])
}

pub fn input_otp_slot_display(value: Option<char>) -> String {
  value.map(|ch| ch.to_string()).unwrap_or_default()
}

pub fn input_otp_separator_class(class: &str) -> String {
  classes([Some(INPUT_OTP_SEPARATOR_BASE_CLASS), Some(class)])
}

pub fn input_otp_hidden_input_class(class: &str) -> String {
  classes([Some(INPUT_OTP_HIDDEN_INPUT_BASE_CLASS), Some(class)])
}

#[component]
pub fn InputOtp(
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = input_otp_class(disabled, &class);

  rsx! {
    div {
      class,
      role: "group",
      "aria-disabled": disabled.to_string(),
      "aria-invalid": invalid.to_string(),
      "data-disabled": disabled.to_string(),
      "data-invalid": invalid.to_string(),
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn InputOtpGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = input_otp_group_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn InputOtpSlot(
  index: usize,
  #[props(default)] value: Option<char>,
  #[props(default)] active: bool,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
) -> Element {
  let class = input_otp_slot_class(active, invalid, disabled, &class);
  let display = input_otp_slot_display(value);

  rsx! {
    div {
      class,
      role: "presentation",
      "data-index": index.to_string(),
      "data-active": active.to_string(),
      "data-disabled": disabled.to_string(),
      "data-invalid": invalid.to_string(),
      span {
        "aria-hidden": "true",
        {display}
      }
    }
  }
}

#[component]
pub fn InputOtpSeparator(
  #[props(default)] class: String,
  #[props(default = String::from("-"))] content: String,
) -> Element {
  let class = input_otp_separator_class(&class);

  rsx! {
    div {
      class,
      role: "presentation",
      "aria-hidden": "true",
      {content}
    }
  }
}

#[component]
pub fn InputOtpHiddenInput(
  value: String,
  length: usize,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] name: Option<String>,
  #[props(default)] input_mode: InputOtpInputMode,
  #[props(default = Some(String::from("one-time-code")))] autocomplete: Option<String>,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let class = input_otp_hidden_input_class(&class);
  let inputmode = input_mode.attribute();
  let current = value.clone();
  let scope_id = use_input_otp_filter();

  rsx! {
    input {
      r#type: "text",
      class,
      value,
      name,
      inputmode,
      autocomplete,
      disabled,
      "aria-invalid": invalid.to_string(),
      "data-length": length.to_string(),
      "data-dxui-otp-input": scope_id,
      oninput: move |event: FormEvent| {
        let next = input_otp_sanitize(&event.value(), length, input_mode);
        if next != current && let Some(handler) = on_value_change {
          handler.call(next);
        }
      },
      ..attributes,
    }
  }
}

/// Runs the filter script for the input's lifetime and returns the value for
/// its `data-dxui-otp-input` attribute.
fn use_input_otp_filter() -> String {
  let scope_id =
    use_hook(|| format!("dxui-otp-{}", NEXT_INPUT_OTP_ID.fetch_add(1, Ordering::Relaxed)));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let eval = document::eval(INPUT_OTP_FILTER_SCRIPT);
    // A send error means the page already finished the script; nothing to track.
    let _ = eval.send(effect_scope_id.as_str());
  });

  scope_id
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn root_class_reflects_disabled_state() {
    let actual = input_otp_class(true, "w-full");

    assert!(actual.contains(INPUT_OTP_BASE_CLASS));
    assert!(actual.contains(INPUT_OTP_DISABLED_CLASS));
    assert!(actual.ends_with("w-full"));
  }

  #[test]
  fn slot_class_reflects_active_invalid_and_disabled_state() {
    let actual = input_otp_slot_class(true, true, true, "h-12");

    assert!(actual.contains(INPUT_OTP_SLOT_BASE_CLASS));
    // The invalid border replaces the active one; both set the border color.
    assert!(!actual.contains(INPUT_OTP_SLOT_ACTIVE_CLASS));
    assert!(actual.contains(INPUT_OTP_SLOT_INVALID_CLASS));
    assert!(actual.contains(INPUT_OTP_SLOT_DISABLED_CLASS));
    assert!(actual.ends_with("h-12"));
    assert!(input_otp_slot_class(true, false, false, "").contains(INPUT_OTP_SLOT_ACTIVE_CLASS));
    assert!(input_otp_slot_class(false, false, false, "").contains("border-input bg-background"));
  }

  #[test]
  fn input_mode_maps_to_native_attribute() {
    assert_eq!(InputOtpInputMode::Numeric.attribute(), "numeric");
    assert_eq!(InputOtpInputMode::Text.attribute(), "text");
  }

  #[test]
  fn sanitize_keeps_allowed_characters_up_to_the_length() {
    assert_eq!(input_otp_sanitize("123-4567", 6, InputOtpInputMode::Numeric), "123456");
    assert_eq!(input_otp_sanitize("12a", 6, InputOtpInputMode::Numeric), "12");
    assert_eq!(input_otp_sanitize("ab-1 c", 6, InputOtpInputMode::Text), "ab1c");
    assert_eq!(input_otp_sanitize("1234", 0, InputOtpInputMode::Numeric), "");
  }

  #[test]
  fn exposes_otp_helpers_for_component_users() {
    let value = otp_apply_paste_filtered("", 0, "1a2", 4, |ch| ch.is_ascii_digit());
    let slots = otp_slots(&value, 4, 2);

    assert_eq!(value, "12");
    assert_eq!(slots.len(), 4);
    assert!(otp_is_complete("1234", 4));
  }
}
