use dioxus::prelude::*;
use dioxus_shadcn::{
  Menubar, MenubarContent, MenubarItem, MenubarLabel, MenubarMenu, MenubarRadioGroup,
  MenubarRadioItem, MenubarSeparator, MenubarSub, MenubarSubContent, MenubarSubTrigger,
  MenubarTrigger,
};

#[component]
pub fn MenubarSubmenuDemo() -> Element {
  let mut profile = use_signal(|| "studio".to_string());
  let mut last_action = use_signal(|| "ready".to_string());

  rsx! {
    Menubar { "aria-label": "Workspace",
      MenubarMenu { value: "project",
        MenubarTrigger { "Project" }
        MenubarContent {
          MenubarLabel { "Build targets" }
          MenubarSub {
            MenubarSubTrigger { "Deploy to" }
            MenubarSubContent {
              MenubarItem { onclick: move |_| last_action.set("staging".to_string()), "Staging" }
              MenubarItem { onclick: move |_| last_action.set("production".to_string()), "Production" }
            }
          }
          MenubarSeparator {}
          MenubarItem { onclick: move |_| last_action.set("clean".to_string()), "Clean cache" }
        }
      }
      MenubarMenu { value: "profile",
        MenubarTrigger { "Profile" }
        MenubarContent {
          MenubarLabel { "Active workspace" }
          MenubarRadioGroup {
            value: Some(profile()),
            on_value_change: move |next| profile.set(next),
            MenubarRadioItem { value: "studio", "Studio" }
            MenubarRadioItem { value: "docs", "Docs preview" }
            MenubarRadioItem { value: "compact", "Compact review" }
          }
        }
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Profile: {profile} · Action: {last_action}" }
  }
}
