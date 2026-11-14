use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardActions, CardActionsJustify,
  CardBody, CardDescription, CardSize, CardTitle, CardVariant,
};

#[component]
pub fn CardVariantsDemo() -> Element {
  rsx! {
    div { class: "grid gap-4 sm:grid-cols-2",
      Card { variant: CardVariant::Border, size: CardSize::Sm,
        CardBody {
          div { class: "flex items-center justify-between gap-2",
            CardTitle { "Bordered card" }
            Badge { variant: BadgeVariant::Outline, "Border" }
          }
          CardDescription { "Crisp 1px border without drop shadow elevation." }
          CardActions { justify: CardActionsJustify::End,
            Button { size: ButtonSize::Sm, "Inspect" }
          }
        }
      }
      Card { variant: CardVariant::Dash, size: CardSize::Sm,
        CardBody {
          div { class: "flex items-center justify-between gap-2",
            CardTitle { "Dashed card" }
            Badge { variant: BadgeVariant::Secondary, "Dash" }
          }
          CardDescription { "Useful for placeholders, drafts, or drop-zone slots." }
          CardActions { justify: CardActionsJustify::End,
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Configure" }
          }
        }
      }
      Card { variant: CardVariant::Primary, size: CardSize::Sm,
        CardBody {
          CardTitle { "Primary surface" }
          CardDescription { "High-emphasis container for featured actions or promos." }
          CardActions { justify: CardActionsJustify::End,
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Upgrade" }
          }
        }
      }
      Card { variant: CardVariant::Muted, size: CardSize::Xs,
        CardBody {
          CardTitle { "Compact xs card" }
          CardDescription { "Tight padding for dense sidebars and summary strips." }
          CardActions { justify: CardActionsJustify::Between,
            span { class: "text-xs text-muted-foreground", "Updated 2m ago" }
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Open" }
          }
        }
      }
    }
  }
}
