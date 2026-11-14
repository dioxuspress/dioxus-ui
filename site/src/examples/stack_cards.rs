use dioxus::prelude::*;
use dioxus_shadcn::{Card, CardBody, CardTitle, Stack, StackAlign};

#[component]
pub fn StackCardsDemo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-10 p-4",
      Stack { align: StackAlign::Bottom, class: "w-56",
        Card { class: "bg-card shadow-sm",
          CardBody { class: "p-4",
            CardTitle { class: "text-sm", "Notification 1" }
            p { class: "text-xs text-muted-foreground", "New comment on PR #42." }
          }
        }
        div { "aria-hidden": "true",
          Card { class: "bg-card shadow-sm",
            CardBody { class: "p-4",
              CardTitle { class: "text-sm", "Notification 2" }
            }
          }
        }
        div { "aria-hidden": "true",
          Card { class: "bg-card shadow-sm",
            CardBody { class: "p-4",
              CardTitle { class: "text-sm", "Notification 3" }
            }
          }
        }
      }
      Stack { align: StackAlign::End, class: "w-56",
        div { class: "grid h-24 place-items-center rounded-lg border border-border bg-primary text-sm font-medium text-primary-foreground shadow-sm",
          "Layer 1"
        }
        div { class: "grid h-24 place-items-center rounded-lg border border-border bg-secondary text-sm font-medium text-secondary-foreground",
          "aria-hidden": "true",
          "Layer 2"
        }
        div { class: "grid h-24 place-items-center rounded-lg border border-border bg-muted text-sm font-medium text-muted-foreground",
          "aria-hidden": "true",
          "Layer 3"
        }
      }
    }
  }
}
