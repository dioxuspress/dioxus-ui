use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, Footer, FooterLayout, FooterNav, FooterTitle, Input, Label,
};

#[component]
pub fn FooterNewsletterDemo() -> Element {
  let mut email = use_signal(String::new);
  let mut subscribed = use_signal(|| false);

  rsx! {
    div { class: "w-full overflow-hidden rounded-lg border border-border",
      Footer { layout: FooterLayout::Columns,
        FooterNav {
          FooterTitle { "Product" }
          a { href: "/components/card", class: "hover:underline", "Cards" }
          a { href: "/blocks", class: "hover:underline", "Page blocks" }
          a { href: "/docs/theming", class: "hover:underline", "Theme presets" }
        }
        FooterNav {
          FooterTitle { "Resources" }
          a { href: "/docs/getting-started", class: "hover:underline", "Installation" }
          a { href: "/components/button", class: "hover:underline", "API reference" }
        }
        FooterNav { class: "w-full max-w-xs",
          FooterTitle { "Newsletter" }
          Label { r#for: "footer-newsletter-email", class: "text-xs text-muted-foreground",
            "Get release notes when new components ship."
          }
          div { class: "mt-1 flex flex-wrap gap-2 sm:flex-nowrap",
            Input {
              id: "footer-newsletter-email",
              r#type: "email",
              class: "h-8 min-w-0 flex-1",
              placeholder: "you@example.com",
              value: email(),
              on_value_change: move |value| {
                email.set(value);
                subscribed.set(false);
              },
            }
            Button {
              size: ButtonSize::Sm,
              onclick: move |_| subscribed.set(true),
              if subscribed() { "Joined" } else { "Join" }
            }
          }
        }
      }
    }
  }
}
