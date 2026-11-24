use dioxus::prelude::*;
use dioxus_shadcn::{Input, Label};

#[component]
pub fn InputValidationDemo() -> Element {
  let mut email = use_signal(|| "ada@dioxuspress".to_string());
  let trimmed = email().trim().to_string();
  let is_valid = trimmed.contains('@')
    && trimmed.split('@').nth(1).is_some_and(|domain| domain.contains('.'));
  let show_error = !trimmed.is_empty() && !is_valid;

  rsx! {
    div { class: "grid max-w-sm gap-2",
      Label { r#for: "input-validation-email", "Work email" }
      Input {
        id: "input-validation-email",
        r#type: "email",
        invalid: show_error,
        "aria-describedby": "input-validation-hint",
        value: email(),
        on_value_change: move |value| email.set(value),
      }
      if show_error {
        p { id: "input-validation-hint", class: "text-xs text-destructive",
          "Include a full domain such as ada@dioxuspress.dev."
        }
      } else if is_valid {
        p { id: "input-validation-hint", class: "text-xs text-muted-foreground",
          "Looks good. Ready to send verification code."
        }
      } else {
        p { id: "input-validation-hint", class: "text-xs text-muted-foreground",
          "We only send release notifications to this address."
        }
      }
    }
  }
}
