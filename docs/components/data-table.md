# Data Table

Data Table provides controlled composition parts and pure state helpers for
building sortable, selectable, paginated tables. It does not own row data,
fetching, filtering, or virtualization.

## Source Copy

```bash
dxui add data-table
```

This creates Data Table and its common composition dependencies:

```text
src/components/ui/data_table.rs
src/components/ui/table.rs
src/components/ui/pagination.rs
src/components/ui/checkbox.rs
src/components/ui/command.rs
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["data-table"] }
```

```rust
use dioxus_shadcn::{DataTable, DataTableHeaderCell, DataTableSortDirection};
```

## API Surface

- `DataTable { class, children }`
- `DataTableToolbar { class, children }`
- `DataTableContainer { class, children }`
- `DataTableHeaderCell { sorted, direction, class, children }`
- `DataTableRow { selected, disabled, class, children }`
- `DataTableCell { hidden, class, children }`
- `DataTablePagination { class, children }`
- `DataTableSelectedCount { count, total, class }`
- `DataTableEmpty { class, children }`
- `DataTableLoading { class, children }`

State helpers:

- `data_table_toggle_sort(current, column_id)`
- `data_table_page_count(total_items, page_size)`
- `data_table_page_window(page, page_size, total_items)`
- `data_table_toggle_row(selected_ids, row_id)`
- `data_table_toggle_all_rows(selected_ids, visible_ids)`
- `data_table_toggle_column(hidden_ids, column_id)`
- `data_table_is_column_visible(hidden_ids, column_id)`

## Accessibility Notes

Use semantic `Table` parts or table markup inside `DataTableContainer`.
Sorted headers map to `aria-sort`, selected rows map to `data-selected`, hidden
cells use `hidden`, and loading content uses `role="status"`. Filtering,
keyboard shortcuts, async loading, and virtualization remain app-owned.
