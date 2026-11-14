//! HoverGallery: switches between multiple images as the pointer moves across horizontal zones.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// Single image entry in a `HoverGallery`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoverGalleryImage {
  /// Image URL.
  pub src: String,
  /// Accessible alternative text for the image.
  pub alt: String,
}

impl HoverGalleryImage {
  /// Creates a new gallery image entry.
  pub fn new(src: impl Into<String>, alt: impl Into<String>) -> Self {
    Self { src: src.into(), alt: alt.into() }
  }
}

const HOVER_GALLERY_BASE_CLASS: &str =
  "group relative grid overflow-hidden rounded-lg bg-muted select-none";

/// Classes for the `HoverGallery` container.
pub fn hover_gallery_class(class: &str) -> String {
  merge_classes(classes([Some(HOVER_GALLERY_BASE_CLASS)]), class)
}

/// Displays a primary image and switches across `images` as the pointer moves across horizontal slices.
#[component]
pub fn HoverGallery(
  images: Vec<HoverGalleryImage>,
  #[props(default = true)] show_indicators: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
  let mut active_index = use_signal(|| 0usize);
  let total = images.len();
  let current = if total == 0 { 0 } else { active_index().min(total - 1) };
  let class = hover_gallery_class(&class);

  rsx! {
    div {
      class,
      "data-active-index": "{current}",
      onmouseleave: move |_| active_index.set(0),
      ..attributes,
      for (idx, image) in images.iter().enumerate() {
        img {
          key: "{idx}-{image.src}",
          src: "{image.src}",
          alt: "{image.alt}",
          class: if idx == current {
            "col-start-1 row-start-1 size-full object-cover opacity-100 transition-opacity duration-200"
          } else {
            "col-start-1 row-start-1 size-full object-cover opacity-0 transition-opacity duration-200"
          },
        }
      }
      if total > 1 {
        div {
          class: "absolute inset-0 z-10 flex",
          for idx in 0..total {
            button {
              key: "zone-{idx}",
              r#type: "button",
              class: "h-full flex-1 cursor-pointer bg-transparent focus-visible:outline-none",
              "aria-label": "Show image {idx + 1} of {total}",
              "aria-pressed": (idx == current).to_string(),
              onmouseenter: move |_| active_index.set(idx),
              onfocus: move |_| active_index.set(idx),
              onclick: move |_| active_index.set(idx),
            }
          }
        }
      }
      if show_indicators && total > 1 {
        div {
          class: "pointer-events-none absolute inset-x-2 bottom-2 z-20 flex gap-1",
          "aria-hidden": "true",
          for idx in 0..total {
            span {
              key: "ind-{idx}",
              class: if idx == current {
                "h-1 flex-1 rounded-full bg-background/90 transition-colors"
              } else {
                "h-1 flex-1 rounded-full bg-background/40 transition-colors"
              },
            }
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dioxus_ssr::render_element;

  #[test]
  fn hover_gallery_renders_images_and_zones() {
    let images = vec![
      HoverGalleryImage::new("/a.jpg", "Front"),
      HoverGalleryImage::new("/b.jpg", "Side"),
      HoverGalleryImage::new("/c.jpg", "Back"),
    ];
    let html = render_element(rsx! {
      HoverGallery { images, class: "aspect-square w-64" }
    });
    assert!(html.contains("data-active-index=\"0\""));
    assert!(html.contains("Show image 2 of 3"));
    assert!(html.contains("opacity-100"));
    assert!(html.contains("opacity-0"));
  }
}
