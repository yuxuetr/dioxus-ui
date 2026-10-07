//! Pure state for the data table: column sorting, paging, row selection, and column
//! visibility. The styled `DataTable` in `dioxus-shadcn` keeps its state in these types; rows
//! and columns are identified by caller-chosen string ids.

use std::ops::Range;

/// Which way a sorted column orders its rows.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataTableSortDirection {
  /// Smallest first.
  Ascending,
  /// Largest first.
  Descending,
}

impl DataTableSortDirection {
  /// The opposite direction.
  pub const fn toggled(self) -> Self {
    match self {
      Self::Ascending => Self::Descending,
      Self::Descending => Self::Ascending,
    }
  }
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

/// Which page of rows is showing. Pages are zero-based.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataTablePaginationState {
  /// Requested zero-based page; may be past the end, see `clamped_page`.
  pub page: usize,
  /// Rows per page; 0 means no pages.
  pub page_size: usize,
  /// Total number of rows across all pages.
  pub total_items: usize,
}

impl DataTablePaginationState {
  /// Paging state as given, without clamping.
  pub const fn new(page: usize, page_size: usize, total_items: usize) -> Self {
    Self { page, page_size, total_items }
  }

  /// Number of pages, rounding up; 0 when there are no rows or `page_size` is 0.
  pub fn page_count(self) -> usize {
    data_table_page_count(self.total_items, self.page_size)
  }

  /// `page` clamped to the last page, or 0 when there are no pages.
  pub fn clamped_page(self) -> usize {
    data_table_clamp_page(self.page, self.total_items, self.page_size)
  }

  /// Indexes of the rows on the current page; see `data_table_page_window`.
  pub fn page_window(self) -> Range<usize> {
    data_table_page_window(self.page, self.page_size, self.total_items)
  }
}

/// Which rows are selected.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DataTableSelectionState {
  /// Selected row ids, sorted and without duplicates when built through `new`.
  pub selected_ids: Vec<String>,
}

impl DataTableSelectionState {
  /// A selection of `selected_ids`, sorted and deduplicated.
  pub fn new(selected_ids: Vec<String>) -> Self {
    Self { selected_ids: sorted_unique(selected_ids) }
  }

  /// The selection with `row_id` added, or removed if it was selected.
  pub fn toggle_row(&self, row_id: &str) -> Self {
    Self::new(data_table_toggle_row(&self.selected_ids, row_id))
  }

  /// The selection after the header checkbox: clears `visible_ids` when all of them are
  /// selected, otherwise adds the missing ones. Rows not in `visible_ids` keep their state.
  pub fn toggle_all_rows(&self, visible_ids: &[String]) -> Self {
    Self::new(data_table_toggle_all_rows(&self.selected_ids, visible_ids))
  }

  /// Whether `row_id` is selected.
  pub fn is_selected(&self, row_id: &str) -> bool {
    self.selected_ids.iter().any(|id| id == row_id)
  }
}

/// Which columns are hidden. Every column not listed is visible.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DataTableColumnState {
  /// Hidden column ids, sorted and without duplicates when built through `new`.
  pub hidden_ids: Vec<String>,
}

impl DataTableColumnState {
  /// Column visibility hiding `hidden_ids`, sorted and deduplicated.
  pub fn new(hidden_ids: Vec<String>) -> Self {
    Self { hidden_ids: sorted_unique(hidden_ids) }
  }

  /// The state with `column_id` hidden, or shown again if it was hidden.
  pub fn toggle_column(&self, column_id: &str) -> Self {
    Self::new(data_table_toggle_column(&self.hidden_ids, column_id))
  }

  /// Whether `column_id` is shown.
  pub fn is_visible(&self, column_id: &str) -> bool {
    data_table_is_column_visible(&self.hidden_ids, column_id)
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

pub(crate) fn data_table_clamp_page(page: usize, total_items: usize, page_size: usize) -> usize {
  let page_count = data_table_page_count(total_items, page_size);

  if page_count == 0 { 0 } else { page.min(page_count - 1) }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn toggles_sort_through_ascending_descending_and_none() {
    let ascending = data_table_toggle_sort(None, "name");
    assert_eq!(ascending, Some(DataTableSortState::ascending("name")));

    let descending = data_table_toggle_sort(ascending.as_ref(), "name");
    assert_eq!(descending, Some(DataTableSortState::descending("name")));

    assert_eq!(data_table_toggle_sort(descending.as_ref(), "name"), None);
    assert_eq!(
      data_table_toggle_sort(descending.as_ref(), "email"),
      Some(DataTableSortState::ascending("email"))
    );
  }

  #[test]
  fn calculates_page_count_and_clamps_page() {
    assert_eq!(data_table_page_count(0, 10), 0);
    assert_eq!(data_table_page_count(25, 10), 3);
    assert_eq!(data_table_clamp_page(9, 25, 10), 2);
  }

  #[test]
  fn calculates_page_window_at_boundaries() {
    assert_eq!(data_table_page_window(0, 10, 25), 0..10);
    assert_eq!(data_table_page_window(2, 10, 25), 20..25);
    assert_eq!(data_table_page_window(9, 10, 25), 20..25);
    assert_eq!(data_table_page_window(0, 0, 25), 0..0);
  }

  #[test]
  fn toggles_row_selection_with_stable_unique_ids() {
    let selected = vec!["row-2".to_string(), "row-1".to_string(), "row-2".to_string()];
    let selected = data_table_toggle_row(&selected, "row-3");

    assert_eq!(selected, vec!["row-1", "row-2", "row-3"]);
    assert_eq!(data_table_toggle_row(&selected, "row-2"), vec!["row-1", "row-3"]);
  }

  #[test]
  fn toggles_all_visible_rows_without_losing_hidden_page_selection() {
    let selected = vec!["row-9".to_string()];
    let visible = vec!["row-1".to_string(), "row-2".to_string()];
    let selected = data_table_toggle_all_rows(&selected, &visible);

    assert_eq!(selected, vec!["row-1", "row-2", "row-9"]);
    assert_eq!(data_table_toggle_all_rows(&selected, &visible), vec!["row-9"]);
  }

  #[test]
  fn toggles_column_visibility() {
    let hidden = data_table_toggle_column(&[], "email");

    assert!(!data_table_is_column_visible(&hidden, "email"));
    assert_eq!(data_table_toggle_column(&hidden, "email"), Vec::<String>::new());
  }

  #[test]
  fn state_wrappers_apply_helpers() {
    let pagination = DataTablePaginationState::new(5, 10, 22);
    let selection = DataTableSelectionState::default().toggle_row("row-1");
    let columns = DataTableColumnState::default().toggle_column("email");

    assert_eq!(pagination.page_window(), 20..22);
    assert!(selection.is_selected("row-1"));
    assert!(!columns.is_visible("email"));
  }
}
