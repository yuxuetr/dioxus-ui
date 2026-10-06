# RFC 0075: Render-Scoped Element Ids

- Status: Accepted
- Created: 2026-10-06

## Summary

Number generated element ids per virtual DOM instead of per process, so a
server-rendered page and the browser that hydrates it agree on them.

## Current State

24 hooks and components take their element ids from process-wide counters
(`static NEXT_TABS_ID: AtomicUsize`). The ids link parts by ARIA
(`aria-controls`, `aria-labelledby`) and let page scripts find their element
(`[data-dxui-roving-group="dxui-roving-group-3"]`).

A server renders every request with the same counters, so the n-th request
writes ids starting near n. The browser starts its counters at 0, and Dioxus
0.7 hydration keeps the server's attribute values: it only records node ids
and `onmounted` listeners (`dioxus-web` 0.7.10, `hydration/hydrate.rs`).
After hydration the DOM holds the server's ids and the browser's hooks hold
their own, so each page script looks for an element that does not exist.

M206 observed this in a fullstack app with Tabs on Dioxus 0.7.9. Three
requests wrote `dxui-roving-group-1`, `-2`, and `-3`; after hydration
ArrowRight left focus on the first tab and the tabs got no tab stop. The same
app rendered in the browser only wrote `dxui-roving-group-0`, and ArrowRight
moved focus and selection to the second tab.

## Decision

A new `element_id` module holds `next_element_id()`, which reads a counter
from the root context of the current virtual DOM, providing it on first use,
and returns the next number. Each `NEXT_*.fetch_add(1, Ordering::Relaxed)`
call becomes `next_element_id()`, still inside `use_hook`, and the id
prefixes stay as they are.

A server builds a new virtual DOM per request, so every request numbers from
0. Hydration requires the browser to build the same tree, creating the same
hooks in the same order, so it hands out the same numbers. Within one virtual
DOM the counter only grows, so an id is never handed out twice.

Copy mode gets an `element_id.rs` helper template and an `element-id` helper
entry (RFC 0074). Each entry whose template calls it lists it as a
dependency, which the registry test already enforces.

## Alternatives

| Option | Why not |
| --- | --- |
| The component's `ScopeId` | Matches across server and browser, but Dioxus reuses the slots of unmounted scopes. The Sidebar shortcut, theme controller, and media query scripts find their element by id until it is gone, so a new component that got an old scope's id would keep an old script running, such as a theme controller applying a stale theme on a system theme change |
| `use_server_cached` to send server ids to the browser | Needs the Dioxus `fullstack` feature in a library that depends only on `dioxus`, and still leaves client-rendered components on separate counters |
| Ask apps to pass ids | Every compound component would need a new prop, and apps that never render on a server would pay for it |

## Impact

- No public API changes. Generated id values change: they restart at 0 per
  virtual DOM rather than per process.
- Two apps sharing one document each start at 0 and can collide; a page runs
  one Dioxus app.
- Apps that copied components before this change keep their counters until
  they re-copy; `dxui diff` lists the files that changed.

## Validation

- An SSR test renders a page with Tabs, Accordion, Checkbox, Slider, and a
  dialog twice in one process and expects the same HTML; it fails on the old
  counters.
- The fullstack app from M206 serves three requests with the same ids, and
  after hydration ArrowRight moves focus between tabs.
