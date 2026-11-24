use dioxus::prelude::*;
use dioxus_shadcn::{Label, Textarea, TextareaSize, TextareaVariant};

#[component]
pub fn TextareaBasicDemo() -> Element {
  let mut message = use_signal(String::new);
  let count = message().chars().count();

  rsx! {
    div { class: "grid max-w-sm gap-3",
      div { class: "grid gap-1.5",
        Label { r#for: "textarea-basic-message", "Message" }
        Textarea {
          id: "textarea-basic-message",
          placeholder: "Tell us what you think",
          invalid: count > 140,
          value: message(),
          on_value_change: move |value| message.set(value),
        }
        p { class: "text-sm text-muted-foreground", "{count} / 140" }
      }
      div { class: "grid gap-1.5",
        Label { r#for: "textarea-basic-soft", "Soft compact note" }
        Textarea {
          id: "textarea-basic-soft",
          size: TextareaSize::Sm,
          variant: TextareaVariant::Soft,
          placeholder: "Quick internal note...",
        }
      }
      div { class: "grid gap-1.5",
        Label { r#for: "textarea-basic-ghost", "Ghost large editor" }
        Textarea {
          id: "textarea-basic-ghost",
          size: TextareaSize::Lg,
          variant: TextareaVariant::Ghost,
          placeholder: "Click to reveal border and write freely...",
        }
      }
      div { class: "grid gap-1.5",
        Label { r#for: "textarea-basic-disabled", "Disabled" }
        Textarea { id: "textarea-basic-disabled", disabled: true, placeholder: "Read only" }
      }
    }
  }
}
