use dioxus::prelude::*;
use super::utils::classes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OtpSlotState {
  pub index: usize,
  pub value: Option<char>,
  pub active: bool,
  pub disabled: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputOtpInputMode {
  Numeric,
  Text,
}

pub const INPUT_OTP_BASE_CLASS: &str = "flex items-center gap-2";
pub const INPUT_OTP_DISABLED_CLASS: &str = "opacity-50";
pub const INPUT_OTP_GROUP_BASE_CLASS: &str = "flex items-center gap-1";
pub const INPUT_OTP_SLOT_BASE_CLASS: &str = "relative flex h-10 w-10 items-center justify-center rounded-md border border-zinc-200 bg-white text-sm font-medium text-zinc-950 transition-colors";
pub const INPUT_OTP_SLOT_ACTIVE_CLASS: &str = "border-blue-600 ring-2 ring-blue-600";
pub const INPUT_OTP_SLOT_INVALID_CLASS: &str = "border-red-500 ring-2 ring-red-500";
pub const INPUT_OTP_SLOT_DISABLED_CLASS: &str = "cursor-not-allowed bg-zinc-50 text-zinc-400";
pub const INPUT_OTP_SLOT_EMPTY_CLASS: &str = "text-zinc-400";
pub const INPUT_OTP_SEPARATOR_BASE_CLASS: &str = "flex items-center px-1 text-zinc-400";
pub const INPUT_OTP_HIDDEN_INPUT_BASE_CLASS: &str = "sr-only";

impl InputOtpInputMode {
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Numeric => "numeric",
      Self::Text => "text",
    }
  }
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
  classes([
    Some(INPUT_OTP_BASE_CLASS),
    disabled.then_some(INPUT_OTP_DISABLED_CLASS),
    Some(class),
  ])
}

pub fn input_otp_group_class(class: &str) -> String {
  classes([Some(INPUT_OTP_GROUP_BASE_CLASS), Some(class)])
}

pub fn input_otp_slot_class(active: bool, invalid: bool, disabled: bool, class: &str) -> String {
  classes([
    Some(INPUT_OTP_SLOT_BASE_CLASS),
    active.then_some(INPUT_OTP_SLOT_ACTIVE_CLASS),
    invalid.then_some(INPUT_OTP_SLOT_INVALID_CLASS),
    disabled.then_some(INPUT_OTP_SLOT_DISABLED_CLASS),
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
  #[props(default)] name: Option<String>,
  #[props(default = InputOtpInputMode::Numeric)] input_mode: InputOtpInputMode,
  #[props(default = Some(String::from("one-time-code")))] autocomplete: Option<String>,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default)] class: String,
) -> Element {
  let class = input_otp_hidden_input_class(&class);
  let inputmode = input_mode.attribute();

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
    }
  }
}
