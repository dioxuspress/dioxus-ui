use dioxus::prelude::*;
use dioxus_shadcn::{
  Avatar, AvatarFallback, Badge, BadgeVariant, Input, Navbar, NavbarEnd, NavbarStart,
};

#[component]
pub fn NavbarSearchDemo() -> Element {
  let mut query = use_signal(String::new);

  rsx! {
    div { class: "w-full overflow-hidden rounded-lg border border-border",
      Navbar { class: "flex-wrap gap-3 py-2",
        NavbarStart { class: "gap-3",
          span { class: "text-sm font-semibold tracking-tight", "Workspace" }
          Badge { variant: BadgeVariant::Secondary, "Pro" }
        }
        NavbarEnd { class: "flex-wrap gap-2",
          Input {
            class: "h-8 w-40 sm:w-52",
            placeholder: "Search docs...",
            "aria-label": "Search documentation",
            value: query(),
            on_value_change: move |value| query.set(value),
          }
          Avatar { class: "h-8 w-8",
            AvatarFallback { class: "text-xs", "AL" }
          }
        }
      }
    }
  }
}
