use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, SonnerAction, SonnerClose, SonnerContent, SonnerDescription,
  SonnerIcon, SonnerTitle, SonnerToast, SonnerVariant, SonnerViewport,
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum DeployPhase {
  Idle,
  Running,
  Succeeded,
  Failed,
}

#[component]
pub fn SonnerAsyncDemo() -> Element {
  let mut phase = use_signal(|| DeployPhase::Idle);
  let phase_key = match phase() {
    DeployPhase::Idle => "idle",
    DeployPhase::Running => "running",
    DeployPhase::Succeeded => "succeeded",
    DeployPhase::Failed => "failed",
  };

  rsx! {
    div { class: "grid gap-3",
      div { class: "flex flex-wrap gap-2",
        Button {
          size: ButtonSize::Sm,
          onclick: move |_| phase.set(DeployPhase::Running),
          "Start deploy"
        }
        Button {
          variant: ButtonVariant::Outline,
          size: ButtonSize::Sm,
          disabled: phase() != DeployPhase::Running,
          onclick: move |_| phase.set(DeployPhase::Succeeded),
          "Mark succeeded"
        }
        Button {
          variant: ButtonVariant::Outline,
          size: ButtonSize::Sm,
          disabled: phase() != DeployPhase::Running,
          onclick: move |_| phase.set(DeployPhase::Failed),
          "Simulate failure"
        }
      }
      p { class: "text-sm text-muted-foreground",
        match phase() {
          DeployPhase::Idle => "Start a deployment to open a persistent loading toast.",
          DeployPhase::Running => "Loading toast uses duration_ms: 0 until resolved.",
          DeployPhase::Succeeded => "Updated in place to Success with auto-dismiss.",
          DeployPhase::Failed => "Updated in place to Error with an inline Retry action.",
        }
      }
      SonnerViewport { "aria-label": "Deployment notifications",
        match phase() {
          DeployPhase::Idle => rsx! {},
          DeployPhase::Running => rsx! {
            SonnerToast {
              key: "{phase_key}",
              variant: SonnerVariant::Loading,
              duration_ms: 0,
              on_dismiss: move |_| phase.set(DeployPhase::Idle),
              SonnerIcon { variant: SonnerVariant::Loading }
              SonnerContent {
                SonnerTitle { "Deploying release v0.7.0..." }
                SonnerDescription {
                  variant: SonnerVariant::Loading,
                  "Building WebAssembly bundle and uploading assets."
                }
              }
              SonnerClose { on_dismiss: move |_| phase.set(DeployPhase::Idle), "×" }
            }
          },
          DeployPhase::Succeeded => rsx! {
            SonnerToast {
              key: "{phase_key}",
              variant: SonnerVariant::Success,
              on_dismiss: move |_| phase.set(DeployPhase::Idle),
              SonnerIcon { variant: SonnerVariant::Success }
              SonnerContent {
                SonnerTitle { "Release live" }
                SonnerDescription {
                  variant: SonnerVariant::Success,
                  "Deployed to ui.dioxuspress.dev in 14s."
                }
              }
              SonnerClose { on_dismiss: move |_| phase.set(DeployPhase::Idle), "×" }
            }
          },
          DeployPhase::Failed => rsx! {
            SonnerToast {
              key: "{phase_key}",
              variant: SonnerVariant::Error,
              duration_ms: 0,
              on_dismiss: move |_| phase.set(DeployPhase::Idle),
              SonnerIcon { variant: SonnerVariant::Error }
              SonnerContent {
                SonnerTitle { "Deployment failed" }
                SonnerDescription {
                  variant: SonnerVariant::Error,
                  "Edge health check timed out after 30s."
                }
              }
              div { class: "flex items-center gap-1",
                SonnerAction {
                  onclick: move |_| phase.set(DeployPhase::Running),
                  "Retry"
                }
                SonnerClose { on_dismiss: move |_| phase.set(DeployPhase::Idle), "×" }
              }
            }
          },
        }
      }
    }
  }
}
