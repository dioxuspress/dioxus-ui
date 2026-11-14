use dioxus::prelude::*;
use dioxus_shadcn::{Footer, FooterLayout, FooterNav, FooterTitle};

#[component]
pub fn FooterColumnsDemo() -> Element {
  rsx! {
    div { class: "w-full overflow-hidden rounded-lg border border-border",
      Footer { layout: FooterLayout::Columns,
        FooterNav {
          FooterTitle { "Components" }
          a { href: "/components/navbar", class: "hover:underline", "Navbar" }
          a { href: "/components/hero", class: "hover:underline", "Hero" }
          a { href: "/components/footer", class: "hover:underline", "Footer" }
        }
        FooterNav {
          FooterTitle { "Ecosystem" }
          a { href: "/themes", class: "hover:underline", "Themes" }
          a { href: "/blocks", class: "hover:underline", "Blocks" }
        }
        FooterNav {
          FooterTitle { "Community" }
          a { href: "/docs", class: "hover:underline", "Documentation" }
          a { href: "/icons", class: "hover:underline", "Icons" }
        }
      }
    }
  }
}
