use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeSize, BadgeStyle, BadgeVariant, Breadcrumb, BreadcrumbItem, BreadcrumbLink,
  BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};

#[component]
pub fn BreadcrumbCustomDemo() -> Element {
  rsx! {
    div { class: "grid gap-4",
      Breadcrumb { "aria-label": "Repository path",
        BreadcrumbList {
          BreadcrumbItem {
            BreadcrumbLink { href: "#workspace", class: "inline-flex items-center gap-1.5",
              svg {
                class: "h-3.5 w-3.5",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                "aria-hidden": "true",
                path { d: "M3 10.5 12 3l9 7.5" }
                path { d: "M5 9.5V20h14V9.5" }
              }
              "Workspace"
            }
          }
          BreadcrumbSeparator { "›" }
          BreadcrumbItem {
            BreadcrumbLink { href: "#crates", "crates" }
          }
          BreadcrumbSeparator { "›" }
          BreadcrumbItem {
            BreadcrumbPage { class: "inline-flex items-center gap-2 font-medium",
              "dioxus-shadcn"
              Badge {
                variant: BadgeVariant::Secondary,
                style: BadgeStyle::Soft,
                size: BadgeSize::Xs,
                "v0.7"
              }
            }
          }
        }
      }
      Breadcrumb { "aria-label": "Documentation section",
        BreadcrumbList {
          BreadcrumbItem {
            BreadcrumbLink { href: "#docs", "Docs" }
          }
          BreadcrumbSeparator { "·" }
          BreadcrumbItem {
            BreadcrumbLink { href: "#navigation", "Navigation" }
          }
          BreadcrumbSeparator { "·" }
          BreadcrumbItem {
            BreadcrumbPage { "Custom separators" }
          }
        }
      }
    }
  }
}
