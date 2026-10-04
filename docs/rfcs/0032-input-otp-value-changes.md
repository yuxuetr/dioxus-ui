# RFC 0032: Input OTP Value Changes

- Status: Accepted
- Created: 2026-10-04

## Summary

Make Input OTP typeable. `InputOtpHiddenInput` takes the code `length`,
calls a new `on_value_change` with the cleaned code, and covers the slots so
a press anywhere on them focuses it. `InputOtp` and `InputOtpHiddenInput`
pass through global and element attributes.

## Current State

As of M156:

- `InputOtpHiddenInput` renders a native text input with `sr-only`, the
  value, `inputmode`, and `autocomplete="one-time-code"`, but no event
  handler. Typing into it never reaches the app.
- `sr-only` hides the input from pointer users. A press on the slots focuses
  nothing, so only Tab reaches it.
- The pure helpers (`otp_insert_char_filtered`, `otp_delete_char`,
  `otp_apply_paste_filtered`) expect the app to route key and paste events
  per slot, which the component gives it no way to do.
- Neither part accepts `id` or `aria-*`, so a `Label` cannot name the input
  or the group.
- A probe in the Web preview showed that filtering only in the Rust handler
  is not enough. After typing `12a` into a numeric input, the app holds
  `12` but the native input keeps `12a`: the app value did not change, so
  Dioxus patches nothing. The next Backspace then removes the invisible `a`
  instead of the `2`. Dioxus 0.7 has no `beforeinput` event to stop the
  character earlier.

## Decision

### API

```rust
let mut code = use_signal(String::new);

rsx! {
  Label { id: "code-label", "Verification code" }
  InputOtp {
    InputOtpGroup {
      for slot in otp_slots(&code(), 6, code().chars().count().min(5)) {
        InputOtpSlot { key: "{slot.index}", index: slot.index, value: slot.value, active: slot.active }
      }
    }
    InputOtpHiddenInput {
      "aria-labelledby": "code-label",
      value: code(),
      length: 6,
      on_value_change: move |value| code.set(value),
    }
  }
}
```

- `length: usize` is a new required prop. The component cannot clamp a code
  without it, and a default would silently cut longer codes.
- `on_value_change: Option<EventHandler<String>>` receives the cleaned code
  on each `input` event, and only when it differs from `value`.
- `input_otp_sanitize(value, length, input_mode)` keeps the characters
  `InputOtpInputMode::allows` accepts, up to `length`. Numeric keeps ASCII
  digits. Text keeps letters and digits. Separators in a pasted
  `123-456` are dropped.
- The input stays controlled: the app passes the code back as `value`, and
  the slots mirror it.
- `InputOtp` extends `GlobalAttributes` and `div`. `InputOtpHiddenInput`
  extends `GlobalAttributes` and `input`. The spread comes after the explicit
  attributes.
- A disabled input fires no `input` event.

### Editing Model

The native input holds the whole code, so typing appends, Backspace removes
the last character, and a paste or platform autofill inserts the code. The
app shows the active slot at the code length. Editing a slot in the middle is
not supported.

### Layout

`INPUT_OTP_HIDDEN_INPUT_BASE_CLASS` changes from `sr-only` to an overlay:
`absolute inset-0 h-full w-full cursor-text opacity-0`. `INPUT_OTP_BASE_CLASS`
gains `relative`. The input must be a child of `InputOtp`. It stays the one
real control for assistive technology.

### Native Value Filter

A page script runs for the input's lifetime. Its `input` listener sits on the
input itself, so it runs before the delegated Dioxus handler. It applies the
same rule as `input_otp_sanitize`, reading `inputmode` and `data-length`
from the input, and rewrites the native value when they differ. Rust still
cleans the value it reads, so the app gets a valid code even without the
script.

## Scope

In scope:

- the callback, the sanitize helper, the overlay classes, the filter script,
  and attribute spreading in the crate and the template
- the Input OTP docs page
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Editing or selecting a slot in the middle | The overlay input keeps its caret at the end; per-slot editing needs selection tracking | A consumer needs to fix one digit without retyping |
| A focus-aware active slot | The app picks `active`; showing it only while focused needs focus events or a `group-focus-within` style | A consumer reports the ring showing while unfocused |
| An `on_complete` callback | `otp_is_complete` on the received value covers it | A consumer needs submit-on-complete in more than one place |
| Custom character policies | Numeric and Text cover digits and alphanumeric codes | A consumer needs another alphabet |
| Desktop and Mobile self-test scenarios | The script uses a plain `input` listener, and the Slider and roving scripts already run in the WebViews | A WebView reports a different listener order |

## Verification

- Unit tests cover `input_otp_sanitize` for both modes and length zero.
- The CLI parity test keeps the template filter script identical to the
  crate script, and the generated fixture smoke keeps the template
  compiling.
- The Web preview renders a labelled six-digit Input OTP, and
  `npm run verify:runtime-interactions` asserts:
  - a press on the slots focuses the input;
  - typed digits reach app state and the slots;
  - a rejected letter leaves the native value unchanged, so Backspace removes
    the last digit;
  - inserted `123-4567` becomes `123456`, and a full code takes no more
    characters;
  - passed attributes render and the `Label` names the input.
- Reverse checks: removing the callback, not spreading the attributes, not
  starting the filter script, or a filter script that does not cut at the
  length each make the verifier fail.
