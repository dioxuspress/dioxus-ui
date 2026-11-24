use dioxus::prelude::*;
use dioxus_shadcn::{Status, StatusSize, StatusVariant};

#[component]
pub fn StatusSizesDemo() -> Element {
  rsx! {
    div { class: "grid gap-4 text-sm",
      div { class: "flex flex-wrap items-center gap-6",
        div { class: "flex items-center gap-2",
          Status { variant: StatusVariant::Primary, size: StatusSize::Sm }
          "Small (6px)"
        }
        div { class: "flex items-center gap-2",
          Status { variant: StatusVariant::Primary, size: StatusSize::Md }
          "Medium (8px)"
        }
        div { class: "flex items-center gap-2",
          Status { variant: StatusVariant::Primary, size: StatusSize::Lg }
          "Large (12px)"
        }
      }
      div { class: "flex flex-wrap items-center gap-4",
        span { class: "text-xs text-muted-foreground", "Standalone labeled dots:" }
        Status { variant: StatusVariant::Success, size: StatusSize::Md, label: "API healthy", class: "animate-pulse" }
        Status { variant: StatusVariant::Warning, size: StatusSize::Md, label: "Worker queue elevated" }
        Status { variant: StatusVariant::Info, size: StatusSize::Md, label: "Syncing replica" }
      }
    }
  }
}
