# Message Scroller

Message Scroller provides controlled transcript viewport parts and pure scroll
intent helpers for chat, logs, and message feeds.

## Source Copy

```bash
dxui add message-scroller
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["message-scroller"] }
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
default. Unread markers should contain visible text. Icon-only jump controls
need accessible labels from the app. Appends must not move focus automatically.

## Ownership Boundaries

Message Scroller owns static viewport composition, visible unread/jump states,
data attributes, Tailwind class maps, and pure intent math. Apps own scroll
metrics, scroll commands, async streams, virtualization, message IDs, history
loading, reduced-motion policy, and runtime adapters.
