# Table

Table provides styled semantic table parts for tabular data.

## Source Copy

```bash
dxui add table
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["table"] }
```

## API Surface

- `Table`
- `TableHeader`
- `TableBody`
- `TableFooter`
- `TableRow`
- `TableHead`
- `TableCell`
- `TableCaption`
- `table_class`

## Accessibility Notes

Use tables only for tabular data. Add captions when the table needs a concise
description, and use header cells for row or column labels.
