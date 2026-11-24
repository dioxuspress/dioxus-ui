use dioxus::prelude::*;
use dioxus_shadcn::{Label, Rating, RatingSize};

#[component]
pub fn RatingReviewDemo() -> Element {
  let mut quality = use_signal(|| 4_u8);
  let mut service = use_signal(|| 7_u8);

  rsx! {
    div { class: "grid gap-4",
      div { class: "grid gap-1",
        Label { id: "rating-quality-label", "Quality (interactive Lg)" }
        Rating {
          "aria-labelledby": "rating-quality-label",
          size: RatingSize::Lg,
          value: quality(),
          on_value_change: move |value| quality.set(value),
        }
      }
      div { class: "grid gap-1",
        Label { id: "rating-service-label", "Service (10-point Md)" }
        Rating {
          "aria-labelledby": "rating-service-label",
          max: 10,
          value: service(),
          on_value_change: move |value| service.set(value),
        }
      }
      div { class: "flex flex-wrap items-center gap-3",
        span { class: "text-sm font-medium", "Verified average" }
        Rating { value: 4, size: RatingSize::Sm, read_only: true, "aria-label": "4 of 5 stars" }
        span { class: "text-sm text-muted-foreground", "4.0 / 5 (read-only Sm)" }
      }
      p { class: "text-sm text-muted-foreground", "Quality {quality}/5, service {service}/10" }
    }
  }
}
