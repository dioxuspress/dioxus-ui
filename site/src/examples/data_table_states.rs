use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, DataTable, DataTableCell, DataTableContainer, DataTableEmpty,
  DataTableHeaderCell, DataTableLoading, DataTablePagination, DataTableRow, DataTableToolbar,
  data_table_is_column_visible, data_table_page_count, data_table_toggle_column,
};

const RELEASES: [(&str, &str, &str, &str); 5] = [
  ("v0.7.0", "Stable", "2026-11-24", "142 KB"),
  ("v0.6.8", "Stable", "2026-11-18", "138 KB"),
  ("v0.6.7", "Archived", "2026-11-09", "134 KB"),
  ("v0.6.6", "Archived", "2026-10-28", "131 KB"),
  ("v0.6.5", "Archived", "2026-10-14", "129 KB"),
];
const PAGE_SIZE: usize = 2;

#[component]
pub fn DataTableStatesDemo() -> Element {
  let mut page = use_signal(|| 1_usize);
  let mut hidden_cols = use_signal(Vec::<String>::new);
  let mut mode = use_signal(|| "data");
  let show_date = data_table_is_column_visible(&hidden_cols(), "date");
  let show_size = data_table_is_column_visible(&hidden_cols(), "size");
  let page_count = data_table_page_count(RELEASES.len(), PAGE_SIZE);
  let start = (page() - 1) * PAGE_SIZE;
  let visible_rows = &RELEASES[start..(start + PAGE_SIZE).min(RELEASES.len())];

  rsx! {
    DataTable {
      DataTableToolbar {
        div { class: "flex flex-wrap gap-2",
          for (key, label) in [("data", "Data"), ("loading", "Loading"), ("empty", "Empty")] {
            Button {
              key: "{key}",
              size: ButtonSize::Xs,
              variant: if mode() == key { ButtonVariant::Primary } else { ButtonVariant::Outline },
              onclick: move |_| mode.set(key),
              "{label}"
            }
          }
        }
        div { class: "flex flex-wrap gap-2",
          Button {
            size: ButtonSize::Xs,
            variant: if show_date { ButtonVariant::Soft } else { ButtonVariant::Outline },
            onclick: move |_| hidden_cols.set(data_table_toggle_column(&hidden_cols(), "date")),
            if show_date { "Hide date" } else { "Show date" }
          }
          Button {
            size: ButtonSize::Xs,
            variant: if show_size { ButtonVariant::Soft } else { ButtonVariant::Outline },
            onclick: move |_| hidden_cols.set(data_table_toggle_column(&hidden_cols(), "size")),
            if show_size { "Hide size" } else { "Show size" }
          }
        }
      }
      DataTableContainer {
        if mode() == "loading" {
          DataTableLoading { "Fetching release artifacts..." }
        } else if mode() == "empty" {
          DataTableEmpty { "No releases match the active filter." }
        } else {
          table { class: "w-full text-sm",
            thead {
              tr {
                DataTableHeaderCell { "Version" }
                DataTableHeaderCell { "Channel" }
                if show_date {
                  DataTableHeaderCell { "Published" }
                }
                if show_size {
                  DataTableHeaderCell { "Bundle" }
                }
              }
            }
            tbody {
              for (version, channel, published, bundle) in visible_rows {
                DataTableRow { key: "{version}",
                  DataTableCell { class: "font-medium", "{version}" }
                  DataTableCell { "{channel}" }
                  DataTableCell { hidden: !show_date, "{published}" }
                  DataTableCell { hidden: !show_size, "{bundle}" }
                }
              }
            }
          }
        }
      }
      DataTablePagination {
        p { class: "text-sm text-muted-foreground", "Page {page} of {page_count}" }
        div { class: "flex gap-2",
          Button {
            size: ButtonSize::Sm,
            variant: ButtonVariant::Outline,
            disabled: page() <= 1,
            onclick: move |_| page.set(page().saturating_sub(1).max(1)),
            "Previous"
          }
          Button {
            size: ButtonSize::Sm,
            variant: ButtonVariant::Outline,
            disabled: page() >= page_count,
            onclick: move |_| page.set((page() + 1).min(page_count)),
            "Next"
          }
        }
      }
    }
  }
}
