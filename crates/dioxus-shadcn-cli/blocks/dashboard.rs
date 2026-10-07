use dioxus::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::chart::{
  ChartAreaSeries, ChartColorToken, ChartDescription, ChartDomain, ChartFallbackTable,
  ChartLineSeries, ChartPoint, ChartRoot, ChartScale, ChartSeries, ChartSvg, ChartTitle,
  chart_fallback_rows,
};
use crate::components::ui::data_table::{
  DataTable, DataTableCell, DataTableContainer, DataTableHeaderCell, DataTableRow,
  DataTableSortDirection,
};
use crate::components::ui::input::Input;
use crate::components::ui::sidebar::{
  Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel, SidebarHeader,
  SidebarItem, SidebarProvider, SidebarTrigger,
};
use crate::components::ui::stat::{Stat, StatDescription, StatGroup, StatTitle, StatValue};

const SECTIONS: [(&str, &str); 4] = [
  ("overview", "Overview"),
  ("orders", "Orders"),
  ("customers", "Customers"),
  ("settings", "Settings"),
];

const REVENUE: [f64; 6] = [18.0, 22.0, 19.0, 27.0, 31.0, 36.0];

/// Sample orders: id, customer, status, amount in dollars.
const ORDERS: [(&str, &str, &str, u32); 5] = [
  ("#3210", "Olivia Martin", "Paid", 1999),
  ("#3209", "Jackson Lee", "Pending", 39),
  ("#3208", "Isabella Nguyen", "Paid", 299),
  ("#3207", "William Kim", "Refunded", 99),
  ("#3206", "Sofia Davis", "Paid", 450),
];

