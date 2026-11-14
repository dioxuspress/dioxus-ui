//! Mask: crops images, avatars, or decorative containers into geometric shapes.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// Geometric clip-path shape applied by `Mask`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MaskShape {
  /// Rounded superellipse (squircle).
  #[default]
  Squircle,
  /// Full circle.
  Circle,
  /// Square with slight corner inset.
  Square,
  /// Rotated diamond.
  Diamond,
  /// Upward-pointing triangle.
  Triangle,
  /// Downward-pointing triangle.
  Triangle2,
  /// Left-pointing triangle.
  Triangle3,
  /// Right-pointing triangle.
  Triangle4,
  /// Regular pentagon.
  Pentagon,
  /// Pointy-top hexagon.
  Hexagon,
  /// Flat-top hexagon.
  Hexagon2,
  /// Regular decagon.
  Decagon,
  /// Classic 5-point star.
  Star,
  /// Rounded 5-point star.
  Star2,
  /// Heart silhouette.
  Heart,
}

impl MaskShape {
  /// CSS `clip-path` value for this shape in percentage coordinates.
  pub const fn clip_path(self) -> &'static str {
    match self {
      Self::Squircle => {
        "polygon(50% 0%, 72% 2%, 88% 12%, 98% 28%, 100% 50%, 98% 72%, 88% 88%, 72% 98%, 50% 100%, 28% 98%, 12% 88%, 2% 72%, 0% 50%, 2% 28%, 12% 12%, 28% 2%)"
      }
      Self::Circle => "circle(50% at 50% 50%)",
      Self::Square => "inset(0% round 6%)",
      Self::Diamond => "polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%)",
      Self::Triangle => "polygon(50% 0%, 100% 100%, 0% 100%)",
      Self::Triangle2 => "polygon(0% 0%, 100% 0%, 50% 100%)",
      Self::Triangle3 => "polygon(0% 50%, 100% 0%, 100% 100%)",
      Self::Triangle4 => "polygon(0% 0%, 100% 50%, 0% 100%)",
      Self::Pentagon => "polygon(50% 0%, 100% 38%, 81% 100%, 19% 100%, 0% 38%)",
      Self::Hexagon => "polygon(50% 0%, 95% 25%, 95% 75%, 50% 100%, 5% 75%, 5% 25%)",
      Self::Hexagon2 => "polygon(25% 5%, 75% 5%, 100% 50%, 75% 95%, 25% 95%, 0% 50%)",
      Self::Decagon => {
        "polygon(50% 0%, 79% 10%, 98% 35%, 98% 65%, 79% 90%, 50% 100%, 21% 90%, 2% 65%, 2% 35%, 21% 10%)"
      }
      Self::Star => {
        "polygon(50% 0%, 61% 35%, 98% 35%, 68% 57%, 79% 91%, 50% 70%, 21% 91%, 32% 57%, 2% 35%, 39% 35%)"
      }
      Self::Star2 => {
        "polygon(50% 2%, 63% 34%, 97% 36%, 71% 58%, 80% 92%, 50% 73%, 20% 92%, 29% 58%, 3% 36%, 37% 34%)"
      }
      Self::Heart => {
        "polygon(50% 92%, 14% 62%, 3% 44%, 3% 26%, 14% 12%, 30% 8%, 44% 15%, 50% 24%, 56% 15%, 70% 8%, 86% 12%, 97% 26%, 97% 44%, 86% 62%)"
      }
    }
  }
}

/// Optional half-mask crop applied inside `Mask` (useful for half-ratings or split visuals).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MaskHalf {
  /// Show the full shape.
  #[default]
  None,
  /// Keep only the inline-start (left) half.
  First,
  /// Keep only the inline-end (right) half.
  Second,
}

impl MaskHalf {
  /// Optional CSS `clip-path` inset for half cropping.
  pub const fn clip_path(self) -> Option<&'static str> {
    match self {
      Self::None => None,
      Self::First => Some("inset(0 50% 0 0)"),
      Self::Second => Some("inset(0 0 0 50%)"),
    }
  }
}

const MASK_BASE_CLASS: &str = "inline-block overflow-hidden align-middle";

/// Classes for the `Mask` wrapper, with `class` merged over the base layout.
pub fn mask_class(class: &str) -> String {
  merge_classes(classes([Some(MASK_BASE_CLASS)]), class)
}

/// Inline CSS style string applying `MaskShape` via `clip-path`.
pub fn mask_style(shape: MaskShape) -> String {
  format!("clip-path: {};", shape.clip_path())
}

/// Crops its children into a geometric `MaskShape` using CSS `clip-path`.
#[component]
pub fn Mask(
  #[props(default)] shape: MaskShape,
  #[props(default)] half: MaskHalf,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = mask_class(&class);
  let outer_style = mask_style(shape);

  rsx! {
    span {
      class,
      style: outer_style,
      "data-mask-shape": format!("{shape:?}").to_lowercase(),
      ..attributes,
      if let Some(half_clip) = half.clip_path() {
        span { class: "block size-full", style: "clip-path: {half_clip};", {children} }
      } else {
        {children}
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dioxus_ssr::render_element;

  #[test]
  fn mask_class_merges_user_classes() {
    assert_eq!(mask_class("size-16"), "inline-block overflow-hidden align-middle size-16");
  }

  #[test]
  fn mask_renders_shape_clip_path_and_half() {
    let html = render_element(rsx! {
      Mask { shape: MaskShape::Heart, half: MaskHalf::First, class: "size-12 bg-primary",
        span { "A" }
      }
    });
    assert!(html.contains("data-mask-shape=\"heart\""));
    assert!(html.contains("clip-path: polygon("));
    assert!(html.contains("inset(0 50% 0 0)"));
  }
}
