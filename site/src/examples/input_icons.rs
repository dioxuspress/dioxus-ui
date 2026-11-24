use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonSize, ButtonVariant, Input, Label};

#[component]
pub fn InputIconsDemo() -> Element {
  let mut search = use_signal(String::new);
  let mut secret = use_signal(|| "dxui-live-token-2026".to_string());
  let mut reveal = use_signal(|| false);

  rsx! {
    div { class: "grid max-w-sm gap-4",
      div { class: "grid gap-2",
        Label { r#for: "input-icons-search", "Quick search" }
        div { class: "relative",
          svg {
            class: "pointer-events-none absolute start-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground",
            view_box: "0 0 16 16",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.8",
            "aria-hidden": "true",
            circle { cx: "7", cy: "7", r: "4.5" }
            path { d: "m10.5 10.5 3 3" }
          }
          Input {
            id: "input-icons-search",
            class: "ps-9",
            placeholder: "Filter components...",
            value: search(),
            on_value_change: move |value| search.set(value),
          }
        }
      }
      div { class: "grid gap-2",
        Label { r#for: "input-icons-secret", "API key" }
        div { class: "flex gap-2",
          Input {
            id: "input-icons-secret",
            r#type: if reveal() { "text" } else { "password" },
            value: secret(),
            on_value_change: move |value| secret.set(value),
          }
          Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Md,
            onclick: move |_| reveal.toggle(),
            if reveal() { "Hide" } else { "Show" }
          }
        }
      }
    }
  }
}
