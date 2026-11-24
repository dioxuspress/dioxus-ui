use dioxus::prelude::*;
use dioxus_shadcn::{Mask, MaskHalf, MaskShape};

#[component]
pub fn MaskShapesGalleryDemo() -> Element {
  rsx! {
    div { class: "grid gap-4 p-2",
      div { class: "flex flex-wrap items-center gap-3",
        for (shape, label) in [
          (MaskShape::Circle, "Circle"),
          (MaskShape::Diamond, "Diamond"),
          (MaskShape::Pentagon, "Penta"),
          (MaskShape::Hexagon2, "Hex 2"),
          (MaskShape::Decagon, "Deca"),
          (MaskShape::Triangle, "Tri"),
        ] {
          Mask {
            key: "{label}",
            shape,
            class: "size-16 bg-secondary text-secondary-foreground grid place-items-center text-xs font-medium",
            "{label}"
          }
        }
      }
      div { class: "flex items-center gap-4",
        span { class: "text-xs text-muted-foreground", "Half masks:" }
        div { class: "flex items-center gap-1",
          Mask {
            shape: MaskShape::Star2,
            half: MaskHalf::First,
            class: "size-12 bg-primary text-primary-foreground",
            ""
          }
          Mask {
            shape: MaskShape::Star2,
            half: MaskHalf::Second,
            class: "size-12 bg-muted text-foreground",
            ""
          }
        }
      }
    }
  }
}
