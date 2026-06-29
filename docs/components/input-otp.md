# Input OTP

Input OTP provides controlled visual slots for one-time-code inputs plus a
native input part for forms, mobile keyboards, and assistive technology.

## Source Copy

```bash
dxui add input-otp
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["input-otp"] }
```

## API Surface

- `InputOtp`
- `InputOtpGroup`
- `InputOtpSlot`
- `InputOtpSeparator`
- `InputOtpHiddenInput`
- `InputOtpInputMode`
- `OtpSlotState`
- `otp_slots`
- `otp_slots_with_disabled`
- `otp_is_complete`
- `otp_insert_char`
- `otp_insert_char_filtered`
- `otp_delete_char`
- `otp_apply_paste`
- `otp_apply_paste_filtered`
- `otp_next_index`
- `otp_previous_index`
- `otp_clamp_value`
- `input_otp_class`
- `input_otp_group_class`
- `input_otp_slot_class`
- `input_otp_slot_display`
- `input_otp_separator_class`
- `input_otp_hidden_input_class`

## Accessibility Notes

Input OTP should be paired with a visible `Label` and app-owned description or
error text when needed. `InputOtpHiddenInput` renders a native text input with
`inputmode` and optional `autocomplete="one-time-code"` so forms, screen
readers, and mobile OTP keyboards have a real control.

Visual slots are presentation mirrors of the controlled value. Keep validation,
submission, resend timers, paste policy, and keyboard event handlers in the
application.
