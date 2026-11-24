use dioxus::prelude::*;
use dioxus_shadcn::{Label, NavigationOrientation, RadioGroup, RadioGroupItem};

const CADENCES: [(&str, &str); 3] = [
  ("monthly", "Monthly"),
  ("quarterly", "Quarterly"),
  ("annual", "Annual (save 20%)"),
];

#[component]
pub fn RadioGroupHorizontalDemo() -> Element {
  let mut cadence = use_signal(|| "annual".to_string());

  rsx! {
    div { class: "grid gap-3",
      RadioGroup {
        orientation: NavigationOrientation::Horizontal,
        class: "flex flex-wrap gap-4",
        "aria-label": "Billing cadence",
        value: Some(cadence()),
        on_value_change: move |value| cadence.set(value),
        for (value, label) in CADENCES {
          div { key: "{value}", class: "flex items-center gap-2",
            RadioGroupItem { id: "radio-horizontal-{value}", value }
            Label { r#for: "radio-horizontal-{value}", "{label}" }
          }
        }
      }
      p { class: "text-sm text-muted-foreground", "Billing cycle: {cadence}" }
    }
  }
}
