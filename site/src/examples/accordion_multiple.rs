use dioxus::prelude::*;
use dioxus_shadcn::{
  Accordion, AccordionContent, AccordionItem, AccordionTrigger, Button, ButtonSize, ButtonVariant,
};

const SECTIONS: [(&str, &str, &str); 3] = [
  (
    "install",
    "1. Install the CLI",
    "Run `cargo install dioxus-shadcn-cli` once, then initialize tokens with `dxui init`.",
  ),
  (
    "add",
    "2. Copy components into your app",
    "Use `dxui add button dialog tabs` to copy source files directly into `src/components/ui/`.",
  ),
  (
    "theme",
    "3. Pick a preset theme",
    "Switch between 33 WCAG AA palettes or override CSS custom properties in your stylesheet.",
  ),
];

#[component]
pub fn AccordionMultipleDemo() -> Element {
  let mut open = use_signal(|| vec!["install".to_string(), "add".to_string()]);

  rsx! {
    div { class: "grid max-w-md gap-3",
      div { class: "flex flex-wrap items-center justify-between gap-2",
        span { class: "text-sm text-muted-foreground", "{open().len()} of 3 panels open" }
        div { class: "flex gap-2",
          Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Xs,
            onclick: move |_| {
              open.set(SECTIONS.iter().map(|(id, _, _)| (*id).to_string()).collect());
            },
            "Expand all"
          }
          Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Xs,
            onclick: move |_| open.set(Vec::new()),
            "Collapse all"
          }
        }
      }
      Accordion {
        multiple: true,
        values: Some(open()),
        on_values_change: move |next| open.set(next),
        for (value, title, body) in SECTIONS {
          AccordionItem { key: "{value}", value,
            AccordionTrigger { "{title}" }
            AccordionContent { "{body}" }
          }
        }
      }
    }
  }
}
