use dioxus::prelude::*;
use dioxus_shadcn::{
  Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
  PaginationNext, PaginationPrevious,
};

const LAST_PAGE: u32 = 10;

#[component]
pub fn Demo() -> Element {
  let mut page = use_signal(|| 1_u32);
  // Show the first pages, an ellipsis, and the last page.
  let numbers = [1, 2, 3];

  rsx! {
    Pagination {
      PaginationContent {
        PaginationItem {
          PaginationPrevious { disabled: page() == 1, onclick: move |_| page -= 1 }
        }
        for number in numbers {
          PaginationItem { key: "{number}",
            PaginationLink { active: page() == number, onclick: move |_| page.set(number), "{number}" }
          }
        }
        PaginationItem { PaginationEllipsis {} }
        PaginationItem {
          PaginationLink { active: page() == LAST_PAGE, onclick: move |_| page.set(LAST_PAGE), "{LAST_PAGE}" }
        }
        PaginationItem {
          PaginationNext { disabled: page() == LAST_PAGE, onclick: move |_| page += 1 }
        }
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Page {page} of {LAST_PAGE}" }
  }
}
