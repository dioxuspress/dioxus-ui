use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, Navbar, NavbarCenter, NavbarEnd, NavbarStart,
};

#[component]
pub fn NavbarSurfaceDemo() -> Element {
  rsx! {
    div { class: "grid w-full gap-4",
      div { class: "w-full overflow-hidden rounded-lg border border-border",
        Navbar { class: "flex-wrap border-transparent bg-primary text-primary-foreground",
          NavbarStart {
            span { class: "text-sm font-semibold tracking-tight", "Dioxus Cloud" }
          }
          NavbarCenter { class: "hidden gap-4 text-sm sm:flex",
            a { href: "/components", class: "underline-offset-4 hover:underline", "Overview" }
            a { href: "/blocks", class: "underline-offset-4 hover:underline", "Deployments" }
          }
          NavbarEnd {
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "New app" }
          }
        }
      }
      div { class: "w-full overflow-hidden rounded-lg border border-border",
        Navbar { class: "flex-wrap bg-muted/50",
          NavbarStart {
            span { class: "text-sm font-semibold tracking-tight text-foreground", "Release Notes" }
          }
          NavbarEnd { class: "gap-2",
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Changelog" }
            Button { size: ButtonSize::Sm, "Subscribe" }
          }
        }
      }
    }
  }
}
