use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonSize, ButtonVariant, Label, Textarea};

const PROMPTS: [&str; 3] = [
  "Great theme contrast across dark mode.",
  "Would love more table density examples.",
  "CLI source copy worked cleanly on the first run.",
];

#[component]
pub fn TextareaFormDemo() -> Element {
  let mut note = use_signal(String::new);
  let mut attempted = use_signal(|| false);
  let mut submitted = use_signal(|| None::<String>);

  let trimmed = note().trim().to_string();
  let too_short = attempted() && trimmed.len() < 12;

  rsx! {
    div { class: "grid max-w-md gap-3",
      div { class: "grid gap-1.5",
        Label { r#for: "textarea-form-feedback", "Release feedback" }
        Textarea {
          id: "textarea-form-feedback",
          placeholder: "Share what worked well or what slowed you down...",
          invalid: too_short,
          "aria-describedby": "textarea-form-help",
          value: note(),
          on_value_change: move |value| {
            note.set(value);
            submitted.set(None);
          },
        }
        p {
          id: "textarea-form-help",
          class: "text-xs text-muted-foreground",
          if too_short {
            "Enter at least 12 characters before submitting."
          } else {
            "Minimum 12 characters. Pick a quick starter below or write your own."
          }
        }
      }
      div { class: "flex flex-wrap gap-2",
        for (index, prompt) in PROMPTS.into_iter().enumerate() {
          Button {
            key: "{index}",
            variant: ButtonVariant::Soft,
            size: ButtonSize::Xs,
            onclick: move |_| {
              note.set(prompt.to_string());
              attempted.set(false);
              submitted.set(None);
            },
            "Preset {index + 1}"
          }
        }
      }
      div { class: "flex flex-wrap items-center gap-2",
        Button {
          size: ButtonSize::Sm,
          onclick: move |_| {
            attempted.set(true);
            let value = note().trim().to_string();
            if value.len() >= 12 {
              submitted.set(Some(value));
              note.set(String::new());
              attempted.set(false);
            }
          },
          "Submit feedback"
        }
        Button {
          variant: ButtonVariant::Ghost,
          size: ButtonSize::Sm,
          disabled: note().is_empty(),
          onclick: move |_| {
            note.set(String::new());
            attempted.set(false);
            submitted.set(None);
          },
          "Clear"
        }
      }
      if let Some(sent) = submitted() {
        p { class: "text-sm text-muted-foreground", role: "status", "Sent: \"{sent}\"" }
      }
    }
  }
}
