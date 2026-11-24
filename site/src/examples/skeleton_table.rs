use dioxus::prelude::*;
use dioxus_shadcn::Skeleton;

#[component]
pub fn SkeletonTableDemo() -> Element {
  rsx! {
    div {
      class: "w-full overflow-x-auto rounded-lg border border-border",
      role: "status",
      "aria-busy": "true",
      "aria-label": "Loading table rows",
      div { class: "min-w-[28rem] divide-y divide-border",
        div { class: "grid grid-cols-4 gap-4 bg-muted/40 px-4 py-3",
          Skeleton { class: "h-3.5 w-20" }
          Skeleton { class: "h-3.5 w-24" }
          Skeleton { class: "h-3.5 w-16" }
          Skeleton { class: "h-3.5 w-14 justify-self-end" }
        }
        for row in 0..4 {
          div { key: "{row}", class: "grid grid-cols-4 items-center gap-4 px-4 py-3.5",
            Skeleton { class: "h-4 w-28" }
            Skeleton { class: "h-4 w-32" }
            Skeleton { class: "h-5 w-16 rounded-full" }
            Skeleton { class: "h-4 w-12 justify-self-end" }
          }
        }
      }
    }
  }
}
