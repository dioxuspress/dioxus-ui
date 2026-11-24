use dioxus::prelude::*;
use dioxus_shadcn::{
  Carousel, CarouselContent, CarouselIndicator, CarouselItem, CarouselNext, CarouselOrientation,
  CarouselPrevious, CarouselViewport,
};

const NOTES: [(&str, &str); 3] = [
  ("0.7.0", "Orthogonal variants & sizes across 12 core components"),
  ("0.6.8", "100 accessible components with bilingual site docs"),
  ("0.6.5", "33 WCAG AA theme presets and density provider"),
];

#[component]
pub fn CarouselVerticalDemo() -> Element {
  let mut current = use_signal(|| 0_usize);

  rsx! {
    Carousel {
      class: "max-w-sm",
      "aria-label": "Release highlights",
      count: NOTES.len(),
      looping: true,
      orientation: CarouselOrientation::Vertical,
      index: Some(current()),
      on_index_change: move |next| current.set(next),
      CarouselViewport { class: "h-28",
        CarouselContent { class: "h-28",
          for (idx, (version, summary)) in NOTES.iter().enumerate() {
            CarouselItem {
              key: "{version}",
              index: idx,
              class: "h-28",
              "aria-label": "Release {version}",
              div { class: "flex h-full flex-col justify-center rounded-md border border-border bg-muted/50 px-4",
                p { class: "text-xs font-semibold text-muted-foreground", "v{version}" }
                p { class: "mt-1 text-sm font-medium text-foreground", "{summary}" }
              }
            }
          }
        }
      }
      div { class: "mt-3 flex items-center justify-between gap-2",
        div { class: "flex items-center gap-2",
          CarouselPrevious { "↑" }
          for idx in 0..NOTES.len() {
            CarouselIndicator { key: "{idx}", index: idx, "aria-label": "Show highlight {idx + 1}" }
          }
          CarouselNext { "↓" }
        }
        span { class: "text-xs text-muted-foreground", "{current() + 1} / {NOTES.len()} (looping)" }
      }
    }
  }
}
