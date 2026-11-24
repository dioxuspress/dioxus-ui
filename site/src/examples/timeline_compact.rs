use dioxus::prelude::*;
use dioxus_shadcn::{
  Timeline, TimelineContent, TimelineItem, TimelineMarker, TimelineTime,
};

#[component]
pub fn TimelineCompactDemo() -> Element {
  rsx! {
    Timeline { class: "max-w-md text-sm",
      TimelineItem {
        TimelineTime { datetime: "09:14", "09:14" }
        TimelineMarker {
          span { class: "size-2.5 rounded-full bg-success" }
        }
        TimelineContent { class: "pb-4",
          p { class: "font-medium", "CI pipeline passed" }
          p { class: "text-xs text-muted-foreground", "9/9 multi-arch images pushed to GHCR." }
        }
      }
      TimelineItem {
        TimelineTime { datetime: "09:18", "09:18" }
        TimelineMarker {
          span { class: "size-2.5 rounded-full bg-info" }
        }
        TimelineContent { class: "pb-4",
          p { class: "font-medium", "Canary traffic shifted" }
          p { class: "text-xs text-muted-foreground", "10% of requests routed to release candidate." }
        }
      }
      TimelineItem {
        TimelineTime { datetime: "09:25", "09:25" }
        TimelineMarker {
          span { class: "size-2.5 rounded-full bg-warning" }
        }
        TimelineContent { class: "pb-0",
          p { class: "font-medium", "Cache warmup triggered" }
          p { class: "text-xs text-muted-foreground", "Rebuilding static edge indexes for /docs." }
        }
      }
    }
  }
}
