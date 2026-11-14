use dioxus::prelude::*;
use dioxus_shadcn::{Aura, AuraSize, AuraTone, Card, CardBody, CardTitle};

#[component]
pub fn AuraSpotlightDemo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-8 p-6",
      Aura { tone: AuraTone::Primary, size: AuraSize::Md,
        Card { class: "w-64 bg-card/90 backdrop-blur",
          CardBody {
            CardTitle { class: "text-base", "Primary Aura" }
            p { class: "text-sm text-muted-foreground", "Soft semantic glow backdrop." }
          }
        }
      }
      Aura { tone: AuraTone::Accent, size: AuraSize::Lg,
        Card { class: "w-64 bg-card/90 backdrop-blur",
          CardBody {
            CardTitle { class: "text-base", "Accent Aura" }
            p { class: "text-sm text-muted-foreground", "Adapts to every theme preset." }
          }
        }
      }
    }
  }
}
