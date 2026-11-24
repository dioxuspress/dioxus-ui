use dioxus::prelude::*;
use dioxus_shadcn::{
  Accordion, AccordionContent, AccordionItem, AccordionTrigger, Badge, BadgeSize, BadgeStyle,
  BadgeVariant,
};

const CHECKS: [(&str, &str, &str, BadgeVariant, &str); 3] = [
  (
    "tokens",
    "Semantic color tokens",
    "Verified",
    BadgeVariant::Success,
    "All 33 built-in presets map cleanly to `--background`, `--foreground`, and surface pairs.",
  ),
  (
    "csp",
    "Strict CSP compliance",
    "Zero eval",
    BadgeVariant::Info,
    "No inline event strings or runtime `eval()`; safe under `script-src 'self' 'wasm-unsafe-eval'`.",
  ),
  (
    "density",
    "Touch density floor",
    "44px target",
    BadgeVariant::Secondary,
    "Wrap any subtree in `DensityProvider` to enforce 44×44 CSS pixel hit targets on touch screens.",
  ),
];

#[component]
pub fn AccordionBorderedDemo() -> Element {
  rsx! {
    Accordion { class: "grid max-w-lg gap-2", default_value: "tokens",
      for (value, title, tag, variant, body) in CHECKS {
        AccordionItem {
          key: "{value}",
          value,
          class: "rounded-lg border border-border bg-card px-4",
          AccordionTrigger {
            span { class: "flex flex-wrap items-center gap-2",
              span { "{title}" }
              Badge { variant, style: BadgeStyle::Soft, size: BadgeSize::Xs, "{tag}" }
            }
          }
          AccordionContent { "{body}" }
        }
      }
    }
  }
}
