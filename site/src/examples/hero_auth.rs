use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, Card, CardContent, CardDescription, CardHeader, CardTitle, Hero, HeroContent, Input,
  Label,
};

#[component]
pub fn HeroAuthDemo() -> Element {
  let mut email = use_signal(|| "ada@dioxuspress.dev".to_string());
  let mut status = use_signal(|| Option::<String>::None);

  rsx! {
    Hero { class: "border border-border bg-muted/30",
      HeroContent { class: "w-full justify-between gap-8 p-6 md:flex-row md:p-8",
        div { class: "max-w-sm space-y-3 text-left",
          h2 { class: "text-2xl font-bold tracking-tight text-foreground", "Welcome back" }
          p { class: "text-sm leading-6 text-muted-foreground",
            "Sign in to sync your saved themes, component snapshots, and team workspaces across devices."
          }
          ul { class: "space-y-1.5 text-sm text-muted-foreground",
            li { "• Zero-eval CSP compliance" }
            li { "• WCAG AA checked presets" }
            li { "• Full keyboard roving focus" }
          }
        }
        Card { class: "w-full max-w-sm text-left",
          CardHeader {
            CardTitle { "Sign in" }
            CardDescription { "Use your workspace email to continue." }
          }
          CardContent { class: "grid gap-4",
            div { class: "grid gap-1.5",
              Label { r#for: "hero-auth-email", "Email" }
              Input {
                id: "hero-auth-email",
                r#type: "email",
                value: email(),
                on_value_change: move |value| email.set(value),
              }
            }
            div { class: "grid gap-1.5",
              Label { r#for: "hero-auth-password", "Password" }
              Input {
                id: "hero-auth-password",
                r#type: "password",
                placeholder: "••••••••",
              }
            }
            Button {
              class: "w-full",
              onclick: move |_| status.set(Some(format!("Signed in as {}", email()))),
              "Continue"
            }
            if let Some(message) = status() {
              p { class: "text-xs text-muted-foreground", role: "status", "{message}" }
            }
          }
        }
      }
    }
  }
}
