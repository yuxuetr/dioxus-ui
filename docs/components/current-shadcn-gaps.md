# Current shadcn Gap Audit

This document records the M30.1 audit against the current shadcn/ui component
catalog.

Status: Audited in M30.1. Low-risk composition APIs planned in M30.2. Input
OTP APIs planned in M30.3.

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
avatar
badge
breadcrumb
button
calendar
card
carousel
checkbox
combobox
command
context-menu
data-table
date-picker
dialog
drawer
dropdown
empty
field
hover-card
input
item
kbd
label
menubar
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
| Low-risk composition | Button Group, Input Group, Collapsible, Direction | M31 | Mostly styled composition or controlled disclosure/direction helpers. |
| Form-specific | Input OTP | M32 | Needs keyboard, paste, mobile input, and screen-reader planning. |
| Message and AI-style composition | Attachment, Bubble, Message, Marker | M33 | Keep upload, markdown, citation, streaming, and provider behavior app-owned. |
| Runtime-dependent message layout | Message Scroller | M34 | Needs scroll intent, bottom anchoring, unread markers, and browser assertions. |
| Chart | Chart | M35 | Backend direction is decided, but public component remains gated. |

## Explicit Deferrals

These behaviors remain app-owned or deferred even after surface components are
implemented:

- upload transport, drag-drop, and object URL lifecycle for Attachment
- markdown parsing, syntax highlighting, and model/provider coupling for Message
- streaming and virtualized rendering for Message Scroller
- actual DOM scroll commands until runtime browser assertions exist
- Chart external backend adapters such as Plotters or Charming
- Chart cursor exploration and animation runtime

## Implementation Order

1. Low-risk composition gaps: Button Group, Input Group, Collapsible, Direction.
2. Form-specific gap: Input OTP.
3. Static message parts: Attachment, Bubble, Message, Marker.
4. Runtime-dependent message layout: Message Scroller and Web browser assertions.
5. Chart public component preparation after measurement and fallback-table gates.

This order avoids starting with runtime-heavy scrolling, upload, or chart
rendering before the underlying contracts are proven.

For Button Group, Input Group, Collapsible, and Direction API details, see
[Low-risk Composition Gap API Plan](low-risk-composition-gaps.md).
For Input OTP API details, see [Input OTP API Plan](input-otp-plan.md).

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
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
