use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonSize, ButtonVariant, Spinner, SpinnerSize};

#[component]
pub fn ButtonIconsDemo() -> Element {
  let mut saving = use_signal(|| false);

  rsx! {
    div { class: "grid max-w-md gap-4",
      div { class: "flex flex-wrap items-center gap-2",
        Button { class: "gap-2",
          svg {
            class: "size-4",
            view_box: "0 0 16 16",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d: "M8 3v10M3 8h10" }
          }
          "New project"
        }
        Button { variant: ButtonVariant::Outline, class: "gap-2 rounded-full",
          "Download"
          svg {
            class: "size-4",
            view_box: "0 0 16 16",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d: "M8 2.5v8M4.5 7.5 8 11l3.5-3.5M3 13.5h10" }
          }
        }
        Button {
          variant: ButtonVariant::Secondary,
          class: "gap-2",
          "aria-busy": saving().to_string(),
          onclick: move |_| saving.toggle(),
          if saving() {
            Spinner { size: SpinnerSize::Sm }
            "Saving..."
          } else {
            "Toggle loading"
          }
        }
      }
      Button {
        variant: ButtonVariant::Outline,
        size: ButtonSize::Sm,
        class: "w-full border-dashed",
        "Add another filter rule"
      }
    }
  }
}
