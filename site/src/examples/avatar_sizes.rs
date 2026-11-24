use dioxus::prelude::*;
use dioxus_shadcn::{Avatar, AvatarFallback, AvatarShape, AvatarSize};

#[component]
pub fn AvatarSizesDemo() -> Element {
  rsx! {
    div { class: "grid gap-5",
      div { class: "flex flex-wrap items-center gap-3",
        Avatar { size: AvatarSize::Xs, AvatarFallback { "XS" } }
        Avatar { size: AvatarSize::Sm, AvatarFallback { "SM" } }
        Avatar { size: AvatarSize::Md, AvatarFallback { "MD" } }
        Avatar { size: AvatarSize::Lg, AvatarFallback { "LG" } }
        Avatar { size: AvatarSize::Xl, AvatarFallback { "XL" } }
      }
      div { class: "flex flex-wrap items-center gap-3",
        Avatar { shape: AvatarShape::Square,
          AvatarFallback { class: "bg-primary text-primary-foreground", "DX" }
        }
        Avatar {
          shape: AvatarShape::Square,
          class: "ring-2 ring-primary ring-offset-2 ring-offset-background",
          AvatarFallback { class: "bg-secondary text-secondary-foreground", "UI" }
        }
        Avatar { size: AvatarSize::Lg, shape: AvatarShape::Square,
          AvatarFallback { "SQ" }
        }
      }
    }
  }
}
