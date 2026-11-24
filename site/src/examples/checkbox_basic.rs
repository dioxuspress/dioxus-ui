use dioxus::prelude::*;
use dioxus_shadcn::{Checkbox, CheckboxSize, Label};

#[component]
pub fn CheckboxBasicDemo() -> Element {
  let mut terms = use_signal(|| true);
  let mut compact = use_signal(|| true);
  let mut prominent = use_signal(|| false);
  let mut items = use_signal(|| [true, false]);
  let all = items().iter().all(|item| *item);
  let some = items().iter().any(|item| *item);

  rsx! {
    div { class: "grid gap-3",
      div { class: "flex flex-wrap items-center gap-5",
        div { class: "flex items-center gap-2",
          Checkbox {
            id: "checkbox-basic-sm",
            size: CheckboxSize::Sm,
            checked: compact(),
            on_checked_change: move |checked| compact.set(checked),
          }
          Label { r#for: "checkbox-basic-sm", "Small" }
        }
        div { class: "flex items-center gap-2",
          Checkbox {
            id: "checkbox-basic-terms",
            checked: terms(),
            on_checked_change: move |checked| terms.set(checked),
          }
          Label { r#for: "checkbox-basic-terms", "Default (Md)" }
        }
        div { class: "flex items-center gap-2",
          Checkbox {
            id: "checkbox-basic-lg",
            size: CheckboxSize::Lg,
            checked: prominent(),
            on_checked_change: move |checked| prominent.set(checked),
          }
          Label { r#for: "checkbox-basic-lg", "Large" }
        }
      }
      div { class: "flex items-center gap-2",
        Checkbox { id: "checkbox-basic-disabled", disabled: true }
        Label { r#for: "checkbox-basic-disabled", "Disabled" }
      }
      div { class: "flex items-center gap-2",
        Checkbox {
          id: "checkbox-basic-all",
          checked: all,
          indeterminate: some && !all,
          on_checked_change: move |checked| items.set([checked, checked]),
        }
        Label { r#for: "checkbox-basic-all", "Select all" }
      }
      for (index, name) in ["Email", "Push"].into_iter().enumerate() {
        div { key: "{name}", class: "ml-6 flex items-center gap-2",
          Checkbox {
            id: "checkbox-basic-{name}",
            checked: items()[index],
            on_checked_change: move |checked| items.with_mut(|items| items[index] = checked),
          }
          Label { r#for: "checkbox-basic-{name}", "{name}" }
        }
      }
    }
  }
}
