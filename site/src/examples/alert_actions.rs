use dioxus::prelude::*;
use dioxus_shadcn::{
  Alert, AlertDescription, AlertTitle, AlertVariant, Button, ButtonSize, ButtonVariant,
};

#[component]
pub fn AlertActionsDemo() -> Element {
  let mut dismissed = use_signal(|| false);
  let mut retried = use_signal(|| 0_u32);

  rsx! {
    div { class: "grid max-w-lg gap-4",
      if dismissed() {
        Button {
          variant: ButtonVariant::Outline,
          size: ButtonSize::Sm,
          class: "w-fit",
          onclick: move |_| dismissed.set(false),
          "Show update notice"
        }
      } else {
        Alert { variant: AlertVariant::Info,
          div { class: "flex flex-wrap items-start justify-between gap-3",
            div { class: "flex items-start gap-3",
              svg {
                class: "mt-0.5 size-4 shrink-0 text-foreground",
                view_box: "0 0 16 16",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                "aria-hidden": "true",
                circle { cx: "8", cy: "8", r: "6.5" }
                path { d: "M8 7v4M8 4.8h.01" }
              }
              div {
                AlertTitle { "New CLI version available" }
                AlertDescription { variant: AlertVariant::Info,
                  "Run dxui diff to review updated component templates."
                }
              }
            }
            Button {
              variant: ButtonVariant::Outline,
              size: ButtonSize::Sm,
              onclick: move |_| dismissed.set(true),
              "Dismiss"
            }
          }
        }
      }
      Alert { variant: AlertVariant::Warning,
        div { class: "flex flex-wrap items-center justify-between gap-3",
          div {
            AlertTitle { "Sync paused" }
            AlertDescription { variant: AlertVariant::Warning,
              if retried() == 0 {
                "Could not reach the remote cache."
              } else {
                "Retry attempt #{retried} queued."
              }
            }
          }
          Button {
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Sm,
            onclick: move |_| retried += 1,
            "Retry"
          }
        }
      }
    }
  }
}
