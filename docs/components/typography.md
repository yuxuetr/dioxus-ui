# Typography

Typography provides styled semantic text composition parts. It does not parse
Markdown or manage heading hierarchy.

## Source Copy

```bash
dxui add typography
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["typography"] }
```

## API Surface

- `TypographyProse`
- `TypographyH1`
- `TypographyH2`
- `TypographyH3`
- `TypographyP`
- `TypographyLead`
- `TypographyMuted`
- `TypographyBlockquote`
- `TypographyInlineCode`

## Accessibility Notes

Typography parts use native text elements where practical. Apps own heading
order, document outline, rich text parsing, and generated content semantics.
