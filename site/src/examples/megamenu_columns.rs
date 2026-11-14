use dioxus::prelude::*;
use dioxus_shadcn::{
  Megamenu, MegamenuColumn, MegamenuContent, MegamenuHeading, MegamenuLink, MegamenuTrigger,
};

#[component]
pub fn MegamenuColumnsDemo() -> Element {
  rsx! {
    div { class: "flex min-h-48 items-start p-4",
      Megamenu {
        MegamenuTrigger { "Explore Components" }
        MegamenuContent {
          MegamenuColumn {
            MegamenuHeading { "Navigation" }
            MegamenuLink { href: "/components/navbar".to_string(), "Navbar" }
            MegamenuLink { href: "/components/megamenu".to_string(), "Megamenu" }
            MegamenuLink { href: "/components/link".to_string(), "Link" }
          }
          MegamenuColumn {
            MegamenuHeading { "Layout" }
            MegamenuLink { href: "/components/hero".to_string(), "Hero" }
            MegamenuLink { href: "/components/footer".to_string(), "Footer" }
            MegamenuLink { href: "/components/join".to_string(), "Join" }
          }
          MegamenuColumn {
            MegamenuHeading { "Forms" }
            MegamenuLink { href: "/components/filter".to_string(), "Filter" }
            MegamenuLink { href: "/components/validator".to_string(), "Validator" }
          }
        }
      }
    }
  }
}
