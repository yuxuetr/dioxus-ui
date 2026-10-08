//! Data Table: controlled parts and pure state helpers for sortable, selectable,
//! paginated tables. The app owns the rows, fetching, and filtering.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;
use std::ops::Range;

/// Which way a sorted column orders its rows.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataTableSortDirection {
  /// Smallest first.
  Ascending,
  /// Largest first.
  Descending,
}

/// The column a table is sorted by, and in which direction. No sort is `None`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataTableSortState {
  /// Id of the sorted column.
  pub column_id: String,
  /// Sort direction.
  pub direction: DataTableSortDirection,
}

impl DataTableSortState {
  /// Sort `column_id` ascending.
  pub fn ascending(column_id: impl Into<String>) -> Self {
    Self { column_id: column_id.into(), direction: DataTableSortDirection::Ascending }
  }

  /// Sort `column_id` descending.
  pub fn descending(column_id: impl Into<String>) -> Self {
    Self { column_id: column_id.into(), direction: DataTableSortDirection::Descending }
  }
}

/// The sort after clicking `column_id`'s header. A column cycles ascending, descending, then
/// unsorted (`None`); clicking a different column starts it at ascending.
pub fn data_table_toggle_sort(
  current: Option<&DataTableSortState>,
  column_id: &str,
) -> Option<DataTableSortState> {
  match current {
    Some(current)
      if current.column_id == column_id
        && current.direction == DataTableSortDirection::Ascending =>
    {
      Some(DataTableSortState::descending(column_id))
    }
    Some(current)
      if current.column_id == column_id
        && current.direction == DataTableSortDirection::Descending =>
    {
      None
    }
    _ => Some(DataTableSortState::ascending(column_id)),
  }
}

/// Number of pages for `total_items` rows, rounding up. 0 when there are no rows or
/// `page_size` is 0.
pub fn data_table_page_count(total_items: usize, page_size: usize) -> usize {
  if page_size == 0 || total_items == 0 {
    return 0;
  }

  total_items.div_ceil(page_size)
}

/// Indexes of the rows on `page`, for slicing the full row list. `page` is clamped to the last
/// page, the last page may be short, and the range is empty when there are no rows or
/// `page_size` is 0.
pub fn data_table_page_window(page: usize, page_size: usize, total_items: usize) -> Range<usize> {
  if page_size == 0 || total_items == 0 {
    return 0..0;
  }

  let page = data_table_clamp_page(page, total_items, page_size);
  let start = page.saturating_mul(page_size).min(total_items);
  let end = start.saturating_add(page_size).min(total_items);

  start..end
}

pub fn data_table_clamp_page(page: usize, total_items: usize, page_size: usize) -> usize {
  let page_count = data_table_page_count(total_items, page_size);

  if page_count == 0 { 0 } else { page.min(page_count - 1) }
}

/// `selected_ids` with `row_id` added, or removed if present; sorted and deduplicated.
pub fn data_table_toggle_row(selected_ids: &[String], row_id: &str) -> Vec<String> {
  let mut next = selected_ids.to_vec();

  if let Some(index) = next.iter().position(|id| id == row_id) {
    next.remove(index);
  } else {
    next.push(row_id.to_string());
  }

  sorted_unique(next)
}

/// `selected_ids` after the header checkbox: removes every id in `visible_ids` when all are
/// already selected, otherwise adds the missing ones. The result is sorted and deduplicated;
/// an empty `visible_ids` counts as all selected and changes nothing.
pub fn data_table_toggle_all_rows(selected_ids: &[String], visible_ids: &[String]) -> Vec<String> {
  let all_visible_selected =
    visible_ids.iter().all(|id| selected_ids.iter().any(|selected_id| selected_id == id));

  let mut next = selected_ids.to_vec();

  if all_visible_selected {
    next.retain(|id| !visible_ids.iter().any(|visible_id| visible_id == id));
  } else {
    for id in visible_ids {
      if !next.iter().any(|selected_id| selected_id == id) {
        next.push(id.clone());
      }
    }
  }

  sorted_unique(next)
}

/// `hidden_ids` with `column_id` added, or removed if present; sorted and deduplicated.
pub fn data_table_toggle_column(hidden_ids: &[String], column_id: &str) -> Vec<String> {
  let mut next = hidden_ids.to_vec();

  if let Some(index) = next.iter().position(|id| id == column_id) {
    next.remove(index);
  } else {
    next.push(column_id.to_string());
  }

  sorted_unique(next)
}

/// Whether `column_id` is absent from `hidden_ids`.
pub fn data_table_is_column_visible(hidden_ids: &[String], column_id: &str) -> bool {
  !hidden_ids.iter().any(|id| id == column_id)
}

fn sorted_unique(mut values: Vec<String>) -> Vec<String> {
  values.sort();
  values.dedup();
  values
}

const DATA_TABLE_BASE_CLASS: &str = "grid gap-4";
const DATA_TABLE_TOOLBAR_BASE_CLASS: &str =
  "flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between";
const DATA_TABLE_CONTAINER_BASE_CLASS: &str =
  "relative w-full overflow-auto rounded-md border border-border";
