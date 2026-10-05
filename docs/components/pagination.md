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

## Page Changes

`PaginationLink`, `PaginationPrevious`, and `PaginationNext` take an
`onclick` callback. Without an `href` they render a button, so an app can keep
the page in a signal:

```rust
let mut page = use_signal(|| 1);

rsx! {
  Pagination {
    PaginationContent {
      PaginationItem {
        PaginationPrevious { disabled: page() == 1, onclick: move |_| page -= 1 }
      }
      for number in 1..=5 {
        PaginationItem { key: "{number}",
          PaginationLink { active: page() == number, onclick: move |_| page.set(number), "{number}" }
        }
      }
      PaginationItem {
        PaginationNext { disabled: page() == 5, onclick: move |_| page += 1 }
      }
    }
  }
}
```

- An empty `href` renders `button type="button"`; Enter and Space activate
  it. Use this form for in-app pages: Desktop opens anchors in the system
  browser.
- A non-empty `href` renders an anchor for server-rendered or router pages.
- A disabled control does not call `onclick`. A disabled button uses the
  native `disabled`, and a disabled anchor drops its `href`, so neither takes
  focus or follows Enter.
- Global and anchor attributes such as `id`, `title`, and `target` pass
  through. A passed `aria-label` replaces the English default on Previous and
  Next, in the browser and in server-rendered HTML; the visible "Previous" and
  "Next" text stays English.

## Narrow Containers

`PaginationContent` wraps its items onto centered lines when they do not fit,
so every link stays reachable at phone width. To keep a single line, show
fewer pages with `PaginationEllipsis`.

## Accessibility Notes

Pagination renders a navigation region with a pagination label. Mark the
current page with `active`, which sets `aria-current="page"` on either form.
