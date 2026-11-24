use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeVariant, Step, StepStatus, Steps, StepsOrientation};

#[component]
pub fn StepsDescriptionsDemo() -> Element {
  rsx! {
    Steps { orientation: StepsOrientation::Vertical, class: "max-w-md", "aria-label": "Deployment milestones",
      Step { status: StepStatus::Complete,
        div { class: "space-y-1",
          div { class: "flex flex-wrap items-center gap-2",
            span { class: "font-semibold", "Build release bundle" }
            Badge { variant: BadgeVariant::Success, "Done" }
          }
          p { class: "text-xs font-normal text-muted-foreground",
            "Compiled server binary and optimized WASM assets in 42s."
          }
        }
      }
      Step { status: StepStatus::Current,
        div { class: "space-y-1",
          div { class: "flex flex-wrap items-center gap-2",
            span { class: "font-semibold", "Run accessibility & theme gates" }
            Badge { variant: BadgeVariant::Info, "Running" }
          }
          p { class: "text-xs font-normal text-muted-foreground",
            "Auditing WCAG AA contrast across light, dark, and 33 theme presets."
          }
        }
      }
      Step { status: StepStatus::Upcoming,
        div { class: "space-y-1",
          div { class: "flex flex-wrap items-center gap-2",
            span { class: "font-semibold", "Publish to GitHub Pages" }
            Badge { variant: BadgeVariant::Outline, "Queued" }
          }
          p { class: "text-xs font-normal text-muted-foreground",
            "Deploy static showcase to ui.dioxuspress.dev after checks pass."
          }
        }
      }
    }
  }
}