const DATA_TABLE_HEADER_CELL_BASE_CLASS: &str = "h-12 px-4 text-left align-middle text-sm font-medium data-[sort=ascending]:text-foreground data-[sort=descending]:text-foreground";
const DATA_TABLE_ROW_BASE_CLASS: &str = "border-b border-border transition-colors hover:bg-muted data-[selected=true]:bg-muted data-[disabled=true]:opacity-50";
const DATA_TABLE_CELL_BASE_CLASS: &str = "p-4 align-middle text-sm";
const DATA_TABLE_PAGINATION_BASE_CLASS: &str =
  "flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between";
const DATA_TABLE_SELECTED_COUNT_BASE_CLASS: &str = "text-sm text-muted-foreground";
const DATA_TABLE_EMPTY_BASE_CLASS: &str = "py-10 text-center text-sm text-muted-foreground";
const DATA_TABLE_LOADING_BASE_CLASS: &str = "py-10 text-center text-sm text-muted-foreground";

fn data_table_class(class: &str) -> String {
  merge_classes(classes([Some(DATA_TABLE_BASE_CLASS)]), class)
}

fn data_table_toolbar_class(class: &str) -> String {
  merge_classes(classes([Some(DATA_TABLE_TOOLBAR_BASE_CLASS)]), class)
}

fn data_table_container_class(class: &str) -> String {
  merge_classes(classes([Some(DATA_TABLE_CONTAINER_BASE_CLASS)]), class)
}

fn data_table_header_cell_class(sorted: bool, class: &str) -> String {
  merge_classes(classes([Some(DATA_TABLE_HEADER_CELL_BASE_CLASS), Some(if sorted { "text-foreground" } else { "text-muted-foreground" })]), class)
}

fn data_table_row_class(selected: bool, disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(DATA_TABLE_ROW_BASE_CLASS), selected.then_some("bg-muted"), disabled.then_some("opacity-50")]), class)
}

fn data_table_cell_class(hidden: bool, class: &str) -> String {
  merge_classes(classes([Some(DATA_TABLE_CELL_BASE_CLASS), hidden.then_some("hidden")]), class)
}

fn data_table_pagination_class(class: &str) -> String {
  merge_classes(classes([Some(DATA_TABLE_PAGINATION_BASE_CLASS)]), class)
}

fn data_table_selected_count_class(class: &str) -> String {
  merge_classes(classes([Some(DATA_TABLE_SELECTED_COUNT_BASE_CLASS)]), class)
}

fn data_table_empty_class(class: &str) -> String {
  merge_classes(classes([Some(DATA_TABLE_EMPTY_BASE_CLASS)]), class)
}

fn data_table_loading_class(class: &str) -> String {
  merge_classes(classes([Some(DATA_TABLE_LOADING_BASE_CLASS)]), class)
}

fn data_table_sort_attribute(direction: Option<DataTableSortDirection>) -> &'static str {
  match direction {
    Some(DataTableSortDirection::Ascending) => "ascending",
    Some(DataTableSortDirection::Descending) => "descending",
    None => "none",
  }
}

#[component]
pub fn DataTable(#[props(default)] class: String, children: Element) -> Element {
  let class = data_table_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn DataTableToolbar(#[props(default)] class: String, children: Element) -> Element {
  let class = data_table_toolbar_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn DataTableContainer(#[props(default)] class: String, children: Element) -> Element {
  let class = data_table_container_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn DataTableHeaderCell(
  #[props(default)] sorted: bool,
  #[props(default)] direction: Option<DataTableSortDirection>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = data_table_header_cell_class(sorted, &class);

  rsx! {
    th {
      class,
      "aria-sort": data_table_sort_attribute(direction),
      "data-sort": data_table_sort_attribute(direction),
      {children}
    }
  }
}

#[component]
pub fn DataTableRow(
  #[props(default)] selected: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = data_table_row_class(selected, disabled, &class);

  rsx! {
    tr {
      class,
      "aria-disabled": disabled.to_string(),
      "data-disabled": disabled.to_string(),
      "data-selected": selected.to_string(),
      {children}
    }
  }
}

#[component]
pub fn DataTableCell(
  #[props(default)] hidden: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = data_table_cell_class(hidden, &class);

  rsx! {
    td {
      class,
      hidden,
      "data-hidden": hidden.to_string(),
      {children}
    }
  }
}

#[component]
pub fn DataTablePagination(#[props(default)] class: String, children: Element) -> Element {
  let class = data_table_pagination_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn DataTableSelectedCount(
  count: usize,
  total: usize,
  #[props(default)] class: String,
) -> Element {
  let class = data_table_selected_count_class(&class);

  rsx! {
    p {
      class,
      "{count} of {total} row(s) selected"
    }
  }
}

#[component]
pub fn DataTableEmpty(#[props(default)] class: String, children: Element) -> Element {
  let class = data_table_empty_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn DataTableLoading(#[props(default)] class: String, children: Element) -> Element {
  let class = data_table_loading_class(&class);

  rsx! {
    div {
      role: "status",
      class,
      {children}
    }
  }
}
