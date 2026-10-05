# Card

Card groups related content with optional header, description, content, and
footer sections.

## Source Copy

```bash
dxui add card
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["card"] }
```

## API Surface

- `Card`
- `CardHeader`
- `CardTitle`
- `CardDescription`
- `CardContent`
- `CardFooter`
- `card_class`

## Accessibility Notes

Cards do not add landmarks or interactive semantics. `CardTitle` renders an
`h3`; when the page outline needs another level, put your own heading inside a
`CardHeader` instead.
