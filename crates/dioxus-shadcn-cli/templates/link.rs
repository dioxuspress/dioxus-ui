//! Link: styled anchor links with semantic color variants and underline modes.

use super::safe_url::safe_href;
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// Visual color variant for `Link`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LinkVariant {
  /// Inherits surrounding text color with primary hover accent.
  #[default]
  Default,
  /// Primary brand color.
  Primary,
  /// Secondary accent color.
  Secondary,
  /// Highlight accent color.
  Accent,
  /// Neutral foreground color.
  Neutral,
  /// Subdued muted foreground color.
  Muted,
  /// Positive semantic color.
  Success,
  /// Warning semantic color.
  Warning,
  /// Destructive semantic color.
  Destructive,
}

impl LinkVariant {
  /// Tailwind classes for this variant.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => "text-foreground hover:text-primary",
      Self::Primary => "text-primary hover:text-primary/80",
      Self::Secondary => "text-secondary-foreground hover:text-secondary-foreground/80",
      Self::Accent => "text-foreground decoration-accent hover:text-foreground/80",
      Self::Neutral => "text-foreground/80 hover:text-foreground",
      Self::Muted => "text-muted-foreground hover:text-foreground",
      Self::Success => "text-foreground decoration-success hover:text-foreground/80",
      Self::Warning => "text-foreground decoration-warning hover:text-foreground/80",
      Self::Destructive => "text-destructive hover:text-destructive/80",
    }
  }
}

/// Underline behavior for `Link`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LinkUnderline {
  /// Always underlined.
  #[default]
  Always,
  /// Underlined only on hover or focus-visible.
  Hover,
  /// Never underlined.
  None,
}

impl LinkUnderline {
  /// Tailwind classes for this underline mode.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Always => "underline underline-offset-4",
      Self::Hover => "no-underline hover:underline focus-visible:underline underline-offset-4",
      Self::None => "no-underline",
    }
  }
}

const LINK_BASE_CLASS: &str =
  "inline-flex items-center gap-1 font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring rounded-xs";

/// Classes for `Link`: base styles, variant, underline mode, and custom `class`.
pub fn link_class(variant: LinkVariant, underline: LinkUnderline, class: &str) -> String {
  merge_classes(
    classes([Some(LINK_BASE_CLASS), Some(variant.class()), Some(underline.class())]),
    class,
  )
}

/// A styled anchor link that filters unsafe URL schemes.
#[component]
pub fn Link(
  href: String,
  #[props(default)] variant: LinkVariant,
  #[props(default)] underline: LinkUnderline,
  #[props(default)] external: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = link_class(variant, underline, &class);
  let resolved_href = safe_href(href);

  rsx! {
    a {
      class,
      href: resolved_href,
      target: external.then_some("_blank"),
      rel: external.then_some("noopener noreferrer"),
      ..attributes,
      {children}
    }
  }
}
