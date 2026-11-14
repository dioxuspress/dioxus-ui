//! Loading: animated loading indicators across spinner, dots, ring, ball, bars, and infinity styles.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// Animation style for `Loading`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LoadingVariant {
  /// Classic rotating arc spinner.
  #[default]
  Spinner,
  /// Three bouncing dots.
  Dots,
  /// Dual-arc concentric ring.
  Ring,
  /// Bouncing ball indicator.
  Ball,
  /// Three pulsing vertical bars.
  Bars,
  /// Continuous infinity loop.
  Infinity,
}

impl LoadingVariant {
  /// Value for the `data-variant` attribute.
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Spinner => "spinner",
      Self::Dots => "dots",
      Self::Ring => "ring",
      Self::Ball => "ball",
      Self::Bars => "bars",
      Self::Infinity => "infinity",
    }
  }
}

/// Size scale for `Loading`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LoadingSize {
  /// Extra-small indicator (`h-3 w-3`).
  Xs,
  /// Small indicator (`h-4 w-4`).
  Sm,
  /// Medium indicator (`h-5 w-5`, default).
  #[default]
  Md,
  /// Large indicator (`h-7 w-7`).
  Lg,
  /// Extra-large indicator (`h-10 w-10`).
  Xl,
}

impl LoadingSize {
  /// Tailwind sizing classes for this `LoadingSize`.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Xs => "h-3 w-3",
      Self::Sm => "h-4 w-4",
      Self::Md => "h-5 w-5",
      Self::Lg => "h-7 w-7",
      Self::Xl => "h-10 w-10",
    }
  }
}

const LOADING_BASE_CLASS: &str = "inline-flex shrink-0 items-center justify-center text-current";

/// Classes for `Loading`: base alignment, size, and user `class`.
pub fn loading_class(variant: LoadingVariant, size: LoadingSize, class: &str) -> String {
  let _ = variant;
  merge_classes(classes([Some(LOADING_BASE_CLASS), Some(size.class())]), class)
}

/// An accessible loading indicator supporting 6 daisyUI-style animation variants.
#[component]
pub fn Loading(
  #[props(default)] variant: LoadingVariant,
  #[props(default)] size: LoadingSize,
  #[props(default = "Loading".to_string())] label: String,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
  let class = loading_class(variant, size, &class);

  rsx! {
    span {
      class,
      role: "status",
      "aria-label": "{label}",
      "data-variant": variant.attribute(),
      ..attributes,
      match variant {
        LoadingVariant::Spinner => rsx! {
          span {
            "aria-hidden": "true",
            class: "h-full w-full animate-spin rounded-full border-2 border-current/25 border-t-current",
          }
        },
        LoadingVariant::Ring => rsx! {
          span {
            "aria-hidden": "true",
            class: "h-full w-full animate-spin rounded-full border-2 border-current border-b-transparent border-l-transparent",
          }
        },
        LoadingVariant::Dots => rsx! {
          span {
            "aria-hidden": "true",
            class: "flex h-full w-full items-center justify-between gap-0.5",
            span { class: "h-1.5 w-1.5 animate-bounce rounded-full bg-current [animation-delay:-0.3s]" }
            span { class: "h-1.5 w-1.5 animate-bounce rounded-full bg-current [animation-delay:-0.15s]" }
            span { class: "h-1.5 w-1.5 animate-bounce rounded-full bg-current" }
          }
        },
        LoadingVariant::Ball => rsx! {
          span {
            "aria-hidden": "true",
            class: "h-3/4 w-3/4 animate-bounce rounded-full bg-current",
          }
        },
        LoadingVariant::Bars => rsx! {
          span {
            "aria-hidden": "true",
            class: "flex h-full w-full items-end justify-between gap-0.5",
            span { class: "h-full w-1/4 animate-pulse rounded-xs bg-current [animation-delay:-0.3s]" }
            span { class: "h-3/4 w-1/4 animate-pulse rounded-xs bg-current [animation-delay:-0.15s]" }
            span { class: "h-full w-1/4 animate-pulse rounded-xs bg-current" }
          }
        },
        LoadingVariant::Infinity => rsx! {
          svg {
            "aria-hidden": "true",
            class: "h-full w-full animate-pulse",
            view_box: "0 0 24 12",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            path {
              d: "M6 9C4.343 9 3 7.657 3 6s1.343-3 3-3c2.5 0 4.5 6 7 6 1.657 0 3-1.343 3-3s-1.343-3-3-3c-2.5 0-4.5 6-7 6Z",
            }
          }
        },
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dioxus_ssr::render_element;

  #[test]
  fn loading_class_includes_size_and_custom_class() {
    let class = loading_class(LoadingVariant::Dots, LoadingSize::Lg, "text-primary");
    assert!(class.contains("h-7 w-7"));
    assert!(class.ends_with("text-primary"));
  }

  #[test]
  fn loading_renders_all_variants() {
    for variant in [
      LoadingVariant::Spinner,
      LoadingVariant::Dots,
      LoadingVariant::Ring,
      LoadingVariant::Ball,
      LoadingVariant::Bars,
      LoadingVariant::Infinity,
    ] {
      let html = render_element(rsx! {
        Loading { variant }
      });
      assert!(html.contains("role=\"status\""));
      assert!(html.contains(variant.attribute()));
    }
  }
}
