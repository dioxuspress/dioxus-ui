use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeStyle, BadgeVariant, MockupWindow};

#[component]
pub fn MockupWindowDemo() -> Element {
  rsx! {
    MockupWindow { class: "max-w-lg", content_class: "p-6",
      div { class: "flex flex-wrap items-center justify-between gap-2 border-b border-border pb-4",
        div {
          p { class: "text-sm font-semibold", "Dioxus Desktop Studio" }
          p { class: "text-xs text-muted-foreground", "v0.7.0 · Local preview server" }
        }
        Badge { variant: BadgeVariant::Success, style: BadgeStyle::Soft, "Connected" }
      }
      div { class: "mt-4 grid gap-2 text-sm text-muted-foreground",
        p { "Frame desktop apps, release notes, or interactive product previews with native window chrome." }
      }
    }
  }
}
