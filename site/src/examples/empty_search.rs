use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeSize, BadgeStyle, BadgeVariant, Button, ButtonSize, ButtonVariant, Empty,
  EmptyActions, EmptyContent, EmptyDescription, EmptyHeader, EmptyTitle,
};

#[component]
pub fn EmptySearchDemo() -> Element {
  let mut cleared = use_signal(|| false);

  rsx! {
    Empty {
      EmptyHeader {
        div { class: "flex h-12 w-12 items-center justify-center rounded-full bg-muted text-foreground",
          svg {
            class: "h-5 w-5",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            "aria-hidden": "true",
            circle { cx: "11", cy: "11", r: "7" }
            path { d: "m20 20-3.5-3.5" }
          }
        }
        EmptyTitle {
          if cleared() { "Filters cleared" } else { "No matching components" }
        }
        EmptyDescription {
          if cleared() {
            "All 100 components are ready to browse again."
          } else {
            "Nothing matched \"date-range-v3\" with the active tag filters."
          }
        }
      }
      if !cleared() {
        EmptyContent { class: "flex flex-wrap items-center justify-center gap-2",
          Badge { variant: BadgeVariant::Secondary, style: BadgeStyle::Soft, size: BadgeSize::Sm, "Tag: forms" }
          Badge { variant: BadgeVariant::Secondary, style: BadgeStyle::Soft, size: BadgeSize::Sm, "Status: beta" }
        }
      }
      EmptyActions {
        Button {
          size: ButtonSize::Sm,
          onclick: move |_| cleared.toggle(),
          if cleared() { "Restore sample query" } else { "Clear filters" }
        }
        Button {
          variant: ButtonVariant::Outline,
          size: ButtonSize::Sm,
          onclick: move |_| cleared.set(true),
          "Browse all"
        }
      }
    }
  }
}
