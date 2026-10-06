use super::element_id::next_element_id;
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

// Keep in sync with `INPUT_OTP_FILTER_SCRIPT` in the `dioxus-shadcn` crate.
// Runs for the input's lifetime. Its listener on the input itself runs before
// the delegated Dioxus handler, so it can drop rejected characters from the
// native value first. Without it the input would keep characters the app
// never saw, and Backspace would remove those instead of the last digit.
const INPUT_OTP_FILTER_SCRIPT: &str = r#"
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OtpSlotState {
  pub index: usize,
  pub value: Option<char>,
  pub active: bool,
  pub disabled: bool,
}

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

pub fn otp_slots(value: &str, length: usize, active_index: usize) -> Vec<OtpSlotState> {
  otp_slots_with_disabled(value, length, active_index, &[])
}

pub fn otp_slots_with_disabled(
  value: &str,
  length: usize,
  active_index: usize,
  disabled_indices: &[usize],
) -> Vec<OtpSlotState> {
  let chars = otp_value_chars(value, length);

  (0..length)
    .map(|index| {
      let disabled = disabled_indices.contains(&index);

      OtpSlotState {
        index,
        value: chars.get(index).copied(),
        active: index == active_index && !disabled,
        disabled,
      }
    })
    .collect()
}

pub fn otp_is_complete(value: &str, length: usize) -> bool {
  value.chars().take(length.saturating_add(1)).count() >= length
}

pub fn otp_insert_char(value: &str, index: usize, ch: char, length: usize) -> String {
  otp_insert_char_filtered(value, index, ch, length, |_| true)
}

pub fn otp_insert_char_filtered<F>(
  value: &str,
  index: usize,
  ch: char,
  length: usize,
  allow_char: F,
) -> String
where
  F: Fn(char) -> bool,
{
  if length == 0 || !allow_char(ch) {
    return otp_clamp_value(value, length);
  }

  let mut chars = otp_value_chars(value, length);
  let index = insertion_index(index, chars.len(), length);

  if index < chars.len() {
    chars[index] = ch;
  } else {
    chars.push(ch);
  }

  chars.into_iter().take(length).collect()
}

pub fn otp_delete_char(value: &str, index: usize) -> String {
  let mut chars: Vec<char> = value.chars().collect();

  if index < chars.len() {
    chars.remove(index);
  }

  chars.into_iter().collect()
}

pub fn otp_apply_paste(value: &str, index: usize, paste: &str, length: usize) -> String {
  otp_apply_paste_filtered(value, index, paste, length, |_| true)
}

pub fn otp_apply_paste_filtered<F>(
  value: &str,
  index: usize,
  paste: &str,
  length: usize,
  allow_char: F,
) -> String
where
  F: Fn(char) -> bool,
{
  if length == 0 {
    return String::new();
  }

  let paste_chars: Vec<char> = paste.chars().filter(|ch| allow_char(*ch)).collect();

  if paste_chars.is_empty() {
    return otp_clamp_value(value, length);
  }

  let mut chars = otp_value_chars(value, length);
  let start = insertion_index(index, chars.len(), length);
  let mut paste_index = 0;
  let mut current_index = start;

  while current_index < length && paste_index < paste_chars.len() {
    if current_index < chars.len() {
      chars[current_index] = paste_chars[paste_index];
    } else {
      chars.push(paste_chars[paste_index]);
    }

    paste_index += 1;
    current_index += 1;
  }

  chars.into_iter().take(length).collect()
}

pub fn otp_next_index(value: &str, index: usize, length: usize) -> usize {
  if length == 0 {
    return 0;
  }

  let value_len = value.chars().take(length).count();
  let next = if value_len >= length {
    index.saturating_add(1)
  } else {
    value_len.max(index.saturating_add(1))
  };

  next.min(length - 1)
}

pub const fn otp_previous_index(index: usize) -> usize {
  index.saturating_sub(1)
}

pub fn otp_clamp_value(value: &str, length: usize) -> String {
  value.chars().take(length).collect()
}

pub fn input_otp_class(disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(INPUT_OTP_BASE_CLASS), disabled.then_some(INPUT_OTP_DISABLED_CLASS)]), class)
}

pub fn input_otp_group_class(class: &str) -> String {
  merge_classes(classes([Some(INPUT_OTP_GROUP_BASE_CLASS)]), class)
}

pub fn input_otp_slot_class(active: bool, invalid: bool, disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(INPUT_OTP_SLOT_BASE_CLASS), Some(if invalid {
      INPUT_OTP_SLOT_INVALID_CLASS
    } else if active {
      INPUT_OTP_SLOT_ACTIVE_CLASS
    } else {
      "border-input"
    }), Some(if disabled { INPUT_OTP_SLOT_DISABLED_CLASS } else { "bg-background text-foreground" })]), class)
}

pub fn input_otp_slot_display(value: Option<char>) -> String {
  value.map(|ch| ch.to_string()).unwrap_or_default()
}

pub fn input_otp_separator_class(class: &str) -> String {
  merge_classes(classes([Some(INPUT_OTP_SEPARATOR_BASE_CLASS)]), class)
}

pub fn input_otp_hidden_input_class(class: &str) -> String {
  merge_classes(classes([Some(INPUT_OTP_HIDDEN_INPUT_BASE_CLASS)]), class)
}

fn otp_value_chars(value: &str, length: usize) -> Vec<char> {
  value.chars().take(length).collect()
}

fn insertion_index(index: usize, value_len: usize, length: usize) -> usize {
  index.min(value_len).min(length)
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
        if next != current {
          if let Some(handler) = on_value_change {
            handler.call(next);
          }
        }
      },
      ..attributes,
    }
  }
}

/// Runs the filter script for the input's lifetime and returns the value for
/// its `data-dxui-otp-input` attribute.
fn use_input_otp_filter() -> String {
  let scope_id = use_hook(|| format!("dxui-otp-{}", next_element_id()));
  let effect_scope_id = scope_id.clone();

  use_effect(move || {
    let eval = document::eval(INPUT_OTP_FILTER_SCRIPT);
    // A send error means the page already finished the script; nothing to track.
    let _ = eval.send(effect_scope_id.as_str());
  });

  scope_id
}
