use dioxus::prelude::*;
use dioxus_shadcn::{
  ButtonSize, ButtonVariant, Command, CommandEmpty, CommandGroup, CommandInput, CommandItem,
  CommandLabel, CommandList, CommandSeparator, CommandShortcut, CommandStatus, Dialog,
  DialogContent, DialogDescription, DialogOverlay, DialogTitle, DialogTrigger, Kbd, KbdSize,
  button_class, command_matches, use_density,
};

const NAV_ITEMS: [(&str, &str, &str); 3] = [
  ("docs", "Open documentation", "G D"),
  ("themes", "Switch theme preset", "G T"),
  ("blocks", "Browse page blocks", "G B"),
];
const QUICK_ACTIONS: [(&str, &str, &str); 2] = [
  ("new-project", "Create workspace", "N"),
  ("copy-cli", "Copy dxui add command", "C"),
];

#[component]
pub fn CommandDialogDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut query = use_signal(String::new);
  let mut last_run = use_signal(|| "none".to_string());
  let matches_query = move |label: &str| command_matches(label, &query());
  let nav_matches = NAV_ITEMS.iter().filter(|(_, label, _)| matches_query(label)).copied().collect::<Vec<_>>();
  let action_matches = QUICK_ACTIONS.iter().filter(|(_, label, _)| matches_query(label)).copied().collect::<Vec<_>>();
  let total = nav_matches.len() + action_matches.len();
  let has_both = !nav_matches.is_empty() && !action_matches.is_empty();

  rsx! {
    div { class: "flex flex-wrap items-center gap-3",
      Dialog {
        open: Some(open()),
        on_open_change: move |next| open.set(next),
        DialogTrigger {
          class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), "gap-2"),
          "Open command palette"
          Kbd { size: KbdSize::Sm, "⌘K" }
        }
        DialogOverlay {}
        DialogContent { class: "max-w-lg p-0 overflow-hidden",
          DialogTitle { class: "sr-only", "Command palette" }
          DialogDescription { class: "sr-only", "Search navigation routes and quick actions." }
          Command {
            on_select: move |value: String| {
              last_run.set(value);
              open.set(false);
            },
            CommandInput {
              value: query(),
              placeholder: "Jump to page or run action...",
              oninput: move |event: FormEvent| query.set(event.value()),
            }
            CommandStatus { "{total} command results" }
            CommandList {
              if total == 0 {
                CommandEmpty { "No matching commands." }
              }
              if !nav_matches.is_empty() {
                CommandGroup {
                  CommandLabel { "Navigation" }
                  for (id, label, shortcut) in nav_matches {
                    CommandItem { key: "{id}", id: "command-dialog-{id}", value: id,
                      "{label}"
                      CommandShortcut { "{shortcut}" }
                    }
                  }
                }
              }
              if has_both {
                CommandSeparator {}
              }
              if !action_matches.is_empty() {
                CommandGroup {
                  CommandLabel { "Actions" }
                  for (id, label, shortcut) in action_matches {
                    CommandItem { key: "{id}", id: "command-dialog-{id}", value: id,
                      "{label}"
                      CommandShortcut { "{shortcut}" }
                    }
                  }
                }
              }
            }
          }
        }
      }
      span { class: "text-sm text-muted-foreground", "Last command: {last_run}" }
    }
  }
}
