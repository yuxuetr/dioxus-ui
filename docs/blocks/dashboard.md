# Dashboard

An app shell: a sidebar, a header with search, key metrics, a revenue chart,
and a sortable table of recent orders.

```bash
dxui add dashboard
```

Components: Badge, Card, Chart, Data Table, Input, Sidebar, Stat.

## Behavior

- The sidebar collapses to its initials on wide screens and slides in as a
  modal panel below 768px ([RFC 0069](../rfcs/0069-off-canvas-sidebar.md));
  the header's trigger and Ctrl or Command and B toggle whichever applies.
  Choosing a section closes the panel on phones.
- The search filters the orders by customer, and the Amount header sorts
  them, highest first.
- Metrics, revenue, and orders come from constants at the top of the file;
  replace them with your data.

## Accessibility Notes

The sidebar is a named `aside` landmark (a named dialog when off-canvas), and
the active section has `aria-current="page"`. Collapsed labels stay in the
accessibility tree as hidden text. The metrics are a definition list that
scrolls sideways on narrow screens and takes keyboard focus to do so. The
chart has a title, a description, and a fallback table.
