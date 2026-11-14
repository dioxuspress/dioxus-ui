use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardActions, CardActionsJustify,
  CardBody, CardDescription, CardLayout, CardMedia, CardSize, CardTitle, CardVariant,
};

#[component]
pub fn CardMediaDemo() -> Element {
  rsx! {
    div { class: "grid gap-4 md:grid-cols-2",
      Card { variant: CardVariant::Border, size: CardSize::Sm,
        CardMedia {
          svg {
            view_box: "0 0 400 180",
            class: "h-36 w-full bg-muted object-cover",
            rect { width: "400", height: "180", fill: "currentColor", class: "text-primary/15" }
            circle { cx: "110", cy: "90", r: "48", fill: "currentColor", class: "text-primary/35" }
            rect { x: "185", y: "48", width: "145", height: "84", rx: "12", fill: "currentColor", class: "text-primary/50" }
          }
        }
        CardBody {
          div { class: "flex items-center gap-2",
            CardTitle { "Running Shoes" }
            Badge { variant: BadgeVariant::Secondary, "NEW" }
          }
          CardDescription { "Lightweight mesh upper with responsive foam cushioning." }
          CardActions { justify: CardActionsJustify::End,
            Button { size: ButtonSize::Sm, "Buy Now" }
          }
        }
      }

      Card { variant: CardVariant::Border, size: CardSize::Sm,
        CardMedia { inset: true,
          svg {
            view_box: "0 0 400 180",
            class: "h-36 w-full rounded-md bg-muted object-cover",
            rect { width: "400", height: "180", rx: "12", fill: "currentColor", class: "text-secondary" }
            polygon { points: "200,28 290,148 110,148", fill: "currentColor", class: "text-primary/45" }
          }
        }
        CardBody { class: "items-center text-center",
          CardTitle { "Centered Inset" }
          CardDescription { "Padded media frame paired with centered title and actions." }
          CardActions { justify: CardActionsJustify::Center,
            Button { size: ButtonSize::Sm, "Explore" }
          }
        }
      }

      Card { variant: CardVariant::Border, size: CardSize::Sm, layout: CardLayout::Side,
        CardMedia {
          svg {
            view_box: "0 0 220 220",
            class: "h-40 w-full bg-muted object-cover sm:h-full",
            rect { width: "220", height: "220", fill: "currentColor", class: "text-primary/15" }
            circle { cx: "110", cy: "110", r: "56", fill: "currentColor", class: "text-primary/45" }
          }
        }
        CardBody {
          CardTitle { "Side-by-side" }
          CardDescription { "Horizontal layout with media pinned on the leading edge." }
          CardActions { justify: CardActionsJustify::End,
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Watch" }
          }
        }
      }

      Card { size: CardSize::Sm, layout: CardLayout::ImageFull, class: "min-h-44",
        CardMedia {
          svg {
            view_box: "0 0 400 220",
            class: "h-full w-full object-cover",
            rect { width: "400", height: "220", fill: "#1e293b" }
            circle { cx: "310", cy: "65", r: "78", fill: "#38bdf8", opacity: "0.35" }
            circle { cx: "100", cy: "165", r: "92", fill: "#818cf8", opacity: "0.35" }
          }
        }
        CardBody { class: "justify-between",
          div { class: "space-y-1.5",
            CardTitle { "Image Overlay" }
            CardDescription { "Full-bleed background media with automatic contrast scrim." }
          }
          CardActions { justify: CardActionsJustify::End,
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Book Now" }
          }
        }
      }
    }
  }
}
