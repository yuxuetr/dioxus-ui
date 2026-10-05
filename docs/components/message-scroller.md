# Message Scroller

Message Scroller provides controlled transcript viewport parts and pure scroll
intent helpers for chat, logs, and message feeds.

## Source Copy

```bash
dxui add message-scroller
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["message-scroller"] }
```

## API Surface

- `MessageScroller`
- `MessageScrollerViewport`
- `MessageScrollerContent`
- `MessageScrollerBottomAnchor`
- `MessageScrollerUnreadMarker`
- `MessageScrollerJumpButton`
- `MessageScrollerMetrics`
- `MessageScrollerEvent`
- `MessageScrollerIntent`
- pure scroll intent helpers
- class helpers for every part

## Accessibility Notes

Message Scroller does not add feed, log, list, or live-region semantics by
default. `MessageScrollerViewport` is a Tab stop (`tabindex="0"`) with a
focus ring so keyboard users can scroll it, and passes through attributes such
as `aria-label`; pass `tabindex: "-1"` when its content already has a
focusable element. Unread markers should contain visible text.
`MessageScrollerJumpButton` is a `type="button"` that takes `onclick`, where
the app scrolls and clears its unread state, and passes through other
attributes; icon-only jump controls need an `aria-label` from the app. Appends must not move focus automatically.

## Runtime Notes

The component does not measure the DOM and does not execute scroll commands.
Web runtime verification currently checks fixture-visible prerequisites with:

```bash
node scripts/runtime-web-verify.mjs
```

Desktop and Mobile scroll behavior remains deferred. Desktop WebView scroll,
resize, device scale, and focus preservation need a repeatable smoke path before
support is claimed. Mobile visual viewport, safe area, keyboard viewport,
native scroll, and reduced-motion behavior remain documentation-only until a
device or emulator command exists.

## Ownership Boundaries

Message Scroller owns static viewport composition, visible unread/jump states,
data attributes, Tailwind class maps, and pure intent math. Apps own scroll
metrics, scroll commands, async streams, virtualization, message IDs, history
loading, reduced-motion policy, and runtime adapters.
