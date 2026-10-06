# Card

Card groups related content with optional header, description, content, and
footer sections.

## Source Copy

```bash
dxui add card
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["card"] }
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

Cards do not add landmarks or interactive semantics. `CardTitle` renders a
`div`, as in shadcn/ui v4; when the title should be a heading, wrap its text in
a heading at the level the page outline needs.
