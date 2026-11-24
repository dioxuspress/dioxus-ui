use dioxus::prelude::*;
use dioxus_shadcn::{Avatar, AvatarFallback};

#[component]
pub fn AvatarSizesDemo() -> Element {
  rsx! {
    div { class: "grid gap-5",
      div { class: "flex flex-wrap items-center gap-3",
        Avatar { class: "h-6 w-6", AvatarFallback { class: "text-[10px]", "XS" } }
        Avatar { class: "h-8 w-8", AvatarFallback { class: "text-xs", "SM" } }
        Avatar { AvatarFallback { "MD" } }
        Avatar { class: "h-12 w-12", AvatarFallback { class: "text-base", "LG" } }
        Avatar { class: "h-16 w-16", AvatarFallback { class: "text-lg", "XL" } }
      }
      div { class: "flex flex-wrap items-center gap-3",
        Avatar { class: "rounded-lg",
          AvatarFallback { class: "rounded-lg bg-primary text-primary-foreground", "DX" }
        }
        Avatar { class: "rounded-md ring-2 ring-primary ring-offset-2 ring-offset-background",
          AvatarFallback { class: "rounded-md bg-secondary text-secondary-foreground", "UI" }
        }
        Avatar { class: "rounded-none",
          AvatarFallback { class: "rounded-none", "SQ" }
        }
      }
    }
  }
}
