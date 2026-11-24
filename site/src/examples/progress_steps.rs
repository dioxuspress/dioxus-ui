use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeVariant, Progress};

#[component]
pub fn ProgressStepsDemo() -> Element {
  rsx! {
    div { class: "max-w-md space-y-4 rounded-lg border border-border bg-card p-4 text-card-foreground",
      div { class: "flex items-center justify-between gap-2",
        div {
          p { class: "text-sm font-semibold", "Workspace storage" }
          p { class: "text-xs text-muted-foreground", "7.8 GB of 10 GB used" }
        }
        Badge { variant: BadgeVariant::Warning, "78%" }
      }
      Progress {
        class: "h-2.5",
        indicator_class: "bg-warning",
        value: 78.0,
        "aria-label": "Total workspace storage used",
      }
      div { class: "grid gap-2.5 pt-1 text-xs",
        for (label, used, value, tone) in [
          ("Build artifacts", "4.6 GB", 46.0_f32, "bg-primary"),
          ("Media assets", "2.1 GB", 21.0_f32, "bg-info"),
          ("Database snapshots", "1.1 GB", 11.0_f32, "bg-success"),
        ] {
          div { key: "{label}", class: "grid gap-1",
            div { class: "flex justify-between text-muted-foreground",
              span { "{label}" }
              span { class: "tabular-nums", "{used}" }
            }
            Progress {
              class: "h-1.5",
              indicator_class: tone,
              value,
              "aria-label": "{label} usage",
            }
          }
        }
      }
    }
  }
}
