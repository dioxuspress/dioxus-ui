use dioxus::prelude::*;
use dioxus_shadcn::{Aura, AuraSize, AuraTone, Card, CardBody, CardTitle};

#[component]
pub fn AuraTonesDemo() -> Element {
  rsx! {
    div { class: "grid gap-6 p-4 sm:grid-cols-2",
      Aura { tone: AuraTone::Secondary, size: AuraSize::Sm,
        Card { class: "bg-card/90 backdrop-blur",
          CardBody {
            CardTitle { class: "text-base", "Secondary · Sm" }
            p { class: "text-sm text-muted-foreground", "Tight ambient halo for compact callouts." }
          }
        }
      }
      Aura { tone: AuraTone::Destructive, size: AuraSize::Md,
        Card { class: "bg-card/90 backdrop-blur",
          CardBody {
            CardTitle { class: "text-base", "Destructive · Md" }
            p { class: "text-sm text-muted-foreground", "Highlights critical incident banners." }
          }
        }
      }
    }
  }
}
