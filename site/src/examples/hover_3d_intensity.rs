use dioxus::prelude::*;
use dioxus_shadcn::{Card, CardBody, CardTitle, Hover3D, Hover3dIntensity};

#[component]
pub fn Hover3dIntensityDemo() -> Element {
  rsx! {
    div { class: "grid gap-6 p-2 sm:grid-cols-3",
      for (intensity, title, desc) in [
        (Hover3dIntensity::Subtle, "Subtle", "Gentle micro-tilt for dense grids."),
        (Hover3dIntensity::Default, "Default", "Balanced perspective for feature cards."),
        (Hover3dIntensity::Strong, "Strong", "Pronounced depth for hero showcases."),
      ] {
        Hover3D { key: "{title}", intensity,
          Card { class: "h-full bg-card shadow-sm",
            CardBody {
              CardTitle { class: "text-base", "{title}" }
              p { class: "text-xs text-muted-foreground", "{desc}" }
            }
          }
        }
      }
    }
  }
}
