# Countdown

Countdown shows the time left, such as until a sale ends, as `D:HH:MM:SS`
with fixed-width digits.

## Source Copy

```bash
dxui add countdown
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["countdown"] }
```

## API Surface

- `Countdown`
- `CountdownParts`
- `countdown_parts`
- `countdown_segments`
- `countdown_class`

```rust
rsx! {
  Countdown { remaining: remaining(), "aria-label": "Sale ends in" }
}
```

`remaining` is in seconds; the day count shows once it reaches a day. The app
owns the clock and passes the new value each second, for example from a
`use_future` loop; Dioxus has no timer that works on every renderer without a
platform crate. `countdown_parts` splits the seconds into days, hours,
minutes, and seconds for a layout with unit labels in the app's language.

## Accessibility Notes

Countdown has `role="timer"`, which is not a live region, so screen readers
do not announce every second; users hear the time when they reach it. Name it
with `aria-label` (see [RFC 0059](../rfcs/0059-display-components.md)).
