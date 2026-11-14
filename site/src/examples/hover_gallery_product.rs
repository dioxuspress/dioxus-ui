use dioxus::prelude::*;
use dioxus_shadcn::{HoverGallery, HoverGalleryImage};

const FRAME_ONE: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 240'><rect width='320' height='240' fill='%232563eb'/><circle cx='160' cy='120' r='54' fill='%23ffffff' fill-opacity='0.25'/><text x='160' y='126' text-anchor='middle' font-family='sans-serif' font-size='20' fill='%23ffffff'>Front View</text></svg>";
const FRAME_TWO: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 240'><rect width='320' height='240' fill='%23059669'/><rect x='96' y='64' width='128' height='112' rx='18' fill='%23ffffff' fill-opacity='0.25'/><text x='160' y='126' text-anchor='middle' font-family='sans-serif' font-size='20' fill='%23ffffff'>Side Profile</text></svg>";
const FRAME_THREE: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 320 240'><rect width='320' height='240' fill='%237c3aed'/><polygon points='160,52 224,180 96,180' fill='%23ffffff' fill-opacity='0.25'/><text x='160' y='142' text-anchor='middle' font-family='sans-serif' font-size='20' fill='%23ffffff'>Detail Shot</text></svg>";

#[component]
pub fn HoverGalleryProductDemo() -> Element {
  rsx! {
    div { class: "flex flex-col items-start gap-2",
      HoverGallery {
        class: "aspect-4/3 w-72 border border-border",
        images: vec![
          HoverGalleryImage::new(FRAME_ONE, "Front view"),
          HoverGalleryImage::new(FRAME_TWO, "Side profile"),
          HoverGalleryImage::new(FRAME_THREE, "Detail shot"),
        ],
      }
      p { class: "text-xs text-muted-foreground", "Move pointer across the image to preview angles." }
    }
  }
}
