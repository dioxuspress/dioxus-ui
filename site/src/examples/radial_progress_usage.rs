use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, RadialProgress, RadialProgressSize, RadialProgressVariant,
};

#[component]
pub fn RadialProgressUsageDemo() -> Element {
  let mut value = use_signal(|| 60.0_f64);

  rsx! {
    div { class: "grid gap-5",
      div { class: "flex flex-wrap items-center gap-6",
        RadialProgress {
          value: value(),
          size: RadialProgressSize::Sm,
          variant: RadialProgressVariant::Info,
          "aria-label": "Small progress",
        }
        RadialProgress { value: value(), "aria-label": "Upload progress" }
        RadialProgress {
          value: value(),
          size: RadialProgressSize::Lg,
          variant: RadialProgressVariant::Success,
          "aria-label": "Tasks done",
          span { class: "text-center leading-tight",
            "{(value() / 20.0).round()}/5"
            br {}
            span { class: "text-xs text-muted-foreground", "tasks" }
          }
        }
        RadialProgress {
          value: (value() + 20.0).min(100.0),
          variant: RadialProgressVariant::Warning,
          "aria-label": "Memory threshold",
        }
        RadialProgress {
          value: (value() + 30.0).min(100.0),
          variant: RadialProgressVariant::Destructive,
          "aria-label": "Disk quota",
        }
      }
      div { class: "flex gap-2",
        Button {
          variant: ButtonVariant::Outline,
          size: ButtonSize::Sm,
          onclick: move |_| value.set((value() - 20.0).max(0.0)),
          "-20"
        }
        Button {
          variant: ButtonVariant::Outline,
          size: ButtonSize::Sm,
          onclick: move |_| value.set((value() + 20.0).min(100.0)),
          "+20"
        }
      }
    }
  }
}
