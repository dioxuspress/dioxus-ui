use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, Drawer, DrawerClose, DrawerContent, DrawerDescription,
  DrawerFooter, DrawerHeader, DrawerOverlay, DrawerTitle, DrawerTrigger, button_class, use_density,
};

const ACTIONS: [(&str, &str); 4] = [
  ("Copy share link", "Anyone with the link can view"),
  ("Duplicate workspace", "Create an editable copy in your team"),
  ("Export CSV report", "Download the last 30 days of metrics"),
  ("Move to archive", "Hide from the active sidebar"),
];

#[component]
pub fn DrawerActionsDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut last_action = use_signal(|| "None".to_string());

  rsx! {
    div { class: "flex flex-wrap items-center gap-3",
      Drawer { open: open(), on_open_change: move |next| open.set(next),
        DrawerTrigger {
          class: button_class(ButtonVariant::Outline, ButtonSize::Sm, use_density(), ""),
          "Open action sheet"
        }
        DrawerOverlay {}
        DrawerContent {
          DrawerHeader {
            DrawerTitle { "Project actions" }
            DrawerDescription { "Choose a quick task for this workspace." }
          }
          div { class: "mx-auto grid w-full max-w-md gap-2",
            for (title, hint) in ACTIONS {
              Button {
                key: "{title}",
                variant: ButtonVariant::Outline,
                class: "h-auto justify-between px-4 py-3 text-left",
                onclick: move |_| {
                  last_action.set(title.to_string());
                  open.set(false);
                },
                span { class: "grid gap-0.5",
                  span { class: "text-sm font-medium", "{title}" }
                  span { class: "text-xs font-normal text-muted-foreground", "{hint}" }
                }
              }
            }
          }
          DrawerFooter {
            DrawerClose { "Cancel" }
          }
        }
      }
      p { class: "text-sm text-muted-foreground", "Last action: {last_action}" }
    }
  }
}
