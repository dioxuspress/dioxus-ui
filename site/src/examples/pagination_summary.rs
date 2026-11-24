use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, Pagination, PaginationContent, PaginationEllipsis,
  PaginationItem, PaginationLink, PaginationNext, PaginationPrevious, PaginationRangeItem,
  pagination_range,
};

const TOTAL_ITEMS: u32 = 128;
const PAGE_SIZES: [u32; 3] = [10, 25, 50];

#[component]
pub fn PaginationSummaryDemo() -> Element {
  let mut page = use_signal(|| 2_u32);
  let mut per_page = use_signal(|| 10_u32);

  let total_pages = TOTAL_ITEMS.div_ceil(per_page());
  let current = page().clamp(1, total_pages);
  let start = (current - 1) * per_page() + 1;
  let end = (current * per_page()).min(TOTAL_ITEMS);
  let entries = pagination_range(current, total_pages, 1);

  rsx! {
    div { class: "grid gap-4",
      div { class: "flex flex-wrap items-center justify-between gap-3 text-sm",
        p { class: "text-muted-foreground",
          "Showing {start}–{end} of {TOTAL_ITEMS} results"
        }
        div { class: "flex items-center gap-1.5",
          span { class: "text-muted-foreground", "Rows:" }
          for size in PAGE_SIZES {
            Button {
              key: "{size}",
              variant: if per_page() == size {
                ButtonVariant::Primary
              } else {
                ButtonVariant::Outline
              },
              size: ButtonSize::Xs,
              onclick: move |_| {
                per_page.set(size);
                page.set(1);
              },
              "{size}"
            }
          }
        }
      }
      Pagination { "aria-label": "Results pagination", class: "mx-0 justify-start",
        PaginationContent {
          PaginationItem {
            PaginationPrevious {
              disabled: current == 1,
              onclick: move |_| page.set(current.saturating_sub(1).max(1)),
            }
          }
          for (index, entry) in entries.into_iter().enumerate() {
            PaginationItem { key: "{index}",
              match entry {
                PaginationRangeItem::Page(number) => rsx! {
                  PaginationLink {
                    active: current == number,
                    onclick: move |_| page.set(number),
                    "{number}"
                  }
                },
                PaginationRangeItem::Ellipsis => rsx! { PaginationEllipsis {} },
              }
            }
          }
          PaginationItem {
            PaginationNext {
              disabled: current == total_pages,
              onclick: move |_| page.set((current + 1).min(total_pages)),
            }
          }
        }
      }
    }
  }
}
