# RFC 0070: Range Slider

- Status: Accepted
- Created: 2026-10-06

## Summary

Add `RangeSlider`, a slider with two thumbs for a low and a high value, next
to `Slider` and without changing it.

## Current State

The release notes exclude multi-thumb sliders
([RFC 0030](0030-slider-keyboard-and-pointer-input.md),
[RFC 0042](0042-slider-thumb-position-and-vertical-orientation.md)). Price
and date filters need a range; two Sliders cannot keep their values ordered.

## Decision

- `value: (f64, f64)` is controlled; `on_value_change` receives the new
  pair. `min_steps_between` keeps the thumbs that many steps apart (0 lets
  them meet).
- Following the WAI-ARIA multi-thumb slider pattern, the root is a `group`
  (named by the app's `aria-label`) and each thumb is a `slider` with its own
  focus. A thumb's `aria-valuemin` or `aria-valuemax` is the other thumb's
  value plus or minus the gap, so assistive technology reports how far it
  can go. `start_label` and `end_label` name the thumbs, "Minimum" and
  "Maximum" by default ([RFC 0035](0035-control-label-overrides.md)).
- Arrow, Page Up, Page Down, Home, and End keys move the focused thumb as
  on `Slider`, and stop at the other thumb's bound.
- A press moves the nearer thumb, or the upper one when the press is above
  both, focuses it, and captures the pointer, so a drag keeps moving it and
  stops at the gap.
- `range_slider_values` holds the rule, snapping and clamping one thumb
  against the other, and is public for apps that move thumbs themselves.

Right-to-left horizontal range sliders are not included, as for `Slider`.

## Alternatives

- **A `Vec<f64>` value on `Slider`.** Changing `Slider`'s value type would
  break every app, and more than two thumbs is rare.

## Verification

- Unit tests for snapping, bounds, the gap, meeting thumbs, and a reversed
  input pair; an SSR test for each thumb's bounds and the range fill.
- A runtime check for keys on each thumb, presses moving the nearer thumb,
  and a drag stopping at the gap; reverse-verified with the gap ignored.
