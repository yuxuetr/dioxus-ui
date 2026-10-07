# Message

Message provides provider-neutral chat row layout for avatars, metadata,
content, and footer actions.

## Source Copy

```bash
dxui add message
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["message"] }
```

## API Surface

- `Message`
- `MessageGroup`
- `MessageAvatar`
- `MessageContent`
- `MessageHeader`
- `MessageFooter`
- `MessageAlign`
- class helpers for every part

## Accessibility Notes

Message does not add list, feed, article, log, or live-region semantics. Apps
own those roles when a transcript needs them. Footer icon-only actions need
accessible names from the app.

## Ownership Boundaries

Message owns row layout, alignment, avatar placement, header placement, content
placement, and footer placement. Sender identity, assistant/user/tool roles,
timestamps, delivery state, markdown parsing, syntax highlighting, streaming,
and provider integration stay app-owned.
