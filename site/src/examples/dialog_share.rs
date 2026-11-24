use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Dialog, DialogClose, DialogContent,
  DialogDescription, DialogOverlay, DialogTitle, DialogTrigger, Input, Label, button_class,
  use_density,
};

#[component]
pub fn DialogShareDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut copied = use_signal(|| false);

  rsx! {
    Dialog {
      open: open(),
      on_open_change: move |next| {
        open.set(next);
        if !next {
          copied.set(false);
        }
      },
      DialogTrigger {
        class: button_class(ButtonVariant::Primary, ButtonSize::Md, use_density(), ""),
        "Share preview"
      }
      DialogOverlay {}
      DialogContent { class: "max-w-md",
        div { class: "flex items-center gap-2",
          DialogTitle { "Share link" }
          Badge { variant: BadgeVariant::Secondary, "Public" }
        }
        DialogDescription { "Anyone with this link can view the live component preview." }
        div { class: "grid gap-2",
          Label { r#for: "dialog-share-url", "Preview URL" }
          div { class: "flex gap-2",
            Input {
              id: "dialog-share-url",
              value: "https://ui.dioxuspress.dev/components/dialog",
              readonly: true,
            }
            Button {
              variant: ButtonVariant::Outline,
              onclick: move |_| copied.set(true),
              if copied() { "Copied" } else { "Copy" }
            }
          }
        }
        if copied() {
          p { class: "text-xs text-muted-foreground", role: "status", "Link ready to paste." }
        }
        DialogClose { "×" }
      }
    }
  }
}
