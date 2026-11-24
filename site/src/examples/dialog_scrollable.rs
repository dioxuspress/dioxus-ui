use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, Dialog, DialogClose, DialogContent, DialogDescription,
  DialogOverlay, DialogTitle, DialogTrigger, button_class, use_density,
};

#[component]
pub fn DialogScrollableDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut accepted = use_signal(|| false);

  rsx! {
    Dialog { open: open(), on_open_change: move |next| open.set(next),
      div { class: "flex flex-wrap items-center gap-3",
        DialogTrigger {
          class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
          "Review release terms"
        }
        if accepted() {
          span { class: "text-sm text-muted-foreground", role: "status", "Terms accepted." }
        }
      }
      DialogOverlay {}
      DialogContent { class: "max-w-lg",
        DialogTitle { "Release checklist & terms" }
        DialogDescription { "Scroll the summary before confirming publication." }
        div {
          class: "max-h-56 space-y-3 overflow-y-auto rounded-md border border-border bg-muted/30 p-3 text-sm leading-6 text-foreground",
          tabindex: "0",
          p { "1. Semantic versioning: public helper signatures remain backwards compatible across minor releases." }
          p { "2. Accessibility: every interactive widget exposes keyboard navigation, focus containment, and ARIA states." }
          p { "3. Theme parity: all 33 theme presets are validated for WCAG AA text contrast in both light and dark modes." }
          p { "4. CSP compliance: runtime helpers run without eval or unsafe inline execution on web targets." }
          p { "5. Template sync: CLI source-copy templates match the packaged crate modules line for line." }
        }
        div { class: "flex justify-end gap-2",
          Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Cancel" }
          Button {
            onclick: move |_| {
              accepted.set(true);
              open.set(false);
            },
            "Accept & publish"
          }
        }
        DialogClose { "×" }
      }
    }
  }
}
