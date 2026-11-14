//! Hero: a full-width showcase section with centered content and optional overlay.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

const HERO_BASE_CLASS: &str = "relative grid min-h-80 w-full place-items-center overflow-hidden rounded-lg bg-muted/40 text-foreground";
const HERO_OVERLAY_BASE_CLASS: &str =
  "col-start-1 row-start-1 h-full w-full bg-background/60 backdrop-blur-xs";
const HERO_CONTENT_BASE_CLASS: &str = "relative z-1 col-start-1 row-start-1 flex max-w-4xl flex-col items-center justify-center gap-6 p-6 text-center md:flex-row md:text-left";

/// Classes for the `Hero` root container.
pub fn hero_class(class: &str) -> String {
  merge_classes(classes([Some(HERO_BASE_CLASS)]), class)
}

/// Classes for `HeroOverlay`.
pub fn hero_overlay_class(class: &str) -> String {
  merge_classes(classes([Some(HERO_OVERLAY_BASE_CLASS)]), class)
}

/// Classes for `HeroContent`.
pub fn hero_content_class(class: &str) -> String {
  merge_classes(classes([Some(HERO_CONTENT_BASE_CLASS)]), class)
}

/// A full-width hero showcase container.
#[component]
pub fn Hero(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = hero_class(&class);
  rsx! {
    section { class, ..attributes, {children} }
  }
}

/// Decorative backdrop overlay layered behind `HeroContent`.
#[component]
pub fn HeroOverlay(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
  let class = hero_overlay_class(&class);
  rsx! {
    div { class, "aria-hidden": "true", ..attributes }
  }
}

/// Foreground content container inside `Hero`.
#[component]
pub fn HeroContent(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = hero_content_class(&class);
  rsx! {
    div { class, ..attributes, {children} }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dioxus_ssr::render_element;

  #[test]
  fn hero_renders_overlay_and_content() {
    let html = render_element(rsx! {
      Hero {
        HeroOverlay {}
        HeroContent { h1 { "Launch Faster" } }
      }
    });
    assert!(html.contains("<section"));
    assert!(html.contains("aria-hidden=\"true\""));
    assert!(html.contains("Launch Faster"));
  }
}
