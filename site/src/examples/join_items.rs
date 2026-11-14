use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonVariant, Input, Join, JoinOrientation};

#[component]
pub fn JoinItemsDemo() -> Element {
  rsx! {
    div { class: "flex flex-col gap-4 p-4",
      Join { orientation: JoinOrientation::Horizontal,
        Input { placeholder: "Search crates..." }
        Button { "Search" }
      }
      Join { orientation: JoinOrientation::Horizontal,
        Button { variant: ButtonVariant::Outline, "Prev" }
        Button { variant: ButtonVariant::Outline, "1" }
        Button { "2" }
        Button { variant: ButtonVariant::Outline, "Next" }
      }
    }
  }
}
