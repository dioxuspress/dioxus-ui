use dioxus::prelude::*;
use dioxus_shadcn::{Toggle, ToggleSize, ToggleVariant};

#[component]
pub fn ToggleSizesDemo() -> Element {
  let mut sm = use_signal(|| true);
  let mut md = use_signal(|| false);
  let mut lg = use_signal(|| true);

  rsx! {
    div { class: "flex flex-wrap items-center gap-3",
      Toggle {
        size: ToggleSize::Sm,
        variant: ToggleVariant::Default,
        "aria-label": "Compact preview",
        pressed: sm(),
        on_pressed_change: move |next| sm.set(next),
        "Small"
      }
      Toggle {
        size: ToggleSize::Md,
        variant: ToggleVariant::Outline,
        "aria-label": "Standard preview",
        pressed: md(),
        on_pressed_change: move |next| md.set(next),
        "Medium"
      }
      Toggle {
        size: ToggleSize::Lg,
        variant: ToggleVariant::Outline,
        "aria-label": "Large preview",
        pressed: lg(),
        on_pressed_change: move |next| lg.set(next),
        "Large"
      }
    }
  }
}
