use dioxus::prelude::*;
use dioxus_shadcn::{Card, CardBody, CardTitle, Hover3D, Hover3dIntensity};

#[component]
pub fn Hover3dCardDemo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-6 p-4",
      Hover3D { intensity: Hover3dIntensity::Default, class: "w-72",
        Card { class: "bg-card shadow-md",
          CardBody {
            CardTitle { "3D Perspective Card" }
            p { class: "text-sm text-muted-foreground",
              "Move your pointer across the card to tilt it in 3D space."
            }
          }
        }
      }
    }
  }
}
