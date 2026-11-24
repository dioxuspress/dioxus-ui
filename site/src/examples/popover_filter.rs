use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeSize, BadgeStyle, BadgeVariant, Button, ButtonSize, ButtonVariant, Checkbox, Label,
  Popover, PopoverContent, PopoverDescription, PopoverHeader, PopoverTitle, PopoverTrigger,
  button_class, use_density,
};

const STATUSES: [&str; 3] = ["Active", "Paused", "Draft"];

#[component]
pub fn PopoverFilterDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut checked = use_signal(|| [true, false, true]);

  let active_labels: Vec<&str> = STATUSES
    .into_iter()
    .enumerate()
    .filter_map(|(index, label)| checked()[index].then_some(label))
    .collect();
  let count = active_labels.len();
  let summary = if active_labels.is_empty() {
    "All statuses".to_string()
  } else {
    active_labels.join(", ")
  };

  rsx! {
    div { class: "flex flex-wrap items-center gap-3",
      Popover { open: open(), on_open_change: move |next| open.set(next),
        PopoverTrigger {
          class: button_class(ButtonVariant::Outline, ButtonSize::Sm, use_density(), "gap-2"),
          span { "Filter status" }
          Badge {
            variant: BadgeVariant::Secondary,
            style: BadgeStyle::Soft,
            size: BadgeSize::Xs,
            "{count}"
          }
        }
        PopoverContent { class: "w-64",
          PopoverHeader {
            PopoverTitle { "Filter by status" }
            PopoverDescription { "Choose which records stay visible." }
          }
          div { class: "mt-3 grid gap-2.5",
            for (index, label) in STATUSES.into_iter().enumerate() {
              div { key: "{label}", class: "flex items-center gap-2",
                Checkbox {
                  id: "popover-filter-{label}",
                  checked: checked()[index],
                  on_checked_change: move |next| checked.with_mut(|items| items[index] = next),
                }
                Label { r#for: "popover-filter-{label}", "{label}" }
              }
            }
          }
          div { class: "mt-4 flex items-center justify-between gap-2 border-t border-border pt-3",
            Button {
              variant: ButtonVariant::Ghost,
              size: ButtonSize::Xs,
              onclick: move |_| checked.set([false, false, false]),
              "Reset"
            }
            Button { size: ButtonSize::Xs, onclick: move |_| open.set(false), "Apply" }
          }
        }
      }
      p { class: "text-sm text-muted-foreground", "Showing: {summary}" }
    }
  }
}
