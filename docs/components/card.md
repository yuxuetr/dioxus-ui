# Card

Card groups related content with optional header, description, content, and
footer sections.

## Source Copy

```bash
dxui add card
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["card"] }
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

Cards do not add landmarks or interactive semantics by default. Choose heading
levels based on the surrounding page hierarchy.
