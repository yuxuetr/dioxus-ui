use dioxus::prelude::*;
use dioxus_ui_core::classes;
pub use dioxus_ui_primitives::{
  DataTableColumnState, DataTablePaginationState, DataTableSelectionState, DataTableSortDirection,
  DataTableSortState, data_table_clamp_page, data_table_is_column_visible, data_table_page_count,
  data_table_page_window, data_table_toggle_all_rows, data_table_toggle_column,
  data_table_toggle_row, data_table_toggle_sort,
};

pub const DATA_TABLE_BASE_CLASS: &str = "grid gap-4";
pub const DATA_TABLE_TOOLBAR_BASE_CLASS: &str =
  "flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between";
pub const DATA_TABLE_CONTAINER_BASE_CLASS: &str =
  "relative w-full overflow-auto rounded-md border border-zinc-200";
pub const DATA_TABLE_HEADER_CELL_BASE_CLASS: &str =
  "h-12 px-4 text-left align-middle text-sm font-medium text-zinc-600 data-sort:text-zinc-950";
pub const DATA_TABLE_ROW_BASE_CLASS: &str =
  "border-b transition-colors hover:bg-zinc-100 data-selected:bg-zinc-100 data-disabled:opacity-50";
pub const DATA_TABLE_CELL_BASE_CLASS: &str = "p-4 align-middle text-sm";
pub const DATA_TABLE_PAGINATION_BASE_CLASS: &str =
  "flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between";
pub const DATA_TABLE_SELECTED_COUNT_BASE_CLASS: &str = "text-sm text-zinc-600";
pub const DATA_TABLE_EMPTY_BASE_CLASS: &str = "py-10 text-center text-sm text-zinc-500";
pub const DATA_TABLE_LOADING_BASE_CLASS: &str = "py-10 text-center text-sm text-zinc-500";

pub fn data_table_class(class: &str) -> String {
  classes([Some(DATA_TABLE_BASE_CLASS), Some(class)])
}

pub fn data_table_toolbar_class(class: &str) -> String {
  classes([Some(DATA_TABLE_TOOLBAR_BASE_CLASS), Some(class)])
}

pub fn data_table_container_class(class: &str) -> String {
  classes([Some(DATA_TABLE_CONTAINER_BASE_CLASS), Some(class)])
}

pub fn data_table_header_cell_class(sorted: bool, class: &str) -> String {
  classes([Some(DATA_TABLE_HEADER_CELL_BASE_CLASS), sorted.then_some("text-zinc-950"), Some(class)])
}

pub fn data_table_row_class(selected: bool, disabled: bool, class: &str) -> String {
  classes([
    Some(DATA_TABLE_ROW_BASE_CLASS),
    selected.then_some("bg-zinc-100"),
    disabled.then_some("opacity-50"),
    Some(class),
  ])
}

pub fn data_table_cell_class(hidden: bool, class: &str) -> String {
  classes([Some(DATA_TABLE_CELL_BASE_CLASS), hidden.then_some("hidden"), Some(class)])
}

pub fn data_table_pagination_class(class: &str) -> String {
  classes([Some(DATA_TABLE_PAGINATION_BASE_CLASS), Some(class)])
}

pub fn data_table_selected_count_class(class: &str) -> String {
  classes([Some(DATA_TABLE_SELECTED_COUNT_BASE_CLASS), Some(class)])
}

pub fn data_table_empty_class(class: &str) -> String {
  classes([Some(DATA_TABLE_EMPTY_BASE_CLASS), Some(class)])
}

pub fn data_table_loading_class(class: &str) -> String {
  classes([Some(DATA_TABLE_LOADING_BASE_CLASS), Some(class)])
}

pub fn data_table_sort_attribute(direction: Option<DataTableSortDirection>) -> &'static str {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn data_table_header_cell_class_reflects_sorted_state() {
    let actual = data_table_header_cell_class(true, "w-40");

    assert!(actual.contains(DATA_TABLE_HEADER_CELL_BASE_CLASS));
    assert!(actual.contains("text-zinc-950"));
    assert!(actual.ends_with("w-40"));
  }

  #[test]
  fn data_table_row_class_reflects_selected_and_disabled_state() {
    let actual = data_table_row_class(true, true, "cursor-default");

    assert!(actual.contains(DATA_TABLE_ROW_BASE_CLASS));
    assert!(actual.contains("bg-zinc-100"));
    assert!(actual.contains("opacity-50"));
    assert!(actual.ends_with("cursor-default"));
  }

  #[test]
  fn data_table_sort_attribute_maps_direction() {
    assert_eq!(data_table_sort_attribute(Some(DataTableSortDirection::Ascending)), "ascending");
    assert_eq!(data_table_sort_attribute(None), "none");
  }

  #[test]
  fn data_table_primitives_are_reexported() {
    assert_eq!(data_table_page_count(25, 10), 3);
  }
}
