use dioxus::prelude::*;
use dioxus_shadcn::{
  ButtonSize, ButtonVariant, Kbd, KbdSize, Tooltip, TooltipContent, TooltipTrigger, button_class,
  use_density,
};

const TOOLS: [(&str, &str, &str); 4] = [
  ("Bold", "Toggle bold", "⌘B"),
  ("Italic", "Toggle italic", "⌘I"),
  ("Link", "Insert link", "⌘K"),
  ("Save", "Save draft", "⌘S"),
];

#[component]
pub fn TooltipShortcutDemo() -> Element {
  let mut last_action = use_signal(|| "None");
  let trigger_class = button_class(ButtonVariant::Outline, ButtonSize::Sm, use_density(), "");

  rsx! {
    div { class: "grid gap-3",
      div { class: "flex flex-wrap items-center gap-2",
        for (label, description, shortcut) in TOOLS {
          Tooltip { key: "{label}", delay_ms: 150,
            TooltipTrigger {
              class: "{trigger_class}",
              onclick: move |_| last_action.set(label),
              "{label}"
            }
            TooltipContent {
              span { class: "inline-flex items-center gap-2",
                span { "{description}" }
                Kbd {
                  size: KbdSize::Sm,
                  class: "border-primary-foreground/30 bg-primary-foreground/15 text-primary-foreground",
                  "{shortcut}"
                }
              }
            }
          }
        }
      }
      p { class: "text-sm text-muted-foreground", "Last command: {last_action}" }
    }
  }
}
