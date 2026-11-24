use dioxus::prelude::*;
use dioxus_shadcn::{Alert, AlertDescription, AlertStyle, AlertTitle, AlertVariant};

#[component]
pub fn AlertVariantsDemo() -> Element {
  rsx! {
    div { class: "grid max-w-lg gap-4",
      Alert {
        AlertTitle { "Heads up!" }
        AlertDescription { "You can add components to your app with the CLI." }
      }
      Alert { variant: AlertVariant::Destructive,
        AlertTitle { "Error" }
        AlertDescription { variant: AlertVariant::Destructive, "Your session has expired. Please sign in again." }
      }
      Alert { variant: AlertVariant::Info, style: AlertStyle::Dash,
        AlertTitle { "Draft migration" }
        AlertDescription { "Dashed border alerts highlight preview or optional actions." }
      }
      Alert { variant: AlertVariant::Success, style: AlertStyle::Solid,
        AlertTitle { "Release published" }
        AlertDescription { "Version 0.7.0 is live across all regions." }
      }
    }
  }
}
