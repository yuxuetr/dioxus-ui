use dioxus::prelude::*;
use dioxus_ui::{Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow};

const INVOICES: [(&str, &str, &str, &str); 3] = [
  ("INV001", "Paid", "Credit card", "$250.00"),
  ("INV002", "Pending", "PayPal", "$150.00"),
  ("INV003", "Unpaid", "Bank transfer", "$350.00"),
];

#[component]
pub fn Demo() -> Element {
  rsx! {
    Table {
      TableCaption { "A list of your recent invoices." }
      TableHeader {
        TableRow {
          TableHead { "Invoice" }
          TableHead { "Status" }
          TableHead { "Method" }
          TableHead { class: "text-right", "Amount" }
        }
      }
      TableBody {
        for (invoice, status, method, amount) in INVOICES {
          TableRow { key: "{invoice}",
            TableCell { class: "font-medium", "{invoice}" }
            TableCell { "{status}" }
            TableCell { "{method}" }
            TableCell { class: "text-right", "{amount}" }
          }
        }
      }
      TableFooter {
        TableRow {
          TableCell { "Total" }
          TableCell {}
          TableCell {}
          TableCell { class: "text-right", "$750.00" }
        }
      }
    }
  }
}
