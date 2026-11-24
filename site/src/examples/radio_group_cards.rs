use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeSize, BadgeStyle, BadgeVariant, Label, RadioGroup, RadioGroupItem};

const TIERS: [(&str, &str, &str, &str); 3] = [
  ("starter", "Starter", "Up to 3 projects and community support.", "$0"),
  ("pro", "Pro", "Unlimited projects, preview builds, and priority email.", "$24/mo"),
  ("scale", "Scale", "Dedicated clusters, custom SLAs, and SAML SSO.", "$99/mo"),
];

#[component]
pub fn RadioGroupCardsDemo() -> Element {
  let mut selected = use_signal(|| "pro".to_string());

  rsx! {
    div { class: "grid max-w-lg gap-3",
      RadioGroup {
        class: "grid gap-3",
        "aria-label": "Subscription tier",
        value: Some(selected()),
        on_value_change: move |value| selected.set(value),
        for (value, title, description, price) in TIERS {
          div {
            key: "{value}",
            class: if selected() == value {
              "flex items-start gap-3 rounded-lg border border-primary bg-muted/40 p-4 transition-colors"
            } else {
              "flex items-start gap-3 rounded-lg border border-border bg-card p-4 transition-colors"
            },
            RadioGroupItem {
              id: "radio-cards-{value}",
              class: "mt-0.5",
              value,
            }
            div { class: "grid flex-1 gap-1",
              div { class: "flex flex-wrap items-center justify-between gap-2",
                Label { r#for: "radio-cards-{value}", class: "cursor-pointer font-medium", "{title}" }
                Badge {
                  variant: BadgeVariant::Secondary,
                  style: BadgeStyle::Soft,
                  size: BadgeSize::Sm,
                  "{price}"
                }
              }
              p { class: "text-sm text-muted-foreground", "{description}" }
            }
          }
        }
      }
      p { class: "text-sm text-muted-foreground", "Selected tier: {selected}" }
    }
  }
}
