use dioxus::prelude::*;
use dioxus_shadcn::Progress;

#[component]
pub fn ProgressTonesDemo() -> Element {
  rsx! {
    div { class: "grid max-w-md gap-4",
      div { class: "grid gap-1.5",
        div { class: "flex justify-between text-xs text-muted-foreground",
          span { "Default · xs" }
          span { "35%" }
        }
        Progress { class: "h-1.5", value: 35.0, "aria-label": "Default progress" }
      }
      div { class: "grid gap-1.5",
        div { class: "flex justify-between text-xs text-muted-foreground",
          span { "Success · sm" }
          span { "72%" }
        }
        Progress {
          class: "h-2.5",
          indicator_class: "bg-success",
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
          class: "h-3.5",
          indicator_class: "bg-warning",
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
          class: "h-5",
          indicator_class: "bg-destructive",
          value: 96.0,
          "aria-label": "Critical usage progress",
        }
      }
    }
  }
}
