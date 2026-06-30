# Current shadcn Gap Audit

This document records the M30.1 audit against the current shadcn/ui component
catalog.

Status: Audited in M30.1. Low-risk composition APIs planned in M30.2. Input
OTP APIs planned in M30.3. Message and AI-style APIs planned in M30.4. Button
Group implemented in M31.1. Input Group implemented in M31.2. Collapsible
implemented in M31.3. Direction implemented in M31.4. Input OTP implemented in
M32. Attachment, Bubble, Message, and Marker implemented in M33. Message
Scroller implemented in M34. Chart implemented in M35. Final upstream parity
rechecked in M36.3 with no new public component gaps found.

## Source

The upstream shadcn/ui Components page lists these current gaps for `dioxus-ui`:

- Attachment
- Bubble
- Button Group
- Chart
- Collapsible
- Direction
- Input Group
- Input OTP
- Marker
- Message
- Message Scroller

The shadcn page marks Attachment, Bubble, Marker, Message, and Message Scroller
as new components.

Source: <https://ui.shadcn.com/docs/components>

## Local Baseline

The local registry currently has public entries for:

```text
accordion
alert
alert-dialog
aspect-ratio
attachment
avatar
badge
breadcrumb
bubble
button
button-group
calendar
card
carousel
chart
checkbox
collapsible
combobox
command
context-menu
data-table
date-picker
dialog
direction
drawer
dropdown
empty
field
hover-card
input
input-group
input-otp
item
kbd
label
marker
menubar
message
message-scroller
native-select
navigation-menu
pagination
popover
progress
radio-group
resizable
scroll-area
select
separator
sheet
sidebar
skeleton
slider
sonner
spinner
switch
table
tabs
textarea
toast
toggle
toggle-group
tooltip
typography
```

`utils` exists as a support template and is excluded from public component
parity.

## Gap Classification

| Group | Missing Components | Priority | Notes |
| --- | --- | --- | --- |
| Low-risk composition | - | M31 | Button Group, Input Group, Collapsible, and Direction are implemented. |
| Form-specific | - | M32 | Input OTP is implemented; validation, submission, resend timers, and paste policy remain app-owned. |
| Message and AI-style composition | - | M33 | Attachment, Bubble, Message, and Marker are implemented; upload, markdown, citation, streaming, and provider behavior remain app-owned. |
| Runtime-dependent message layout | - | M34 | Message Scroller is implemented; actual scroll commands, browser automation, and Desktop/Mobile behavior remain runtime-gated. |
| Chart | - | M35 | Chart is implemented as first-party SVG composition for line, bar, and area charts. |

## Explicit Deferrals

These behaviors remain app-owned or deferred even after surface components are
implemented:

- upload transport, drag-drop, and object URL lifecycle for Attachment
- markdown parsing, syntax highlighting, and model/provider coupling for Message
- streaming, virtualized rendering, and history prepend behavior for Message Scroller
- actual DOM/WebView scroll commands until runtime browser assertions exist
- Chart external backend adapters such as Plotters or Charming
- Chart cursor exploration and animation runtime
- Chart pie, radial, radar, heatmap, candlestick, composed, dense-data, and synchronized cursor variants

## Implementation Order

1. Completed low-risk composition gaps: Button Group, Input Group,
   Collapsible, Direction.
2. Completed form-specific gap: Input OTP.
3. Completed static message parts: Attachment, Bubble, Message, Marker.
4. Completed runtime-dependent message layout surface: Message Scroller and Web assertion prerequisites.
5. Completed Chart public component preparation: M35.1 defined the public SVG
   API plan, M35.2 validated the example fixture, and M35.3 implemented the
   public source-copy and crate-mode surface.

This order avoids starting with runtime-heavy scrolling, upload, or chart
rendering before the underlying contracts are proven.

For Button Group, Input Group, Collapsible, and Direction API details, see
[Low-risk Composition Gap API Plan](low-risk-composition-gaps.md).
For Input OTP API details, see [Input OTP API Plan](input-otp-plan.md).
For Attachment, Bubble, Message, Marker, and Message Scroller API details, see
[Message and AI-style API Plan](message-ai-plan.md).
For Chart API preparation, see [Chart Public API Plan](chart-public-api-plan.md).
For the public Chart component, see [Chart](chart.md).
For the final M36 upstream audit, see
[Final shadcn Parity Audit](final-shadcn-parity-audit.md).

## Quality Gates

Each public component milestone must update:

- crate module and feature
- source-copy template
- registry entry
- component docs page
- component catalog link
- parity matrix
- generated fixture smoke coverage

Before marking implementation tasks done, run:

```bash
cargo test --workspace --all-features --quiet
cargo test -q -p dioxus-ui-cli --test registry
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
