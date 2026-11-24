use dioxus::prelude::*;
use dioxus_shadcn::Skeleton;

#[component]
pub fn SkeletonListDemo() -> Element {
  rsx! {
    div {
      class: "grid max-w-md divide-y divide-border rounded-lg border border-border",
      role: "status",
      "aria-busy": "true",
      "aria-label": "Loading activity feed",
      for row in 0..3 {
        div { key: "{row}", class: "flex items-center justify-between gap-4 p-4",
          div { class: "flex items-center gap-3",
            Skeleton { class: "h-10 w-10 rounded-full" }
            div { class: "grid gap-2",
              Skeleton { class: "h-3.5 w-36" }
              Skeleton { class: "h-3 w-24" }
            }
          }
          Skeleton { class: "h-7 w-16 rounded-md" }
        }
      }
    }
  }
}
