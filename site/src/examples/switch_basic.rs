use dioxus::prelude::*;
use dioxus_shadcn::{Label, Switch, SwitchSize};

#[component]
pub fn SwitchBasicDemo() -> Element {
  let mut compact = use_signal(|| true);
  let mut wifi = use_signal(|| true);
  let mut focus_mode = use_signal(|| false);

  rsx! {
    div { class: "grid gap-3",
      div { class: "flex items-center gap-2",
        Switch {
          id: "switch-basic-compact",
          size: SwitchSize::Sm,
          checked: compact(),
          on_checked_change: move |checked| compact.set(checked),
        }
        Label { r#for: "switch-basic-compact", "Compact notifications (Sm)" }
      }
      div { class: "flex items-center gap-2",
        Switch {
          id: "switch-basic-wifi",
          checked: wifi(),
          on_checked_change: move |checked| wifi.set(checked),
        }
        Label { r#for: "switch-basic-wifi", "Wi-Fi (Md)" }
      }
      div { class: "flex items-center gap-2",
        Switch {
          id: "switch-basic-focus",
          size: SwitchSize::Lg,
          checked: focus_mode(),
          on_checked_change: move |checked| focus_mode.set(checked),
        }
        Label { r#for: "switch-basic-focus", "Focus mode (Lg)" }
      }
      div { class: "flex items-center gap-2",
        Switch { id: "switch-basic-airplane", checked: true, disabled: true }
        Label { r#for: "switch-basic-airplane", "Airplane mode (managed)" }
      }
    }
  }
}
