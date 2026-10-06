# Alert

Alert presents important status or validation messages with title and
description parts.

## Source Copy

```bash
dxui add alert
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.3", default-features = false, features = ["alert"] }
```

## API Surface

- `Alert`
- `AlertTitle`
- `AlertDescription`
- `AlertVariant`
- `alert_class`
- `alert_title_class`
- `alert_description_class`

`AlertVariant` has `Default`, `Destructive`, and the status variants
`Success`, `Warning`, and `Info`. The status colors are too light to be text,
so status alerts tint the surface and border and keep foreground text; pass
the same variant to `AlertDescription`
([RFC 0058](../rfcs/0058-status-variants.md)).

## Accessibility Notes

`Alert` always renders `role="alert"`, so use it for messages that should be
announced immediately; use a plain `div` for passive supporting copy.

`AlertTitle` renders a `div`, as in shadcn/ui v4, so it never skips a heading
level. Wrap its text in a heading of the right level when the page outline
needs one (see [RFC 0054](../rfcs/0054-automated-accessibility-audit.md)).
