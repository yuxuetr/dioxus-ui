# Breadcrumb

Breadcrumb provides semantic navigation composition parts. It owns structure and
styling only; routing, links, icons, and current-page decisions remain app-owned.

## Source Copy

```bash
dxui add breadcrumb
```

This generates:

```text
src/components/ui/breadcrumb.rs
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["breadcrumb"] }
```

## API Surface

- `Breadcrumb`
- `BreadcrumbList`
- `BreadcrumbItem`
- `BreadcrumbLink`
- `BreadcrumbPage`
- `BreadcrumbSeparator`
- `BreadcrumbEllipsis`
- class helpers for every part

## Accessibility Notes

- `Breadcrumb` renders a navigation region with a breadcrumb label.
- `BreadcrumbList` renders an ordered list.
- `BreadcrumbLink { current: true }` and `BreadcrumbPage` expose
  `aria-current="page"`.
- Routing and hidden labels for custom icons remain app-owned.
