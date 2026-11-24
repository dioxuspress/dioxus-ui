use dioxus::prelude::*;
use dioxus_shadcn::{Input, InputSize, InputVariant, Label};

#[component]
pub fn InputStatesDemo() -> Element {
  let mut email = use_signal(String::new);

  rsx! {
    div { class: "grid max-w-sm gap-4",
      div { class: "grid gap-2",
        Label { r#for: "input-states-email", "Email (Default · Md)" }
        Input {
          id: "input-states-email",
          r#type: "email",
          placeholder: "you@example.com",
          value: email(),
          on_value_change: move |value| email.set(value),
        }
      }
      div { class: "grid gap-2",
        Label { r#for: "input-states-soft", "Workspace slug (Soft · Sm)" }
        Input {
          id: "input-states-soft",
          size: InputSize::Sm,
          variant: InputVariant::Soft,
          placeholder: "dioxus-press",
        }
      }
      div { class: "grid gap-2",
        Label { r#for: "input-states-ghost", "Document title (Ghost · Lg)" }
        Input {
          id: "input-states-ghost",
          size: InputSize::Lg,
          variant: InputVariant::Ghost,
          placeholder: "Untitled specification",
        }
      }
      div { class: "grid gap-2",
        Label { r#for: "input-states-invalid", "Invalid" }
        Input { id: "input-states-invalid", invalid: true, value: "not an email" }
      }
      div { class: "grid gap-2",
        Label { r#for: "input-states-disabled", "Disabled" }
        Input { id: "input-states-disabled", disabled: true, placeholder: "Unavailable" }
      }
    }
  }
}
