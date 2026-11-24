use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, Empty, EmptyActions, EmptyDescription, EmptyHeader,
  EmptyTitle,
};

#[component]
pub fn EmptyPermissionDemo() -> Element {
  let mut requested = use_signal(|| false);

  rsx! {
    Empty {
      EmptyHeader {
        div { class: "flex h-12 w-12 items-center justify-center rounded-full bg-muted text-foreground",
          svg {
            class: "h-5 w-5",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            "aria-hidden": "true",
            rect { x: "5", y: "11", width: "14", height: "10", rx: "2" }
            path { d: "M8 11V7a4 4 0 0 1 8 0v4" }
          }
        }
        EmptyTitle { "Restricted workspace" }
        EmptyDescription {
          "Billing and production API keys require the Maintainer role in this organization."
        }
      }
      EmptyActions {
        Button {
          size: ButtonSize::Sm,
          disabled: requested(),
          onclick: move |_| requested.set(true),
          if requested() { "Access requested" } else { "Request access" }
        }
        if requested() {
          Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Sm,
            onclick: move |_| requested.set(false),
            "Cancel request"
          }
        }
      }
    }
  }
}
