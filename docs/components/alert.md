# Alert

Alert presents important status or validation messages with title and
description parts.

## Source Copy

```bash
dxui add alert
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["alert"] }
```

## API Surface

- `Alert`
- `AlertTitle`
- `AlertDescription`
- `AlertVariant`
- `alert_class`
- `alert_title_class`
- `alert_description_class`

## Accessibility Notes

Alerts use `role="alert"` by default. Use this for messages that should be
announced immediately; use a non-alert container for passive supporting copy.

`AlertTitle` renders a `div`, as in shadcn/ui v4, so it never skips a heading
level. Wrap its text in a heading of the right level when the page outline
needs one (see [RFC 0054](../rfcs/0054-automated-accessibility-audit.md)).
