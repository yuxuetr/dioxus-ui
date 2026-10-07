# Input OTP

Input OTP provides controlled visual slots for one-time-code inputs plus a
native input part for forms, mobile keyboards, and assistive technology.

## Source Copy

```bash
dxui add input-otp
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["input-otp"] }
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
- `input_otp_sanitize`
- `input_otp_class`
- `input_otp_group_class`
- `input_otp_slot_class`
- `input_otp_slot_display`
- `input_otp_separator_class`
- `input_otp_hidden_input_class`

## Value Changes

`InputOtp` owns the code
([RFC 0077](../rfcs/0077-component-owned-state.md)). Give it the code
`length`; it starts from `default_value`, or follows `value` when the app
controls it, and `on_value_change` hears every change with the cleaned code.
Each `InputOtpSlot` shows the character at its `index`, and
`InputOtpHiddenInput` takes the typing:

```rust
rsx! {
  Label { id: "code-label", "Verification code" }
  InputOtp { length: 6, on_value_change: move |code: String| verify(&code),
    InputOtpGroup {
      for index in 0..6 {
        InputOtpSlot { key: "{index}", index }
      }
    }
    InputOtpHiddenInput { "aria-labelledby": "code-label" }
  }
}
```

- `input_otp_sanitize` keeps the characters the input mode allows, up to
  `length`. Numeric keeps ASCII digits and Text keeps letters and digits, so
  a pasted `123-456` becomes `123456`.
- Typing appends, Backspace removes the last character, and a paste or
  platform autofill inserts the code. Editing a slot in the middle is not
  supported.
- The slot the next character goes to is active, or the last slot when the
  code is full. `disabled` and `invalid` on the root reach every slot and the
  input; `disabled` on a slot disables that slot alone.
- The parts must be inside `InputOtp`; otherwise they render nothing and log
  which root they are missing.
- A page script filters the native value before Dioxus reads it, so a
  rejected character never stays in the input for Backspace to remove.
- A disabled input fires no event.

## Accessibility Notes

Input OTP should be paired with a visible `Label` and app-owned description or
error text when needed. Name the input with `aria-labelledby` or `id` and a
`Label`; both parts pass through global attributes. `InputOtpHiddenInput`
renders a native text input with `inputmode` and optional
`autocomplete="one-time-code"` so forms, screen readers, and mobile OTP
keyboards have a real control.

The input covers the slots with zero opacity, so a press anywhere on them
focuses it. Render it inside `InputOtp`, whose root is `relative`. Visual
slots are presentation mirrors of the code. Keep validation,
submission, and resend timers in the application.

## Pure Helpers

`otp_insert_char_filtered`, `otp_delete_char`, `otp_apply_paste_filtered`,
`otp_next_index`, and `otp_previous_index` remain for apps that build their
own per-slot inputs.

## Mobile Notes

Use `InputOtpHiddenInput` with `InputOtpInputMode::Numeric` for numeric codes.
The default autocomplete value is `one-time-code`; pass `None` when an app does
not want platform OTP autofill.

## Quality Gates

M32 verified Input OTP through crate tests, registry docs checks, feature
checks, generated source-copy fixture smoke, and Web/Desktop demo smoke output.
