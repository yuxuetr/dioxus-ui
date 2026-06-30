# Final shadcn Parity Audit

This document records the M36.3 final audit after M31-M35 filled the expanded
shadcn-style component gaps.

Status: Audited in M36.3.

## Source

The upstream shadcn/ui Components page was rechecked on 2026-06-30:

<https://ui.shadcn.com/docs/components>

The current upstream catalog includes the earlier component set plus the newer
Attachment, Bubble, Marker, Message, and Message Scroller entries.

## Result

No new public upstream component gaps were found during M36.3.

The local registry exposes 64 public component entries. `schema` and `utils`
remain support entries and are excluded from component parity.

Implemented expanded-gap entries:

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

## Remaining Differences

The remaining differences are behavior-depth and verification gaps, not missing
component files:

- visual parity is not proven until a rendered Web/Desktop preview app and
  screenshot gate exist
- Desktop and Mobile runtime behavior remains checklist or fixture based
- actual DOM/WebView scroll commands for Message Scroller remain runtime-owned
- Attachment upload transport, drag-drop, and object URL lifecycle remain
  app-owned
- Message markdown parsing, syntax highlighting, citation resolution, streaming,
  and provider integration remain app-owned
- Chart external backends, cursor exploration, hit testing, animation runtime,
  dense-data rendering, and non-line/bar/area chart families remain deferred

## Next Work

No new component-fill milestone is required from this upstream audit.

The next useful milestone should focus on rendered preview and screenshot
verification:

- build a real Dioxus Web preview surface for component states
- add a Desktop WebView preview smoke path
- add desktop-width and mobile-width screenshot assertions
- keep source-copy registry and crate-mode examples in sync with the previews
