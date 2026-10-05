# Empty

Empty provides empty-state layout composition parts for zero results, first-run
states, and filtered-empty states. It does not own actions, icons,
illustrations, or async loading behavior.

## Source Copy

```bash
dxui add empty
```

This generates:

```text
src/components/ui/empty.rs
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["empty"] }
```

## API Surface

- `Empty`
- `EmptyHeader`
- `EmptyTitle`
- `EmptyDescription`
- `EmptyContent`
- `EmptyActions`
- class helpers for every part

## Accessibility Notes

- Empty does not add implicit `alert` or `status` semantics.
- Use clear text in `EmptyTitle` and `EmptyDescription` so the empty reason is
  available without relying on icons.
- Actions remain regular app-owned controls.
