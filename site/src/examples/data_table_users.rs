use dioxus::prelude::*;
use dioxus_shadcn::{
  Checkbox, DataTable, DataTableCell, DataTableContainer, DataTableHeaderCell, DataTableRow,
  DataTableSelectedCount, DataTableSortDirection, DataTableToolbar, Input, data_table_toggle_row,
};

const USERS: [(&str, &str, u32); 4] = [
  ("ada", "Ada Lovelace", 36),
  ("alan", "Alan Turing", 41),
  ("grace", "Grace Hopper", 85),
  ("linus", "Linus Torvalds", 54),
];

#[component]
pub fn DataTableUsersDemo() -> Element {
  let mut query = use_signal(String::new);
  let mut selected = use_signal(Vec::<String>::new);
  let mut ascending = use_signal(|| true);
  let mut rows = USERS
    .iter()
    .filter(|(_, name, _)| name.to_lowercase().contains(&query().to_lowercase()))
    .copied()
    .collect::<Vec<_>>();
  rows.sort_by_key(|(_, _, age)| *age);
  if !ascending() {
    rows.reverse();
  }
  let direction = if ascending() { DataTableSortDirection::Ascending } else { DataTableSortDirection::Descending };

  rsx! {
    DataTable {
      DataTableToolbar {
        Input {
          class: "max-w-xs",
          "aria-label": "Filter names",
          placeholder: "Filter names...",
          value: query(),
          on_value_change: move |value| query.set(value),
        }
      }
      DataTableContainer {
        table { class: "w-full text-sm",
          thead {
            tr {
              DataTableHeaderCell { span { class: "sr-only", "Select" } }
              DataTableHeaderCell { "Name" }
              DataTableHeaderCell { sorted: true, direction,
                button { class: "font-medium", onclick: move |_| ascending.toggle(), "Age" }
              }
            }
          }
          tbody {
            for (id, name, age) in rows {
              DataTableRow { key: "{id}", selected: selected().iter().any(|row| row == id),
                DataTableCell {
                  Checkbox {
                    "aria-label": "Select {name}",
                    checked: selected().iter().any(|row| row == id),
                    on_checked_change: move |_| selected.set(data_table_toggle_row(&selected(), id)),
                  }
                }
                DataTableCell { "{name}" }
                DataTableCell { "{age}" }
              }
            }
          }
        }
      }
      DataTableSelectedCount { count: selected().len(), total: USERS.len() }
    }
  }
}
