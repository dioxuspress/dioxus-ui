use dioxus::prelude::*;
use dioxus_shadcn::{Mask, MaskHalf, MaskShape};

#[component]
pub fn MaskShapesDemo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-4 p-2",
      Mask { shape: MaskShape::Squircle, class: "size-16 bg-primary text-primary-foreground grid place-items-center text-xs font-medium",
        "Squircle"
      }
      Mask { shape: MaskShape::Heart, class: "size-16 bg-destructive text-destructive-foreground grid place-items-center text-xs font-medium",
        "Heart"
      }
      Mask { shape: MaskShape::Hexagon, class: "size-16 bg-secondary grid place-items-center text-xs font-medium text-secondary-foreground",
        "Hex"
      }
      Mask { shape: MaskShape::Star2, class: "size-16 bg-accent grid place-items-center text-xs font-medium text-accent-foreground",
        "Star"
      }
      Mask { shape: MaskShape::Star, half: MaskHalf::First, class: "size-16 bg-primary grid place-items-center text-xs font-medium text-primary-foreground",
        ""
      }
    }
  }
}
