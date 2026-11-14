use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonVariant, Hero, HeroContent, HeroOverlay};

#[component]
pub fn HeroShowcaseDemo() -> Element {
  rsx! {
    Hero {
      HeroOverlay {}
      HeroContent {
        div { class: "max-w-lg space-y-4",
          h2 { class: "text-2xl font-bold tracking-tight", "Build Full-Stack Rust UIs" }
          p { class: "text-sm text-muted-foreground",
            "Composable Dioxus 0.7 primitives and Tailwind v4 theme tokens ready for SSR and WASM."
          }
          div { class: "flex flex-wrap justify-center gap-3 md:justify-start",
            Button { "Get Started" }
            Button { variant: ButtonVariant::Outline, "Browse Components" }
          }
        }
      }
    }
  }
}
