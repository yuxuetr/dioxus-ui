# RFC 0048: Phone-width Preview Layout

- Status: Accepted
- Created: 2026-10-05

## Summary

Make the preview page and Pagination fit a 375px viewport. The preview grids
get a one-column template below their breakpoints, Pagination content wraps,
and the browser check measures the page at phone width.

## Current State

As of M172:

- The preview page, which the Web, Desktop, and Mobile previews share, lays
  out its panels with grids such as `grid gap-4 lg:grid-cols-2`. Below the
  breakpoint, the grid has no column template, so its implicit `auto` track
  grows to the widest item's min-content width. The inventory cards hold
  class strings in `whitespace-nowrap` code blocks, so at 375px the page is
  2379px wide and scrolls sideways. The iOS and Android previews render the
  same page in a phone WebView.
- `PaginationContent` is `flex flex-row items-center gap-1` inside the
  centered `Pagination` root. Previous, five pages, and Next need 360px. In a
  293px card, the row overflows equally on both sides, and the overflow on
  the left is outside the page, where scrolling cannot reach it.
- With the grids fixed, Pagination was the only rendered element that
  extended outside its fixture card without a clipping ancestor.

## Decision

- The preview grids add `grid-cols-1`, which Tailwind compiles to
  `repeat(1, minmax(0, 1fr))`. The track then cannot grow past the container,
  and a long code block is truncated with its ellipsis as intended.
- `PAGINATION_CONTENT_BASE_CLASS` becomes
  `flex flex-row flex-wrap items-center justify-center gap-1`. A row that
  does not fit wraps onto centered lines instead of overflowing.
- `npm run verify:runtime-interactions` resizes the page to 375px after the
  interactions. It fails when the page scrolls sideways, or when an element
  extends outside its fixture card and no ancestor inside the card clips it.

## Scope

In scope:

- the preview grid templates
- the Pagination content class in the crate and the template, and its docs
- the phone-width browser check

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| A Pagination that drops pages to fit | Which pages to show is the app's choice; wrapping keeps every link reachable | A consumer asks for a compact Pagination |
| Phone-width checks of open overlays | Overlays are fixed or anchored to the viewport, and the check measures cards | An overlay is reported to open off-screen on a phone |
| Phone-width checks in the Mobile self-tests | The page is the same; the Web check covers its layout | A WebView renders the page wider than Chrome does |

## Verification

- `npm run verify:runtime-interactions` passes at 375px with no horizontal
  page scroll and no element outside its fixture card.
- Reverse checks: removing `grid-cols-1` from the preview grids, or the wrap
  from the Pagination content, each make the verifier fail.
