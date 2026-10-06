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
dioxus-shadcn = { version = "0.3", default-features = false, features = ["breadcrumb"] }
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
- `BreadcrumbLink` passes through anchor attributes such as `title` and
  `target`.
- Routing and hidden labels for custom icons remain app-owned.
