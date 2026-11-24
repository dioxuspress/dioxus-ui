use dioxus::prelude::*;
use dioxus_shadcn::{
  Avatar, AvatarFallback, Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Table, TableBody,
  TableCell, TableHead, TableHeader, TableRow,
};

const MEMBERS: [(&str, &str, &str, &str, BadgeVariant); 3] = [
  ("AL", "Ada Lovelace", "Core Maintainer", "Active", BadgeVariant::Success),
  ("GK", "Grace Hopper", "Compiler Lead", "Reviewing", BadgeVariant::Info),
  ("MN", "Margaret Hamilton", "Release Ops", "Invited", BadgeVariant::Warning),
];

#[component]
pub fn TableStatusDemo() -> Element {
  let mut selected = use_signal(|| Option::<String>::None);

  rsx! {
    div { class: "grid min-w-0 gap-3",
      div { class: "min-w-0 overflow-hidden rounded-md border border-border",
        Table {
          TableHeader {
            TableRow {
              TableHead { "Member" }
              TableHead { "Role" }
              TableHead { "Status" }
              TableHead { class: "text-right", "Action" }
            }
          }
          TableBody {
            for (initials, name, role, status, variant) in MEMBERS {
              TableRow { key: "{name}",
                TableCell {
                  div { class: "flex items-center gap-2.5",
                    Avatar { class: "h-8 w-8", AvatarFallback { class: "text-xs", "{initials}" } }
                    span { class: "font-medium whitespace-nowrap", "{name}" }
                  }
                }
                TableCell { class: "text-muted-foreground whitespace-nowrap", "{role}" }
                TableCell {
                  Badge { variant, "{status}" }
                }
                TableCell { class: "text-right",
                  Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| selected.set(Some(name.to_string())),
                    "Inspect"
                  }
                }
              }
            }
          }
        }
      }
      if let Some(member) = selected() {
        p { class: "text-xs text-muted-foreground", role: "status", "Selected member: {member}" }
      }
    }
  }
}
