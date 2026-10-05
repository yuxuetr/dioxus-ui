# Steps

Steps shows where the user is in a process, such as checkout or onboarding,
as numbered steps joined by a line.

## Source Copy

```bash
dxui add steps
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["steps"] }
```

## API Surface

- `Steps`
- `StepsOrientation`
- `Step`
- `StepStatus`
- `steps_class`
- `step_class`
- `step_track_class`
- `step_indicator_class`
- `step_label_class`

```rust
rsx! {
  Steps {
    Step { status: StepStatus::Complete, "Cart" }
    Step { status: StepStatus::Current, "Shipping" }
    Step { "Payment" }
  }
}
```

`StepStatus` is `Complete` (primary circle), `Current` (primary ring), or
`Upcoming` (muted, the default); the line is primary up to the current step.
The list numbers the steps with a CSS counter. `StepsOrientation::Horizontal`
is the default; `Vertical` stacks the steps with the labels beside the
circles. The app owns which step is current.

## Accessibility Notes

`Steps` is an `ol`, so assistive technology reads the step count and order.
The current step sets `aria-current="step"`, a complete step adds visually
hidden "(completed)" text, and the numbered circles and line are
`aria-hidden`. Steps only shows progress; make each step's content reachable
through the app's own navigation (see
[RFC 0059](../rfcs/0059-display-components.md)).
