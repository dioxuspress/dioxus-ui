use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeVariant, Timeline, TimelineContent, TimelineItem, TimelineMarker, TimelineTime,
};

#[component]
pub fn TimelineCardsDemo() -> Element {
  rsx! {
    Timeline { class: "max-w-lg",
      TimelineItem {
        TimelineTime { datetime: "2026-11-10", "Nov 10" }
        TimelineMarker {}
        TimelineContent {
          div { class: "rounded-lg border border-border bg-card p-3.5 text-card-foreground shadow-sm",
            div { class: "flex flex-wrap items-center justify-between gap-2",
              p { class: "text-sm font-semibold", "v0.6.8 published" }
              Badge { variant: BadgeVariant::Success, "Released" }
            }
            p { class: "mt-1 text-xs leading-5 text-muted-foreground",
              "Added calendar wheel pickers, daisyUI card variants, and bilingual site docs."
            }
          }
        }
      }
      TimelineItem {
        TimelineTime { datetime: "2026-11-18", "Nov 18" }
        TimelineMarker {}
        TimelineContent {
          div { class: "rounded-lg border border-border bg-card p-3.5 text-card-foreground shadow-sm",
            div { class: "flex flex-wrap items-center justify-between gap-2",
              p { class: "text-sm font-semibold", "CSP verification gate" }
              Badge { variant: BadgeVariant::Info, "Security" }
            }
            p { class: "mt-1 text-xs leading-5 text-muted-foreground",
              "Verified zero-eval runtime scripts across all overlay and roving-focus widgets."
            }
          }
        }
      }
      TimelineItem {
        TimelineTime { datetime: "2026-11-24", "Nov 24" }
        TimelineMarker {}
        TimelineContent {
          div { class: "rounded-lg border border-dashed border-border bg-muted/30 p-3.5 text-foreground",
            div { class: "flex flex-wrap items-center justify-between gap-2",
              p { class: "text-sm font-semibold", "v0.7.0 planning" }
              Badge { variant: BadgeVariant::Outline, "Upcoming" }
            }
            p { class: "mt-1 text-xs leading-5 text-muted-foreground",
              "Expanding soft/dash surface variants and multi-scenario interactive examples."
            }
          }
        }
      }
    }
  }
}
