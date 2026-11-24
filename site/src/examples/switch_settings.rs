use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeSize, BadgeStyle, BadgeVariant, Label, Switch};

const PREFERENCES: [(&str, &str, &str); 3] = [
  ("releases", "Release notes", "Receive a summary when a new crate version ships."),
  ("mentions", "Mentions and replies", "Notify immediately when teammates mention you."),
  ("digest", "Weekly digest", "Monday morning roll-up of workspace activity."),
];

#[component]
pub fn SwitchSettingsDemo() -> Element {
  let mut enabled = use_signal(|| [true, true, false]);
  let active = enabled().into_iter().filter(|state| *state).count();

  rsx! {
    div { class: "grid max-w-lg divide-y divide-border rounded-lg border border-border bg-card",
      for (index, (id, title, description)) in PREFERENCES.into_iter().enumerate() {
        div { key: "{id}", class: "flex items-center justify-between gap-4 p-4",
          div { class: "grid gap-1",
            Label { r#for: "switch-settings-{id}", class: "font-medium", "{title}" }
            p { class: "text-sm text-muted-foreground", "{description}" }
          }
          Switch {
            id: "switch-settings-{id}",
            checked: enabled()[index],
            on_checked_change: move |checked| enabled.with_mut(|items| items[index] = checked),
          }
        }
      }
      div { class: "flex items-center justify-between gap-4 p-4",
        div { class: "grid gap-1",
          div { class: "flex items-center gap-2",
            Label { r#for: "switch-settings-security", class: "font-medium", "Security alerts" }
            Badge {
              variant: BadgeVariant::Secondary,
              style: BadgeStyle::Outline,
              size: BadgeSize::Xs,
              "Required"
            }
          }
          p { class: "text-sm text-muted-foreground",
            "Always enabled for sign-in anomalies and token rotations."
          }
        }
        Switch { id: "switch-settings-security", checked: true, disabled: true }
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "{active} optional channels active" }
  }
}
