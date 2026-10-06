# Attachment

Attachment provides provider-neutral file preview rows, cards, and actions for
message or upload interfaces.

## Source Copy

```bash
dxui add attachment
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["attachment"] }
```

## API Surface

- `Attachment`
- `AttachmentGroup`
- `AttachmentMedia`
- `AttachmentContent`
- `AttachmentTitle`
- `AttachmentDescription`
- `AttachmentActions`
- `AttachmentAction`
- `AttachmentTrigger`
- `AttachmentState`
- `AttachmentSize`
- `AttachmentOrientation`
- `AttachmentMediaVariant`
- class helpers for every part

## Accessibility Notes

`AttachmentAction` and `AttachmentTrigger` render native buttons that take
`onclick` and pass through other attributes. Icon-only actions need an
`aria-label` from the app, such as `"Remove report.pdf"`. Error states must include visible
text in `AttachmentDescription`; color alone is not enough.

`AttachmentGroup` is visual grouping only. Apps own group labels when the group
is focusable or scrollable.

## Ownership Boundaries

Attachment does not own file upload transport, object URL lifecycle,
drag-and-drop behavior, preview loading, progress values, retry behavior, or
network state. Apps pass those states into `AttachmentState` and render the
appropriate description or actions.
