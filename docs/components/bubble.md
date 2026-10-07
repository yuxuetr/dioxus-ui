# Bubble

Bubble provides provider-neutral message surfaces and reaction placement for
chat or activity UIs.

## Source Copy

```bash
dxui add bubble
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["bubble"] }
```

## API Surface

- `Bubble`
- `BubbleGroup`
- `BubbleContent`
- `BubbleReactions`
- `BubbleVariant`
- `BubbleAlign`
- `BubbleReactionSide`
- `BubbleReactionAlign`
- class helpers for every part

## Accessibility Notes

Bubble variants are visual only. Destructive, warning, or status meaning must
also be visible in text or surrounding context. Reaction buttons or icon-only
content need accessible names from the app.

## Ownership Boundaries

Bubble owns only the framed content surface, alignment, variant styling, and
reaction placement. Avatar, sender name, timestamp, delivery state, row-level
actions, markdown parsing, code rendering, and rich content sanitization stay
app-owned or belong to higher-level message composition.
