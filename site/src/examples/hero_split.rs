use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeVariant, Button, ButtonVariant, Hero, HeroContent};

#[component]
pub fn HeroSplitDemo() -> Element {
  rsx! {
    Hero { class: "border border-border bg-card",
      HeroContent { class: "w-full justify-between gap-8 p-8 md:flex-row",
        div { class: "max-w-md space-y-4 text-left",
          Badge { variant: BadgeVariant::Secondary, "v0.6 · Dioxus 0.7" }
          h2 { class: "text-2xl font-bold tracking-tight text-foreground",
            "Ship accessible Rust UI without leaving Cargo"
          }
          p { class: "text-sm leading-6 text-muted-foreground",
            "Pair headless keyboard primitives with Tailwind v4 theme tokens that render identically across SSR, WASM, and desktop."
          }
          div { class: "flex flex-wrap gap-3",
            Button { "Start building" }
            Button { variant: ButtonVariant::Outline, "Read docs" }
          }
        }
        div {
          class: "w-full max-w-xs rounded-lg border border-border bg-background p-4 text-left shadow-sm",
          "aria-hidden": "true",
          div { class: "flex items-center gap-1.5 border-b border-border pb-3",
            span { class: "size-2.5 rounded-full bg-destructive" }
            span { class: "size-2.5 rounded-full bg-warning" }
            span { class: "size-2.5 rounded-full bg-success" }
            span { class: "ms-2 text-xs text-muted-foreground", "dxui add hero" }
          }
          div { class: "mt-3 space-y-2",
            div { class: "h-3 w-3/4 rounded bg-muted" }
            div { class: "h-3 w-full rounded bg-muted" }
            div { class: "h-3 w-5/6 rounded bg-muted" }
          }
          div { class: "mt-4 grid grid-cols-2 gap-2",
            div { class: "rounded-md border border-border bg-muted/50 p-2.5",
              p { class: "text-xs text-muted-foreground", "Components" }
              p { class: "text-lg font-bold text-foreground", "100" }
            }
            div { class: "rounded-md border border-border bg-muted/50 p-2.5",
              p { class: "text-xs text-muted-foreground", "Presets" }
              p { class: "text-lg font-bold text-foreground", "33" }
            }
          }
        }
      }
    }
  }
}
