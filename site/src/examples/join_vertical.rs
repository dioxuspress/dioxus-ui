use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonVariant, Input, Join, JoinOrientation};

#[component]
pub fn JoinVerticalDemo() -> Element {
  let mut active = use_signal(|| "overview");
  let mut query = use_signal(String::new);

  rsx! {
    div { class: "flex flex-wrap items-start gap-6 p-2",
      Join { orientation: JoinOrientation::Vertical, class: "w-44",
        for (id, label) in [("overview", "Overview"), ("tokens", "Design tokens"), ("releases", "Releases")] {
          Button {
            key: "{id}",
            variant: if active() == id { ButtonVariant::Primary } else { ButtonVariant::Outline },
            class: "justify-start",
            onclick: move |_| active.set(id),
            "{label}"
          }
        }
      }
      div { class: "grid flex-1 min-w-60 gap-2",
        Join { orientation: JoinOrientation::Horizontal,
          Input {
            "aria-label": "Filter components",
            placeholder: "Filter components...",
            value: query(),
            on_value_change: move |next| query.set(next),
          }
          Button {
            variant: ButtonVariant::Secondary,
            onclick: move |_| query.set(String::new()),
            "Clear"
          }
        }
        p { class: "text-xs text-muted-foreground", "Section: {active}" }
      }
    }
  }
}
