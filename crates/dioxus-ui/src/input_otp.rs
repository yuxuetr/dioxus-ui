use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  OtpSlotState, otp_apply_paste, otp_apply_paste_filtered, otp_clamp_value, otp_delete_char,
  otp_insert_char, otp_insert_char_filtered, otp_is_complete, otp_next_index, otp_previous_index,
  otp_slots, otp_slots_with_disabled,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InputOtpInputMode {
  #[default]
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

pub fn input_otp_class(disabled: bool, class: &str) -> String {
  classes([Some(INPUT_OTP_BASE_CLASS), disabled.then_some(INPUT_OTP_DISABLED_CLASS), Some(class)])
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
  #[props(default)] input_mode: InputOtpInputMode,
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
    assert!(actual.contains(INPUT_OTP_SLOT_ACTIVE_CLASS));
    assert!(actual.contains(INPUT_OTP_SLOT_INVALID_CLASS));
    assert!(actual.contains(INPUT_OTP_SLOT_DISABLED_CLASS));
    assert!(actual.ends_with("h-12"));
  }

  #[test]
  fn input_mode_maps_to_native_attribute() {
    assert_eq!(InputOtpInputMode::Numeric.attribute(), "numeric");
    assert_eq!(InputOtpInputMode::Text.attribute(), "text");
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
