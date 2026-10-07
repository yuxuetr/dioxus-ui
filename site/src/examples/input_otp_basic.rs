use dioxus::prelude::*;
use dioxus_shadcn::{InputOtp, InputOtpGroup, InputOtpHiddenInput, InputOtpSeparator, InputOtpSlot, Label};

#[component]
pub fn InputOtpBasicDemo() -> Element {
  let mut code = use_signal(|| "12".to_string());
  let filled = code().chars().count();

  rsx! {
    Label { id: "input-otp-basic-label", "Verification code" }
    InputOtp { class: "mt-2", length: 6, value: code(), on_value_change: move |value| code.set(value),
      InputOtpGroup {
        for index in 0..3 {
          InputOtpSlot { key: "{index}", index }
        }
      }
      InputOtpSeparator {}
      InputOtpGroup {
        for index in 3..6 {
          InputOtpSlot { key: "{index}", index }
        }
      }
      InputOtpHiddenInput { "aria-labelledby": "input-otp-basic-label" }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Entered {filled} of 6 digits." }
  }
}
