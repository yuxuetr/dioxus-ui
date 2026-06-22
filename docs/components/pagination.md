# Pagination

Pagination provides styled navigation parts for paged result sets.

## Source Copy

```bash
dxui add pagination
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["pagination"] }
```

## API Surface

- `Pagination`
- `PaginationContent`
- `PaginationItem`
- `PaginationLink`
- `PaginationPrevious`
- `PaginationNext`
- `PaginationEllipsis`
- `pagination_link_class`

## Accessibility Notes

Pagination renders a navigation region with a pagination label. Mark the
current page with `active` and avoid linking disabled previous or next items to
unavailable pages.
