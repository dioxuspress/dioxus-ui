use dioxus::prelude::*;
use dioxus_shadcn::{Avatar, AvatarFallback, AvatarGroup};

#[component]
pub fn AvatarGroupDemo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-8",
      AvatarGroup {
        for (initials, tone) in [
          ("AL", "bg-primary text-primary-foreground"),
          ("GK", "bg-secondary text-secondary-foreground"),
          ("MN", "bg-muted text-foreground"),
          ("+4", "bg-muted text-foreground text-xs"),
        ] {
          Avatar { key: "{initials}",
            AvatarFallback { class: tone, "{initials}" }
          }
        }
      }
      div { class: "flex items-center gap-4",
        div { class: "relative inline-flex",
          Avatar { AvatarFallback { "ON" } }
          span {
            class: "absolute end-0 bottom-0 size-3 rounded-full border-2 border-background bg-success",
            "aria-hidden": "true",
          }
          span { class: "sr-only", "Online" }
        }
        div { class: "relative inline-flex",
          Avatar { AvatarFallback { "AW" } }
          span {
            class: "absolute end-0 bottom-0 size-3 rounded-full border-2 border-background bg-warning",
            "aria-hidden": "true",
          }
          span { class: "sr-only", "Away" }
        }
        div { class: "relative inline-flex",
          Avatar { AvatarFallback { "OF" } }
          span {
            class: "absolute end-0 bottom-0 size-3 rounded-full border-2 border-background bg-muted-foreground",
            "aria-hidden": "true",
          }
          span { class: "sr-only", "Offline" }
        }
      }
    }
  }
}
