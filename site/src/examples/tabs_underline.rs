use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeVariant, Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn TabsUnderlineDemo() -> Element {
  let mut active = use_signal(|| "overview".to_string());

  let trigger_class = |id: &str| {
    if active() == id {
      "gap-2 rounded-none border-b-2 border-primary bg-transparent px-1 pb-2.5 pt-1 text-foreground shadow-none"
    } else {
      "gap-2 rounded-none border-b-2 border-transparent bg-transparent px-1 pb-2.5 pt-1 text-muted-foreground shadow-none hover:text-foreground"
    }
  };

  rsx! {
    Tabs {
      class: "max-w-lg",
      value: Some(active()),
      on_value_change: move |next| active.set(next),
      TabsList {
        class: "h-auto w-full flex-wrap justify-start gap-5 rounded-none border-b border-border bg-transparent p-0",
        "aria-label": "Repository views",
        TabsTrigger { value: "overview", class: trigger_class("overview"), "Overview" }
        TabsTrigger { value: "issues", class: trigger_class("issues"),
          "Issues"
          Badge { variant: BadgeVariant::Secondary, "12" }
        }
        TabsTrigger { value: "releases", class: trigger_class("releases"),
          "Releases"
          Badge { variant: BadgeVariant::Outline, "0.6.8" }
        }
      }
      TabsContent { value: "overview", class: "pt-3 text-sm text-muted-foreground",
        "100 components, 11 page blocks, and 33 theme presets verified against WCAG AA."
      }
      TabsContent { value: "issues", class: "pt-3 text-sm text-muted-foreground",
        "12 open issues triaged across forms, overlays, and responsive table layouts."
      }
      TabsContent { value: "releases", class: "pt-3 text-sm text-muted-foreground",
        "Latest crate release v0.6.8 published to crates.io with full CLI template parity."
      }
    }
  }
}
