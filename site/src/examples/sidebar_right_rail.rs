use dioxus::prelude::*;
use dioxus_shadcn::{
  Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel, SidebarHeader,
  SidebarItem, SidebarProvider, SidebarRail, SidebarSide, SidebarTrigger,
};

#[component]
pub fn SidebarRightRailDemo() -> Element {
  let mut tab = use_signal(|| "tokens");

  rsx! {
    SidebarProvider {
      div { class: "flex h-64 overflow-hidden rounded-md border border-border",
        div { class: "flex-1 p-4",
          div { class: "flex items-center justify-between gap-2",
            p { class: "text-sm font-medium", "Inspector: {tab}" }
            SidebarTrigger { "aria-label": "Toggle inspector sidebar", "Inspector" }
          }
          p { class: "mt-2 text-xs text-muted-foreground",
            "The inspector docks on the right with a visual rail edge when collapsed."
          }
        }
        SidebarRail {}
        Sidebar { side: SidebarSide::Right, "aria-label": "Inspector",
          SidebarHeader { span { class: "text-sm font-semibold", "Inspector" } }
          SidebarContent {
            SidebarGroup { role: "group", "aria-labelledby": "sidebar-right-rail-label",
              SidebarGroupLabel { id: "sidebar-right-rail-label", "Properties" }
              for (value, label) in [("tokens", "Tokens"), ("layout", "Layout"), ("a11y", "Accessibility")] {
                SidebarItem {
                  key: "{value}",
                  active: tab() == value,
                  onclick: move |_| tab.set(value),
                  "{label}"
                }
              }
            }
          }
          SidebarFooter { span { class: "text-xs text-muted-foreground", "Side: Right" } }
        }
      }
    }
  }
}
