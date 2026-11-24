use dioxus::prelude::*;
use dioxus_shadcn::{Button, ButtonSize, ButtonVariant, Step, StepStatus, Steps};

const STAGES: [(&str, &str); 3] = [
  ("Workspace", "Choose a project name and default region for your Dioxus app."),
  ("Theme", "Pick one of the 33 presets or customize semantic CSS tokens."),
  ("Deploy", "Connect your repository for zero-downtime edge previews."),
];

#[component]
pub fn StepsClickableDemo() -> Element {
  let mut active = use_signal(|| 0_usize);
  let status = move |index: usize| {
    if index < active() {
      StepStatus::Complete
    } else if index == active() {
      StepStatus::Current
    } else {
      StepStatus::Upcoming
    }
  };
  let (title, summary) = STAGES.get(active()).copied().unwrap_or(STAGES[0]);

  rsx! {
    div { class: "grid max-w-lg gap-5",
      Steps { "aria-label": "Onboarding stages",
        for (index, (label, _)) in STAGES.iter().enumerate() {
          Step { key: "{label}", status: status(index),
            button {
              r#type: "button",
              class: "cursor-pointer rounded px-1.5 py-0.5 text-inherit underline-offset-4 hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
              onclick: move |_| active.set(index),
              "{label}"
            }
          }
        }
      }
      div { class: "rounded-md border border-border bg-card p-4 text-card-foreground",
        p { class: "text-sm font-semibold", "{title}" }
        p { class: "mt-1 text-sm text-muted-foreground", "{summary}" }
        div { class: "mt-3 flex justify-end gap-2",
          Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            disabled: active() == 0,
            onclick: move |_| active -= 1,
            "Previous"
          }
          Button {
            size: ButtonSize::Sm,
            disabled: active() + 1 == STAGES.len(),
            onclick: move |_| active += 1,
            "Continue"
          }
        }
      }
    }
  }
}
