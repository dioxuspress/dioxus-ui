use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeVariant, Button, ButtonSize, ButtonVariant};

const DEFAULT_TAGS: [&str; 3] = ["Rust", "Dioxus 0.7", "Tailwind v4"];

#[component]
pub fn BadgeCompositionDemo() -> Element {
  let mut tags = use_signal(|| DEFAULT_TAGS.iter().map(|tag| (*tag).to_string()).collect::<Vec<_>>());

  rsx! {
    div { class: "grid gap-4",
      div { class: "flex flex-wrap items-center gap-2",
        Badge { variant: BadgeVariant::Outline, class: "gap-1.5 rounded-full px-2.5",
          span { class: "size-1.5 rounded-full bg-success", "aria-hidden": "true" }
          "Operational"
        }
        Badge { variant: BadgeVariant::Outline, class: "gap-1.5 rounded-full px-2.5",
          span { class: "size-1.5 rounded-full bg-warning", "aria-hidden": "true" }
          "Degraded"
        }
        Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, class: "gap-2",
          "Inbox"
          Badge { variant: BadgeVariant::Secondary, class: "rounded-full px-1.5", "9+" }
        }
      }
      div { class: "flex flex-wrap items-center gap-2",
        for tag in tags() {
          Badge { key: "{tag}", variant: BadgeVariant::Secondary, class: "gap-1.5 pe-1",
            span { "{tag}" }
            button {
              r#type: "button",
              class: "inline-flex size-4 items-center justify-center rounded-sm hover:bg-background/60 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
              "aria-label": "Remove {tag}",
              onclick: move |_| {
                let target = tag.clone();
                tags.write().retain(|item| item != &target);
              },
              "×"
            }
          }
        }
        if tags().is_empty() {
          Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Sm,
            onclick: move |_| tags.set(DEFAULT_TAGS.iter().map(|tag| (*tag).to_string()).collect()),
            "Reset tags"
          }
        }
      }
    }
  }
}
