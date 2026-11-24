use dioxus::prelude::*;
use dioxus_shadcn::{
  Attachment, AttachmentContent, AttachmentDescription, AttachmentGroup, AttachmentMedia,
  AttachmentMediaVariant, AttachmentOrientation, AttachmentSize, AttachmentState, AttachmentTitle,
};

#[component]
pub fn AttachmentGalleryDemo() -> Element {
  rsx! {
    div { class: "grid max-w-xl gap-4",
      div { class: "grid gap-2",
        Attachment { size: AttachmentSize::Xs, state: AttachmentState::Idle,
          AttachmentMedia { variant: AttachmentMediaVariant::Icon,
            span { class: "text-xs font-semibold", "MD" }
          }
          AttachmentContent {
            AttachmentTitle { "CHANGELOG.md" }
            AttachmentDescription { "Extra-small idle row · 18 KB" }
          }
        }
        Attachment { size: AttachmentSize::Sm, state: AttachmentState::Done,
          AttachmentMedia { variant: AttachmentMediaVariant::Icon,
            span { class: "text-xs font-semibold", "RS" }
          }
          AttachmentContent {
            AttachmentTitle { "components.rs" }
            AttachmentDescription { "Small completed row · 42 KB" }
          }
        }
      }
      AttachmentGroup { class: "grid grid-cols-1 gap-3 sm:grid-cols-2",
        Attachment {
          size: AttachmentSize::Default,
          orientation: AttachmentOrientation::Vertical,
          state: AttachmentState::Idle,
          AttachmentMedia { variant: AttachmentMediaVariant::Image, class: "w-full h-24 bg-muted",
            div { class: "flex h-full w-full items-center justify-center text-xs font-medium text-muted-foreground",
              "hero-preview.png"
            }
          }
          AttachmentContent {
            AttachmentTitle { "hero-preview.png" }
            AttachmentDescription { "Vertical card · 1280 × 720" }
          }
        }
        Attachment {
          size: AttachmentSize::Default,
          orientation: AttachmentOrientation::Vertical,
          state: AttachmentState::Done,
          AttachmentMedia { variant: AttachmentMediaVariant::Image, class: "w-full h-24 bg-muted",
            div { class: "flex h-full w-full items-center justify-center text-xs font-medium text-muted-foreground",
              "theme-tokens.svg"
            }
          }
          AttachmentContent {
            AttachmentTitle { "theme-tokens.svg" }
            AttachmentDescription { "Vertical card · Verified" }
          }
        }
      }
    }
  }
}
