use dioxus::prelude::*;
use dioxus_shadcn::{
  AlertDialog, AlertDialogAction, AlertDialogActionVariant, AlertDialogCancel, AlertDialogContent,
  AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogOverlay,
  AlertDialogTitle, AlertDialogTrigger, ButtonSize, ButtonVariant, Input, Label, button_class,
  use_density,
};

#[component]
pub fn AlertDialogInputDemo() -> Element {
  let mut confirm_text = use_signal(String::new);
  let mut status = use_signal(|| "workspace active");
  let matches_slug = confirm_text().trim() == "acme-prod";

  rsx! {
    AlertDialog {
      AlertDialogTrigger {
        class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
        "Transfer workspace"
      }
      p { class: "mt-3 text-sm text-muted-foreground", "Status: {status}" }
      AlertDialogOverlay {}
      AlertDialogContent {
        AlertDialogHeader {
          AlertDialogTitle { "Confirm workspace transfer" }
          AlertDialogDescription {
            "Type acme-prod below to confirm transferring ownership to the platform team."
          }
        }
        div { class: "grid gap-1.5 py-2",
          Label { r#for: "alert-dialog-slug-input", "Workspace slug" }
          Input {
            id: "alert-dialog-slug-input",
            placeholder: "acme-prod",
            value: confirm_text(),
            on_value_change: move |next| confirm_text.set(next),
          }
        }
        AlertDialogFooter {
          AlertDialogCancel {
            onclick: move |_| {
              confirm_text.set(String::new());
              status.set("transfer cancelled");
            },
            "Cancel"
          }
          AlertDialogAction {
            variant: AlertDialogActionVariant::Default,
            disabled: !matches_slug,
            onclick: move |_| {
              confirm_text.set(String::new());
              status.set("transfer scheduled");
            },
            "Transfer"
          }
        }
      }
    }
  }
}
