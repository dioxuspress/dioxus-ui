use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonSize, ButtonVariant};

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
        Button { variant: ButtonVariant::Soft, class: "gap-2 rounded-full",
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
          loading: saving(),
          onclick: move |_| saving.set(true),
          if saving() { "Saving..." } else { "Save changes" }
        }
        if saving() {
          Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Sm,
            onclick: move |_| saving.set(false),
            "Reset"
          }
        }
      }
      Button {
        variant: ButtonVariant::Dash,
        size: ButtonSize::Sm,
        class: "w-full",
        "Add another filter rule"
      }
    }
  }
}
