use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, Navbar, NavbarCenter, NavbarEnd, NavbarStart,
};

#[component]
pub fn NavbarSlotsDemo() -> Element {
  rsx! {
    div { class: "w-full overflow-hidden rounded-lg border border-border",
      Navbar {
        NavbarStart {
          span { class: "text-sm font-semibold tracking-tight", "DioxusPress" }
        }
        NavbarCenter {
          div { class: "flex items-center gap-4 text-sm text-muted-foreground",
            a { href: "/components", class: "hover:text-foreground", "Components" }
            a { href: "/themes", class: "hover:text-foreground", "Themes" }
            a { href: "/docs", class: "hover:text-foreground", "Docs" }
          }
        }
        NavbarEnd {
          Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Sign in" }
        }
      }
    }
  }
}
