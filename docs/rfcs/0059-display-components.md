# RFC 0059: Display Components

- Status: Accepted
- Created: 2026-10-05

## Summary

Add the daisyUI display components that have no counterpart here: Stat,
Timeline, Steps, Indicator, Status, Radial Progress, Countdown, and Diff.
Each follows the existing conventions: one module and Cargo feature, a
source-copy template, controlled state, tokens only, and native semantics
before ARIA.

## Current State

daisyUI 5 lists these under data display, navigation, feedback, and layout;
shadcn/ui has none of them. Apps build dashboards (Stat), activity feeds
(Timeline), checkout and onboarding flows (Steps), notification counts
(Indicator), presence dots (Status), circular gauges (Radial Progress), sale
timers (Countdown), and before-and-after images (Diff) from raw markup.

## Decision

### Stat

```rust
StatGroup {
  Stat {
    StatTitle { "Revenue" }
    StatValue { "$45,231" }
    StatDescription { "+20% from last month" }
    StatFigure { Icon {} }
  }
}
```

`StatGroup` is a `dl`, `Stat` a `div` inside it, `StatTitle` a `dt`, and
`StatValue`, `StatDescription`, and `StatFigure` are `dd`s, so assistive
technology pairs each value with its title. `Stat` must sit in a `StatGroup`
for the `dt` to be valid. A grid puts the figure in a second column spanning
the rows, so source order stays title first. `StatGroupOrientation` is
`Horizontal` (dividers between columns, horizontal scroll when narrow) or
`Vertical`.

### Timeline

`Timeline` is an `ol`, and `TimelineItem` an `li` with three parts:
`TimelineTime` (a `time` element; pass `datetime`), `TimelineMarker` (the dot
or icon, `aria-hidden`), and `TimelineContent`. A connector line joins the
markers, drawn by the item, not the app. `TimelineOrientation` is `Vertical`
(default; time beside the marker) or `Horizontal` (time above, content
below).

### Steps

`Steps` is an `ol` that numbers its `Step`s with a CSS counter. `Step` takes
`status: StepStatus`: `Complete` (primary fill), `Current` (primary ring and
`aria-current="step"`), or `Upcoming` (muted). A complete step adds
visually hidden "completed" text, since color alone cannot say it.
`StepsOrientation` is `Horizontal` (default) or `Vertical`.

### Indicator and Status

`Indicator` wraps an element in `relative inline-flex`, and `IndicatorItem`
places its child on a corner with `IndicatorPlacement`: `TopEnd` (default),
`TopStart`, `BottomEnd`, or `BottomStart`, using logical properties so
right-to-left layouts mirror.

`Status` is a dot with `StatusVariant`: `Neutral` (default), `Primary`,
`Success`, `Warning`, `Info`, or `Destructive`, and `StatusSize`. With
`label` it renders `role="img"` and `aria-label`; without it, it is
`aria-hidden`, for a dot next to text that already says the state.

### Radial Progress

`RadialProgress` is an SVG ring with `role="progressbar"`,
`aria-valuemin="0"`, `aria-valuemax="100"`, and `aria-valuenow`. `value` is
clamped to 0 through 100. Children replace the centered `{value}%` label.
`RadialProgressSize` sets the diameter and stroke.

### Countdown

`Countdown` renders `remaining` seconds as days, hours, minutes, and seconds
blocks with two-digit, tabular numbers, and `role="timer"`, which is not a
live region, so screen readers do not announce every second. The app owns
the clock and passes the new value; Dioxus has no timer that works on every
renderer without a platform crate. Days show only when `remaining` reaches a
day. `aria-label` names the timer.

### Diff

`Diff` overlays `DiffBefore` and `DiffAfter` and clips the after layer at
`position` percent (default 50) with `clip-path`. A native range input
covers the frame, transparent except for a divider handle, so dragging,
clicking, and the arrow keys all move it without custom pointer code; the
input calls `on_position_change`, and its `aria-label` defaults to
"Comparison position".

## Alternatives

- **Divs for Stat.** daisyUI uses divs; a definition list ties each value to
  its title for screen readers at no cost.
- **A Countdown that runs its own timer.** It would need `gloo-timers` on Web
  and `tokio` elsewhere, a dependency per renderer for a one-line app loop.
- **Pointer events for Diff.** A range input gives keyboard and assistive
  support for free, and the same code runs on every renderer.

## Verification

- SSR tests for each component's semantics and classes.
- A site example per component, audited in both themes and every preset.
- A runtime check that the Diff range input moves the clip with the arrow
  keys and calls `on_position_change`.
