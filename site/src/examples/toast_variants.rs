use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, ToastClose, ToastDescription, ToastPlacement, ToastRoot,
  ToastTitle, ToastVariant, ToastViewport,
};

const LEVELS: [(&str, ToastVariant, &str, &str); 4] = [
  ("Success", ToastVariant::Success, "Build succeeded", "Bundle size stayed under 180 KB gzipped."),
  ("Info", ToastVariant::Info, "Update ready", "Refresh when convenient to load the latest docs."),
  ("Warning", ToastVariant::Warning, "Rate limit near", "85% of your hourly API quota is used."),
  ("Error", ToastVariant::Error, "Sync failed", "Check your network connection and retry."),
];

const PLACEMENTS: [(&str, ToastPlacement); 3] = [
  ("Bottom right", ToastPlacement::BottomRight),
  ("Top right", ToastPlacement::TopRight),
  ("Top center", ToastPlacement::TopCenter),
];

#[component]
pub fn ToastVariantsDemo() -> Element {
  let mut active = use_signal(|| None::<usize>);
  let mut placement = use_signal(|| ToastPlacement::BottomRight);

  rsx! {
    div { class: "grid gap-3",
      div { class: "flex flex-wrap gap-2",
        for (index, (label, _, _, _)) in LEVELS.iter().enumerate() {
          Button {
            key: "{label}",
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            onclick: move |_| active.set(Some(index)),
            "{label}"
          }
        }
      }
      div { class: "flex flex-wrap items-center gap-2 text-xs text-muted-foreground",
        span { "Placement:" }
        for (label, value) in PLACEMENTS {
          Button {
            key: "{label}",
            variant: if placement() == value {
              ButtonVariant::Soft
            } else {
              ButtonVariant::Ghost
            },
            size: ButtonSize::Xs,
            onclick: move |_| placement.set(value),
            "{label}"
          }
        }
      }
      ToastViewport {
        placement: placement(),
        "aria-label": "Severity notifications",
        if let Some((_, variant, title, description)) = active().and_then(|i| LEVELS.get(i)).copied() {
          ToastRoot {
            key: "{title}",
            variant,
            open: true,
            on_dismiss: move |_| active.set(None),
            ToastTitle { "{title}" }
            ToastDescription { variant, "{description}" }
            ToastClose { on_dismiss: move |_| active.set(None), "×" }
          }
        }
      }
    }
  }
}
