use dioxus::prelude::*;
use dioxus_shadcn::{Footer, FooterLayout};

#[component]
pub fn FooterCenteredDemo() -> Element {
  rsx! {
    div { class: "w-full overflow-hidden rounded-lg border border-border",
      Footer { layout: FooterLayout::Centered, class: "gap-4 py-8",
        div { class: "flex flex-wrap items-center justify-center gap-4 text-sm font-medium",
          a { href: "/docs/getting-started", class: "hover:underline", "Docs" }
          a { href: "/components/button", class: "hover:underline", "Components" }
          a { href: "/blocks", class: "hover:underline", "Blocks" }
          a { href: "/docs/theming", class: "hover:underline", "Themes" }
        }
        p { class: "text-xs text-muted-foreground",
          "Built with Dioxus 0.7 and semantic Tailwind v4 tokens."
        }
      }
    }
  }
}
