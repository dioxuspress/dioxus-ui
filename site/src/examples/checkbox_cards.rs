use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeSize, BadgeStyle, BadgeVariant, Checkbox, Label};

const ADDONS: [(&str, &str, &str, &str); 3] = [
  ("backups", "Automated backups", "Daily encrypted snapshots with 30-day retention.", "$12/mo"),
  ("audit", "Audit logs", "Immutable event trail for every workspace action.", "$19/mo"),
  ("sso", "SAML SSO", "Single sign-on with Okta, Entra ID, or Google Workspace.", "$35/mo"),
];

#[component]
pub fn CheckboxCardsDemo() -> Element {
  let mut selected = use_signal(|| [true, false, false]);
  let enabled_count = selected().into_iter().filter(|checked| *checked).count();

  rsx! {
    div { class: "grid max-w-lg gap-3",
      for (index, (id, title, description, price)) in ADDONS.into_iter().enumerate() {
        div {
          key: "{id}",
          class: if selected()[index] {
            "flex items-start gap-3 rounded-lg border border-primary bg-muted/40 p-4 transition-colors"
          } else {
            "flex items-start gap-3 rounded-lg border border-border bg-card p-4 transition-colors"
          },
          Checkbox {
            id: "checkbox-cards-{id}",
            class: "mt-0.5",
            checked: selected()[index],
            on_checked_change: move |checked| selected.with_mut(|items| items[index] = checked),
          }
          div { class: "grid flex-1 gap-1",
            div { class: "flex flex-wrap items-center justify-between gap-2",
              Label { r#for: "checkbox-cards-{id}", class: "cursor-pointer font-medium", "{title}" }
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
      p { class: "text-sm text-muted-foreground", "{enabled_count} of 3 add-ons enabled" }
    }
  }
}
