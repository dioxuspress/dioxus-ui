use dioxus::prelude::*;
use dioxus_shadcn::{Progress, ProgressSize, ProgressVariant};

#[component]
pub fn ProgressTonesDemo() -> Element {
  rsx! {
    div { class: "grid max-w-md gap-4",
      div { class: "grid gap-1.5",
        div { class: "flex justify-between text-xs text-muted-foreground",
          span { "Default · xs" }
          span { "35%" }
        }
        Progress { size: ProgressSize::Xs, value: 35.0, "aria-label": "Default progress" }
      }
      div { class: "grid gap-1.5",
        div { class: "flex justify-between text-xs text-muted-foreground",
          span { "Success · sm" }
          span { "72%" }
        }
        Progress {
          variant: ProgressVariant::Success,
          size: ProgressSize::Sm,
          value: 72.0,
          "aria-label": "Success progress",
        }
      }
      div { class: "grid gap-1.5",
        div { class: "flex justify-between text-xs text-muted-foreground",
          span { "Warning · md" }
          span { "86%" }
        }
        Progress {
          variant: ProgressVariant::Warning,
          size: ProgressSize::Md,
          value: 86.0,
          "aria-label": "Warning progress",
        }
      }
      div { class: "grid gap-1.5",
        div { class: "flex justify-between text-xs text-muted-foreground",
          span { "Destructive · lg" }
          span { "96%" }
        }
        Progress {
          variant: ProgressVariant::Destructive,
          size: ProgressSize::Lg,
          value: 96.0,
          "aria-label": "Critical usage progress",
        }
      }
      div { class: "grid gap-1.5",
        div { class: "flex justify-between text-xs text-muted-foreground",
          span { "Info · sm" }
          span { "58%" }
        }
        Progress {
          variant: ProgressVariant::Info,
          size: ProgressSize::Sm,
          value: 58.0,
          "aria-label": "Info progress",
        }
      }
    }
  }
}
