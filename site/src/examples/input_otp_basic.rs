use dioxus::prelude::*;
use dioxus_shadcn::{InputOtp, InputOtpGroup, InputOtpHiddenInput, InputOtpSeparator, InputOtpSlot, Label, otp_slots};

#[component]
pub fn Demo() -> Element {
  let mut code = use_signal(|| "12".to_string());
  let filled = code().chars().count();
  let slots = otp_slots(&code(), 6, filled.min(5));

  rsx! {
    Label { id: "input-otp-basic-label", "Verification code" }
    InputOtp { class: "mt-2",
      InputOtpGroup {
        for slot in slots.iter().take(3).cloned() {
          InputOtpSlot { key: "{slot.index}", index: slot.index, value: slot.value, active: slot.active }
        }
      }
      InputOtpSeparator {}
      InputOtpGroup {
        for slot in slots.iter().skip(3).cloned() {
          InputOtpSlot { key: "{slot.index}", index: slot.index, value: slot.value, active: slot.active }
        }
      }
      InputOtpHiddenInput {
        "aria-labelledby": "input-otp-basic-label",
        value: code(),
        length: 6,
        on_value_change: move |value| code.set(value),
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Entered {filled} of 6 digits." }
  }
}
