use dioxus::prelude::*;
use dioxus_shadcn::{Filter, FilterOption};

#[component]
pub fn FilterPillsDemo() -> Element {
  let mut selected = use_signal(|| "rust".to_string());

  rsx! {
    div { class: "flex flex-col gap-3 p-4",
      Filter {
        label: Some("Framework filter".to_string()),
        options: vec![
          FilterOption::new("rust", "Rust"),
          FilterOption::new("wasm", "WASM"),
          FilterOption::new("ssr", "SSR"),
          FilterOption::new("tailwind", "Tailwind"),
        ],
        value: Some(selected()),
        on_change: move |next| selected.set(next),
      }
      p { class: "text-sm text-muted-foreground",
        if selected().is_empty() {
          "Showing all frameworks"
        } else {
          "Active filter: {selected()}"
        }
      }
    }
  }
}