/// An app shell: a sidebar that collapses on wide screens and slides in on
/// phones (Ctrl or Command and B toggle it), a header with search, key
/// metrics, a revenue chart, and a sortable orders table. Replace the sample
/// constants with your data.
#[component]
pub fn DashboardBlock() -> Element {
  let mut section = use_signal(|| "overview");
  let mut collapsed = use_signal(|| false);
  let mut mobile_open = use_signal(|| false);
  let mut query = use_signal(String::new);
  let mut ascending = use_signal(|| false);

  let mut orders = ORDERS
    .iter()
    .filter(|(_, customer, _, _)| customer.to_lowercase().contains(&query().to_lowercase()))
    .copied()
    .collect::<Vec<_>>();
  orders.sort_by_key(|(_, _, _, amount)| *amount);
  if !ascending() {
    orders.reverse();
  }
  let direction = if ascending() {
    DataTableSortDirection::Ascending
  } else {
    DataTableSortDirection::Descending
  };

  let revenue = ChartSeries::new(
    "revenue",
    "Revenue",
    REVENUE
      .iter()
      .enumerate()
      .map(|(month, value)| ChartPoint::new(month as f64, *value))
      .collect(),
  );
  // Data mapped into the 640 by 320 view box, with room at the edges.
  let x = ChartScale::new(ChartDomain::new(0.0, 5.0), ChartDomain::new(32.0, 608.0));
  let y = ChartScale::new(ChartDomain::new(0.0, 40.0), ChartDomain::new(288.0, 24.0));
  let rows = chart_fallback_rows(std::slice::from_ref(&revenue));

  rsx! {
    // A full-height shell: the sidebar fills it and the content scrolls.
    // The labels read `collapsed` and items close the panel, so the app
    // controls both.
    SidebarProvider {
      collapsed: collapsed(),
      on_collapsed_change: move |next| collapsed.set(next),
      off_canvas: true,
      mobile_open: mobile_open(),
      on_mobile_open_change: move |next| mobile_open.set(next),
      div { class: "flex h-screen bg-background text-foreground",
        Sidebar { "aria-label": "Main", shortcut: 'b',
          SidebarHeader { span { class: "truncate font-semibold", "Acme" } }
          SidebarContent {
            SidebarGroup { role: "group", "aria-labelledby": "dashboard-nav-label",
              SidebarGroupLabel { id: "dashboard-nav-label", class: if collapsed() { "sr-only" } else { "" },
                "Workspace"
              }
              for (value, label) in SECTIONS {
                SidebarItem {
                  key: "{value}",
                  active: section() == value,
                  title: "{label}",
                  onclick: move |_| {
                    section.set(value);
                    mobile_open.set(false);
                  },
                  span { class: "w-4 shrink-0 text-center font-medium", "aria-hidden": "true",
                    "{label.chars().next().unwrap_or(' ')}"
                  }
                  span { class: if collapsed() { "sr-only" } else { "truncate" }, "{label}" }
                }
              }
            }
          }
          SidebarFooter {
            span { class: if collapsed() { "sr-only" } else { "truncate text-xs text-muted-foreground" },
              "ada@acme.example"
            }
          }
        }
        div { class: "flex min-w-0 flex-1 flex-col overflow-y-auto",
          header { class: "flex h-14 items-center gap-3 border-b border-border px-4",
            SidebarTrigger { "aria-label": "Toggle sidebar",
              span { "aria-hidden": "true", "☰" }
            }
            h1 { class: "text-lg font-semibold", "Overview" }
            div { class: "ms-auto w-full max-w-56",
              Input {
                r#type: "search",
                "aria-label": "Search customers",
                placeholder: "Search customers...",
                value: query(),
                on_value_change: move |value| query.set(value),
              }
            }
          }
          main { class: "grid min-w-0 gap-6 p-4 md:p-6",
            StatGroup { class: "w-full", tabindex: "0", "aria-label": "Key metrics",
              Stat {
                StatTitle { "Revenue" }
                StatValue { "$36,240" }
                StatDescription { "+16% from last month" }
              }
              Stat {
                StatTitle { "Orders" }
                StatValue { "1,284" }
                StatDescription { "+8% from last month" }
              }
              Stat {
                StatTitle { "Customers" }
                StatValue { "932" }
                StatDescription { "+42 this week" }
              }
              Stat {
                StatTitle { "Refund rate" }
                StatValue { "1.4%" }
                StatDescription { "-0.3% from last month" }
              }
            }
            Card {
              CardHeader {
                CardTitle { "Revenue" }
                CardDescription { "Thousands of dollars per month, January to June." }
              }
              CardContent {
                ChartRoot {
                  ChartTitle { id: "dashboard-revenue-title", class: "sr-only", "Revenue" }
                  ChartDescription { id: "dashboard-revenue-description", class: "sr-only",
                    "Monthly revenue in thousands of dollars, January to June."
                  }
                  ChartSvg { title_id: "dashboard-revenue-title", description_id: "dashboard-revenue-description",
                    ChartAreaSeries { series: revenue.clone(), x_scale: x, y_scale: y, color: ChartColorToken::Primary, baseline: 0.0 }
                    ChartLineSeries { series: revenue.clone(), x_scale: x, y_scale: y, color: ChartColorToken::Primary }
                  }
                  div { class: "sr-only", ChartFallbackTable { rows, caption: "Revenue by month" } }
                }
              }
            }
            Card {
              CardHeader {
                CardTitle { "Recent orders" }
                CardDescription { "The last five orders, by amount." }
              }
              CardContent {
                DataTable {
                  DataTableContainer {
                    table { class: "w-full text-sm",
                      thead {
                        tr {
                          DataTableHeaderCell { "Order" }
                          DataTableHeaderCell { "Customer" }
                          DataTableHeaderCell { "Status" }
                          DataTableHeaderCell { sorted: true, direction,
                            button { class: "font-medium", onclick: move |_| ascending.toggle(), "Amount" }
                          }
                        }
                      }
                      tbody {
                        for (id, customer, status, amount) in orders {
                          DataTableRow { key: "{id}",
                            DataTableCell { class: "font-medium", "{id}" }
                            DataTableCell { "{customer}" }
                            DataTableCell {
                              Badge {
                                variant: match status {
                                  "Paid" => BadgeVariant::Success,
                                  "Pending" => BadgeVariant::Warning,
                                  _ => BadgeVariant::Secondary,
                                },
                                "{status}"
                              }
                            }
                            DataTableCell { "${amount}" }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
