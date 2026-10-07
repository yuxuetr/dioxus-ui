# Badge

Badge is a compact status or category indicator with static visual variants.

## Source Copy

```bash
dxui add badge
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["badge"] }
```

## API Surface

- `Badge`
- `BadgeVariant`
- `badge_class`

`BadgeVariant` has `Default`, `Secondary`, `Destructive`, and `Outline`, and
the solid status variants `Success`, `Warning`, and `Info`, which draw
`text-success-foreground` and the like on the status color
([RFC 0058](../rfcs/0058-status-variants.md)).

## Accessibility Notes

Use badges as supporting text, not as the only way to communicate important
state. Pair color-coded badges with clear wording.
