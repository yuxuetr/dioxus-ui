# RFC 0033: Pagination Page Changes

- Status: Accepted
- Created: 2026-10-05

## Summary

Let Pagination drive app state. `PaginationLink`, `PaginationPrevious`, and
`PaginationNext` take an `onclick` callback and render a button when they have
no `href`. A disabled control can no longer be focused or activated, and all
three pass through global and anchor attributes.

## Current State

As of M157:

- The three controls always render `a href`, with `""` as the default. They
  have no click handler, so an app that keeps the page in a signal cannot use
  them.
- `href=""` points at the current document, so an activated control reloads
  or scrolls instead of changing a page.
- On Desktop, the Dioxus interpreter intercepts anchor clicks and opens the
  `href` in the system browser, so anchors cannot change in-app state there at
  all.
- `disabled` adds `aria-disabled="true"` and `pointer-events-none`. Pointer
  events stop, but the link keeps its `href`, stays in the Tab order, and Enter
  still follows it.
- None of them accepts `id`, `title`, `target`, or `aria-*` beyond what they
  set.

## Decision

### API

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

- `onclick: Option<EventHandler<MouseEvent>>` on `PaginationLink`,
  `PaginationPrevious`, and `PaginationNext`.
- An empty `href` renders `button type="button"`. A native button turns Enter
  and Space into a click, and `disabled` removes it from the Tab order.
- A non-empty `href` keeps `a href`, so server-rendered or router pages still
  navigate. A disabled anchor drops `href`, which takes it out of the Tab
  order and stops Enter, and keeps `aria-disabled="true"`.
- A disabled control does not call `onclick`, even on a pointer press.
- `aria-current="page"` marks the active control in both forms.
- `attributes` extends `GlobalAttributes` and `a` and is spread after the
  explicit attributes, so `aria-label` on Previous and Next keeps its English
  default.

## Scope

In scope:

- the callback, the button form, the disabled anchor, and attribute spreading
  in the crate and the template
- the Pagination docs page
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| A page range helper with ellipses | Apps already know their page count and can build the list; the helper needs a sibling and boundary policy | A second consumer writes the same range code |
| Localized Previous and Next labels | The explicit `aria-label` and text win over spread attributes; a label prop is an API addition | A consumer ships a non-English Pagination |
| A `PaginationRoot` with page state and `on_page_change` | Each control already reports clicks, and the app owns the page count | A consumer reports repeating the same handlers |
| Desktop and Mobile self-test scenarios | The controls use plain click events with no script | They gain script behavior |

## Verification

- The CLI generated fixture smoke keeps the template compiling.
- The Web preview renders a five-page button Pagination and an anchor
  Pagination, and `npm run verify:runtime-interactions` asserts:
  - a click, Enter, and Space change the page and move `aria-current`;
  - Previous and Next are disabled at the ends;
  - an anchor control keeps its `href`, a disabled one has none, cannot take
    focus, and does not call `onclick`;
  - passed attributes render.
- Reverse checks: removing the callback, always rendering an anchor, keeping
  `href` on a disabled anchor, calling `onclick` while disabled, or not
  spreading the attributes each make the verifier fail.
