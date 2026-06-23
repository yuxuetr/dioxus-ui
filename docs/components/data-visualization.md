# Data Table and Chart Strategy

This document defines the M16 Data Table and Chart strategy before
implementation. The goal is to add a useful Data Table composition layer while
keeping Chart as an explicit strategy decision instead of shipping a weak
backend too early.

Status: Planned in M16.

## Scope

M16 covers:

- Data Table
- Chart strategy

Data Table should ship in crate mode and source-copy mode. Crate mode can reuse
`dioxus-ui-core`, `dioxus-ui-primitives`, and existing styled components;
generated templates must remain self-contained and must not import internal
crates.

Chart should not ship as a public component in M16 unless the backend, data API,
accessibility contract, and Web/Desktop constraints are clear.

## Data Table Boundaries

Data Table is a composition layer over existing table, pagination, checkbox,
and command/search surfaces. It should not own row data fetching, async loading,
or virtualization in the first implementation.

Data Table owns:

- styled composition parts for toolbar, container, header cells, rows, cells,
  pagination slot, selected-count text, empty state, and loading state
- pure helpers for sorting state, pagination windows, row selection, and column
  visibility
- data attributes for sorted, selected, hidden, loading, and empty states

The consuming app owns:

- row data storage
- filtering implementation and query parsing
- async loading and error state transitions
- server-side sorting or pagination
- virtualized rendering
- cell rendering and formatting

## Data Table State Primitives

Planned primitive API:

```rust
DataTableSortDirection::{Ascending, Descending}
DataTableSortState { column_id, direction }
DataTablePaginationState { page, page_size, total_items }
DataTableSelectionState { selected_ids }
DataTableColumnState { hidden_ids }
```

Planned helper API:

```rust
data_table_toggle_sort(current, column_id) -> Option<DataTableSortState>
data_table_page_count(total_items, page_size) -> usize
data_table_page_window(page, page_size, total_items) -> Range<usize>
data_table_toggle_row(selected_ids, row_id) -> Vec<String>
data_table_toggle_all_rows(selected_ids, visible_ids) -> Vec<String>
data_table_is_column_visible(hidden_ids, column_id) -> bool
```

Rules:

- helpers are deterministic and pure
- page numbers are zero-based internally
- sorting state does not compare row data directly in primitives
- row IDs and column IDs are strings owned by the app
- hidden columns are tracked separately from row data

## Data Table Component

Planned crate API:

```rust
DataTable { class, children }
DataTableToolbar { class, children }
DataTableContainer { class, children }
DataTableHeaderCell { sorted, direction, class, children }
DataTableRow { selected, disabled, class, children }
DataTableCell { hidden, class, children }
DataTablePagination { class, children }
DataTableSelectedCount { count, total, class }
DataTableEmpty { class, children }
DataTableLoading { class, children }
```

Behavior defaults:

- table semantics remain provided by the underlying `Table` parts or app markup
- selected row state maps to `data-selected`
- sorted header state maps to `aria-sort` and `data-sort`
- hidden cells set `hidden`
- pagination is slotted so apps can use existing Pagination parts

## Chart Strategy

Chart is deferred as a component and documented as a strategy in M16. See the
[Chart Strategy](chart-strategy.md) for the backend and accessibility policy.

M16 position:

- do not introduce a first-party rendering backend in M16
- do not hand-roll chart SVG primitives before accessibility and data APIs are
  specified
- prefer an adapter strategy that can support multiple backends later
- document the minimum contract for future chart components

Future Chart requirements:

- explicit data model per chart type
- accessible title, description, and tabular fallback expectations
- color-token strategy that does not rely on color alone
- Web/Desktop rendering support plan
- SSR and hydration behavior plan
- generated source compatibility

## Platform Defaults

| Target | Data and visualization |
| --- | --- |
| Web | Data Table can use semantic tables; Chart needs a backend decision. |
| Desktop | Data Table should avoid DOM measurement assumptions; Chart needs WebView validation. |
| Mobile | Data Table should support horizontal overflow and compact controls; large tables may need app-owned alternate layouts. |

## Implementation Order

1. Data Table and Chart strategy plan
2. Data Table state primitives
3. Data Table styled composition parts
4. Chart strategy document
5. documentation, examples, and parity updates

This order implements the lower-risk Data Table surface while keeping Chart out
of the public API until the backend decision is explicit.

## Quality Gates

Each M16 implementation task should include:

- primitive unit tests for state helpers
- crate-mode class composition tests
- registry entry and self-contained template for public components
- component docs page
- web and desktop demo usage
- generated fixture smoke coverage
- per-feature compile coverage

Before marking implementation tasks done, run:

```bash
cargo test --workspace --all-features
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
