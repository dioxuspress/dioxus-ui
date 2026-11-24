use dioxus::prelude::*;
use dioxus_shadcn::{Tabs, TabsContent, TabsList, TabsOrientation, TabsTrigger};

#[component]
pub fn TabsVerticalDemo() -> Element {
  rsx! {
    Tabs {
      orientation: TabsOrientation::Vertical,
      default_value: "general",
      class: "max-w-lg flex-col sm:flex-row",
      TabsList { class: "w-full sm:w-40", "aria-label": "Workspace sections",
        TabsTrigger { value: "general", class: "justify-start", "General" }
        TabsTrigger { value: "security", class: "justify-start", "Security" }
        TabsTrigger { value: "tokens", class: "justify-start", "API tokens" }
      }
      TabsContent { value: "general", class: "flex-1 rounded-md border border-border p-4",
        p { class: "text-sm font-medium", "General settings" }
        p { class: "mt-1 text-sm text-muted-foreground",
          "Configure workspace display name, locale defaults, and timezone."
        }
      }
      TabsContent { value: "security", class: "flex-1 rounded-md border border-border p-4",
        p { class: "text-sm font-medium", "Security controls" }
        p { class: "mt-1 text-sm text-muted-foreground",
          "Require hardware passkeys and enforce single sign-on for new members."
        }
      }
      TabsContent { value: "tokens", class: "flex-1 rounded-md border border-border p-4",
        p { class: "text-sm font-medium", "API tokens" }
        p { class: "mt-1 text-sm text-muted-foreground",
          "Create scoped tokens for CI publishing and automated deploys."
        }
      }
    }
  }
}
