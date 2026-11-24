use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeSize, BadgeVariant, Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};

#[component]
pub fn TabsUnderlineDemo() -> Element {
  rsx! {
    div { class: "grid max-w-lg gap-6",
      Tabs {
        variant: TabsVariant::Underline,
        default_value: Some("overview".to_string()),
        TabsList { class: "w-full flex-wrap", "aria-label": "Repository views",
          TabsTrigger { value: "overview", "Overview" }
          TabsTrigger { value: "issues", class: "gap-2",
            "Issues"
            Badge { variant: BadgeVariant::Secondary, size: BadgeSize::Xs, pill: true, "12" }
          }
          TabsTrigger { value: "releases", class: "gap-2",
            "Releases"
            Badge { variant: BadgeVariant::Outline, size: BadgeSize::Xs, pill: true, "0.7.0" }
          }
        }
        TabsContent { value: "overview", class: "pt-3 text-sm text-muted-foreground",
          "100 components, 11 page blocks, and 33 theme presets verified against WCAG AA."
        }
        TabsContent { value: "issues", class: "pt-3 text-sm text-muted-foreground",
          "12 open issues triaged across forms, overlays, and responsive table layouts."
        }
        TabsContent { value: "releases", class: "pt-3 text-sm text-muted-foreground",
          "Latest crate release v0.7.0 ships first-class component variants with CLI parity."
        }
      }
      Tabs {
        variant: TabsVariant::Bordered,
        default_value: Some("preview".to_string()),
        TabsList { class: "flex-wrap", "aria-label": "Editor mode",
          TabsTrigger { value: "preview", "Preview" }
          TabsTrigger { value: "code", "Code" }
        }
        TabsContent { value: "preview", class: "text-sm text-muted-foreground",
          "Rendered output using TabsVariant::Bordered."
        }
        TabsContent { value: "code", class: "text-sm text-muted-foreground",
          "Source snippet with zero manual trigger styling."
        }
      }
    }
  }
}
