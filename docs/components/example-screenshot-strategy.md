# Example And Screenshot Strategy

This document records the M36.2 example and screenshot strategy for expanded
shadcn-style parity.

Status: Implemented in M36.2.

## Current Example Coverage

The current examples are command-line smoke fixtures, not rendered Dioxus Web or
Desktop preview apps. They still cover the public API surface by running real
crate-mode helpers and printing stable output fragments.

Representative Web and Desktop demo states now cover:

- low-risk composition: Button Group, Input Group, Collapsible, Direction
- form-specific components: Input OTP
- message and AI-style composition: Attachment, Bubble, Message, Marker,
  Message Scroller
- data visualization: Chart line, bar, area helper state and fallback rows

Run:

```bash
scripts/example-smoke.sh
```

The script runs both demo crates and checks that those representative states are
present in the output.

## Screenshot Policy

Do not claim visual parity from the current command-line examples. Screenshots
should be added only after a rendered preview surface exists.

Minimum screenshot gate for a future rendered docs or demo app:

- Web screenshots for desktop-width and mobile-width viewports
- Desktop WebView screenshots for at least one compact and one comfortable
  density state
- examples for normal, disabled, invalid, selected, loading, empty, open, and
  overflow states where applicable
- chart screenshots with visible title, legend, SVG plot, and fallback table
- message screenshots with user, assistant, status, attachment, marker, and
  scroller states
- no screenshots based on marketing or landing pages instead of component
  previews

## Known Gaps

- `examples/web-demo` and `examples/desktop-demo` do not render components yet.
- No Playwright screenshot command exists for component previews.
- No Desktop WebView screenshot automation exists.
- Mobile verification remains checklist-based until a repeatable device or
  emulator command is chosen.

These gaps should remain visible until a rendered preview app is added.
