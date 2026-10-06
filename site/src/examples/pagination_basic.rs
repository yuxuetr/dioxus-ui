use dioxus::prelude::*;
use dioxus_shadcn::{
  Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
  PaginationNext, PaginationPrevious, PaginationRangeItem, pagination_range,
};

const LAST_PAGE: u32 = 10;

#[component]
pub fn PaginationBasicDemo() -> Element {
  let mut page = use_signal(|| 1_u32);
  // The first and last page, one page on each side of the current one, and
  // ellipses for the gaps.
  let entries = pagination_range(page(), LAST_PAGE, 1);

  rsx! {
    Pagination {
      PaginationContent {
        PaginationItem {
          PaginationPrevious { disabled: page() == 1, onclick: move |_| page -= 1 }
        }
        for (index, entry) in entries.into_iter().enumerate() {
          PaginationItem { key: "{index}",
            match entry {
              PaginationRangeItem::Page(number) => rsx! {
                PaginationLink { active: page() == number, onclick: move |_| page.set(number), "{number}" }
              },
              PaginationRangeItem::Ellipsis => rsx! { PaginationEllipsis {} },
            }
          }
        }
        PaginationItem {
          PaginationNext { disabled: page() == LAST_PAGE, onclick: move |_| page += 1 }
        }
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Page {page} of {LAST_PAGE}" }
  }
}
