# Input OTP API Plan

This document defines the M30.3 API plan for the Input OTP component.

Status: Planned in M30.3. Implemented in M32.

## Decision

Input OTP should be implemented as a controlled form component with small pure
helpers. It should not own verification, submission, resend flows, timers, or
authentication state.

M32 should split the work:

1. pure helper functions for slot indexing, paste distribution, deletion, and
   completion checks
2. controlled styled components for root, group, slot, separator, and hidden or
   native input strategy
3. docs and quality gates for generated source, accessibility, and mobile input

## Ownership Boundaries

Input OTP owns:

- visual slot layout
- invalid and disabled styling
- optional separator layout
- helper functions for deterministic slot updates
- ARIA and native input guidance

The consuming app owns:

- OTP value storage
- validation and submission
- resend cooldowns and timers
- server verification
- masking policy
- localized labels and error messages
- paste permission policy
- analytics and abuse protection

## Primitive Helper Plan

Planned helper API:

```rust
OtpSlotState {
  index: usize,
  value: Option<char>,
  active: bool,
  disabled: bool,
}

otp_slots(value: &str, length: usize, active_index: usize) -> Vec<OtpSlotState>
otp_is_complete(value: &str, length: usize) -> bool
otp_insert_char(value: &str, index: usize, ch: char, length: usize) -> String
otp_delete_char(value: &str, index: usize) -> String
otp_apply_paste(value: &str, index: usize, paste: &str, length: usize) -> String
otp_next_index(value: &str, index: usize, length: usize) -> usize
otp_previous_index(index: usize) -> usize
```

Implemented helper API also includes:

```rust
otp_slots_with_disabled(...)
otp_insert_char_filtered(...)
otp_apply_paste_filtered(...)
otp_clamp_value(...)
```

Rules:

- helpers are pure and renderer-independent
- helpers do not store state
- invalid characters should be filtered by app-provided policy or a simple
  allow-list parameter if implementation needs it
- helper tests must cover paste overflow, empty paste, deletion boundaries,
  disabled slots if supported, and completion behavior

## Component API

Crate feature:

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["input-otp"] }
```

Source-copy command:

```bash
dxui add input-otp
```

Planned crate API:

```rust
InputOtp {
  invalid: bool,
  disabled: bool,
  class: String,
  children: Element,
}

InputOtpGroup {
  class: String,
  children: Element,
}

InputOtpSlot {
  index: usize,
  value: Option<char>,
  active: bool,
  invalid: bool,
  disabled: bool,
  class: String,
}

InputOtpSeparator {
  class: String,
  content: String,
}

InputOtpHiddenInput {
  value: String,
  name: Option<String>,
  input_mode: InputOtpInputMode,
  autocomplete: Option<String>,
  disabled: bool,
  invalid: bool,
  class: String,
}

InputOtpInputMode::{Numeric, Text}
input_otp_class(...)
input_otp_group_class(...)
input_otp_slot_class(...)
input_otp_separator_class(...)
input_otp_hidden_input_class(...)
```

The implementation preserves:

- controlled `value` on `InputOtpHiddenInput`
- explicit `length` through `otp_slots(value, length, active_index)`
- slot composition
- invalid and disabled visual states
- native input compatibility for mobile keyboards and forms

## Implementation Result

M32 implemented Input OTP in two steps:

- M32.1 added renderer-independent primitive helpers with tests for slot state,
  insertion, deletion, filtering, paste overflow, disabled slots, and completion.
- M32.2 added the styled component feature, source-copy template, registry
  entry, component docs page, catalog row, feature-check coverage, and
  Web/Desktop smoke output.

Validation, submission, resend timers, paste permission, and keyboard event
wiring remain app-owned as planned.

## Keyboard Behavior

Expected behavior to document and test through helpers:

- typing a valid character fills the active slot and advances
- Backspace clears the active slot or moves backward when appropriate
- Delete clears the active slot
- ArrowLeft and ArrowRight can move active index if the consuming app wires
  keyboard handlers
- Home and End can move to first and last slots if app wiring supports them
- paste distributes characters from the active index without exceeding length

The first component should expose state and markup that make these behaviors
possible, but the actual Dioxus event handlers can remain app-owned in the
source-copy template if that keeps the component simple and controlled.

## Accessibility

Input OTP must not be only a row of visual boxes. It needs a native input or a
clear app-owned labeling strategy.

Requirements:

- pair with `Label` or equivalent visible label
- support `aria-invalid` on the actual form control when invalid
- support `aria-describedby` from the consuming app
- keep grouped slots visually synchronized with native input value
- ensure separators are decorative unless the app gives them meaning
- avoid announcing each slot as a separate unlabeled control unless there is no
  hidden/native input strategy

Recommended default:

- one native input for form and assistive technology behavior
- visual slots rendered as presentation or status mirrors
- app-owned event handlers update the controlled value

## Mobile Considerations

Mobile behavior should be conservative:

- prefer `inputmode="numeric"` for numeric OTP values
- allow `autocomplete="one-time-code"` when apps want platform OTP autofill
- keep touch targets large enough for slot focus affordances
- avoid hover-only focus indicators
- keep paste and SMS autofill behavior app-owned

## Tailwind Constraints

- all classes must be complete static Tailwind tokens
- invalid, disabled, active, and filled states should use match-based class maps
- do not generate width, gap, ring, or color class fragments dynamically
- templates must remain Tailwind v4 compatible and not depend on runtime class
  generation

## Source-copy Policy

Generated Input OTP source must remain self-contained:

- no imports from `dioxus-shadcn-core`
- no imports from `dioxus-shadcn-primitives`
- no renderer runtime adapters
- no authentication or timer dependencies

If helper functions are needed in generated source, copy them into
`input_otp.rs` with focused tests in crate mode.

## Quality Gates

M32 implementation should run:

```bash
cargo test --workspace --all-features --quiet
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
git diff --check
```

Before marking Input OTP complete, update:

- `registry/input-otp.json`
- `templates/input_otp.rs`
- `crates/dioxus-shadcn/src/input_otp.rs`
- component docs page
- component catalog
- parity matrix
- accessibility checklist
