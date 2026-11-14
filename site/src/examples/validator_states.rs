use dioxus::prelude::*;
use dioxus_shadcn::{Input, Validator, ValidatorHint, ValidatorState};

#[component]
pub fn ValidatorStatesDemo() -> Element {
  rsx! {
    div { class: "grid max-w-md gap-4 p-4",
      Validator { state: ValidatorState::Valid,
        Input {
          r#type: "text".to_string(),
          value: "dioxus-shadcn".to_string(),
          "aria-label": "Package name",
        }
        ValidatorHint { state: ValidatorState::Valid, "Package name is available." }
      }
      Validator { state: ValidatorState::Invalid,
        Input {
          r#type: "email".to_string(),
          value: "invalid-email".to_string(),
          "aria-label": "Email address",
        }
        ValidatorHint { state: ValidatorState::Invalid, "Enter a valid email address." }
      }
    }
  }
}
