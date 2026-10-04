# RFC 0035: Control Label Overrides

- Status: Accepted
- Created: 2026-10-05

## Summary

Make a passed `aria-label` replace a component's English default the same way
in the browser and in server-side rendering. Pagination Previous and Next use
the label filter Carousel introduced in M159, the filter moves to one shared
helper, and a new SSR test checks the rendered HTML.

## Current State

As of M159:

- `PaginationPrevious` and `PaginationNext` render an explicit English
  `aria-label` and then spread the passed attributes, which may carry their
  own `aria-label`.
- An M159 SSR probe rendered `PaginationNext { "aria-label": "Nächste Seite" }`
  as `aria-label="Go to next page" aria-current="false"
  aria-label="Nächste Seite"`. An HTML parser keeps the first value, so the
  server-rendered page announces the English label.
- In the browser the Dioxus interpreter sets the attributes in order, so the
  passed label wins. The same component announces different names depending on
  the renderer, and RFC 0033 wrongly states that the English default always
  wins.
- Carousel Previous, Next, and Indicator already skip their default when the
  passed attributes carry an `aria-label`, through a private `default_label`
  in `carousel.rs`.
- No test renders HTML, so the browser verifier cannot see the difference.

## Decision

- A shared `default_aria_label(attributes, label) -> Option<&'static str>`
  returns `label` only when `attributes` has no `aria-label`. In the crate it
  lives in a private module compiled for the `carousel` and `pagination`
  features; in source-copy templates it lives in `utils.rs`.
- `PaginationPrevious` and `PaginationNext` pass their default through it, so
  a passed `aria-label` is the only one rendered.
- Carousel uses the shared helper instead of its private copy.
- `dioxus-ssr` becomes a dev-dependency of `dioxus-ui`. Unit tests render the
  controls with and without a passed label and assert that the HTML has exactly
  one `aria-label` with the expected value.

## Scope

In scope:

- the shared helper in the crate and the template utilities
- Pagination Previous and Next, and Carousel switching to the helper
- SSR unit tests and the Pagination docs page

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Localized visible "Previous" and "Next" text | The text is a child the component renders; replacing it needs a children or label prop | A consumer ships a non-English Pagination |
| Other explicit attributes duplicated by a spread, such as `role` or `type` | Those values define the component, and apps have no reason to replace them | A consumer reports needing to replace one |
| A `Pagination` root that accepts attributes | The root is a fixed `nav` with no spread | A consumer needs to name or identify the root |

## Verification

- SSR unit tests render `PaginationPrevious`, `PaginationNext`, and
  `CarouselIndicator` with a passed `aria-label` and check a single
  `aria-label` with the passed value, and without one and check the English
  default.
- The CLI generated fixture smoke keeps the templates compiling, and the CLI
  tests keep the template utilities in sync.
- `npm run verify:runtime-interactions` keeps passing.
- Reverse check: making the helper always return the default makes the SSR
  tests fail.
