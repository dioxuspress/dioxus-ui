use dioxus::prelude::*;
use dioxus_shadcn::{Link, LinkUnderline, LinkVariant};

#[component]
pub fn LinkVariantsDemo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-4 p-4",
      Link { href: "/components/button".to_string(), "Default link" }
      Link { href: "/components/card".to_string(), variant: LinkVariant::Primary, "Primary" }
      Link {
        href: "/components/badge".to_string(),
        variant: LinkVariant::Secondary,
        underline: LinkUnderline::Hover,
        "Hover underline"
      }
      Link {
        href: "/components/alert".to_string(),
        variant: LinkVariant::Success,
        underline: LinkUnderline::None,
        "Success (no underline)"
      }
      Link {
        href: "https://dioxuspress.dev".to_string(),
        variant: LinkVariant::Destructive,
        external: true,
        "External docs"
      }
    }
  }
}
