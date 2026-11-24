//! Badge: a compact indicator of a status or category.
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// The colors of a `Badge`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BadgeVariant {
  /// The primary color.
  #[default]
  Default,
  /// The secondary color, for less emphasis.
  Secondary,
  /// The destructive color, for errors and removals.
  Destructive,
  /// A border and no fill.
  Outline,
  /// The success color.
  Success,
  /// The warning color.
  Warning,
  /// The info color.
  Info,
}

impl BadgeVariant {
  /// The variant's solid border, background, and text color classes.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => "border-transparent bg-primary text-primary-foreground",
      Self::Secondary => "border-transparent bg-secondary text-secondary-foreground",
      Self::Destructive => "border-transparent bg-destructive text-destructive-foreground",
      Self::Outline => "border-border text-foreground",
      Self::Success => "border-transparent bg-success text-success-foreground",
      Self::Warning => "border-transparent bg-warning text-warning-foreground",
      Self::Info => "border-transparent bg-info text-info-foreground",
    }
  }
}

/// Visual surface style of a [`Badge`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BadgeStyle {
  /// Standard filled surface (default).
  #[default]
  Solid,
  /// Subtle tinted surface with foreground text (RFC 0058 safe across presets).
  Soft,
  /// Transparent surface with a solid semantic border and foreground text.
  Outline,
  /// Transparent surface with a dashed semantic border and foreground text.
  Dash,
}

/// Padding and text scale of a [`Badge`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BadgeSize {
  /// Extra-small badge for dense counts.
  Xs,
  /// Small badge with tighter horizontal padding.
  Sm,
  /// Medium badge (default).
  #[default]
  Md,
  /// Large badge for prominent tags.
  Lg,
}

impl BadgeSize {
  /// The size's padding and text classes.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Xs => "px-1.5 py-0 text-[10px]",
      Self::Sm => "px-1.5 py-0.5 text-xs",
      Self::Md => "px-2 py-0.5 text-xs",
      Self::Lg => "px-2.5 py-1 text-sm",
    }
  }
}

const BADGE_BASE_CLASS: &str =
  "inline-flex items-center rounded-md border px-2 py-0.5 text-xs font-semibold transition-colors";
const BADGE_CORE_CLASS: &str = "inline-flex items-center border font-semibold transition-colors";

const fn badge_tone_class(variant: BadgeVariant, style: BadgeStyle) -> &'static str {
  match style {
    BadgeStyle::Solid => variant.class(),
    BadgeStyle::Soft => match variant {
      BadgeVariant::Default => "border-primary/30 bg-primary/10 text-foreground",
      BadgeVariant::Secondary => "border-border bg-muted/50 text-foreground",
      BadgeVariant::Destructive => "border-destructive/30 bg-destructive/10 text-foreground",
      BadgeVariant::Outline => "border-border bg-muted/30 text-foreground",
      BadgeVariant::Success => "border-success/40 bg-success/10 text-foreground",
      BadgeVariant::Warning => "border-warning/40 bg-warning/10 text-foreground",
      BadgeVariant::Info => "border-info/40 bg-info/10 text-foreground",
    },
    BadgeStyle::Outline => match variant {
      BadgeVariant::Default => "border-primary bg-transparent text-foreground",
      BadgeVariant::Secondary => "border-secondary bg-transparent text-foreground",
      BadgeVariant::Destructive => "border-destructive bg-transparent text-foreground",
      BadgeVariant::Outline => "border-border bg-transparent text-foreground",
      BadgeVariant::Success => "border-success bg-transparent text-foreground",
      BadgeVariant::Warning => "border-warning bg-transparent text-foreground",
      BadgeVariant::Info => "border-info bg-transparent text-foreground",
    },
    BadgeStyle::Dash => match variant {
      BadgeVariant::Default => "border-dashed border-primary bg-transparent text-foreground",
      BadgeVariant::Secondary => "border-dashed border-secondary bg-transparent text-foreground",
      BadgeVariant::Destructive => {
        "border-dashed border-destructive bg-transparent text-foreground"
      }
      BadgeVariant::Outline => "border-dashed border-border bg-transparent text-foreground",
      BadgeVariant::Success => "border-dashed border-success bg-transparent text-foreground",
      BadgeVariant::Warning => "border-dashed border-warning bg-transparent text-foreground",
      BadgeVariant::Info => "border-dashed border-info bg-transparent text-foreground",
    },
  }
}

/// Classes for the badge: base classes, the variant's, then `class` merged over them.
pub fn badge_class(variant: BadgeVariant, class: &str) -> String {
  merge_classes(classes([Some(BADGE_BASE_CLASS), Some(variant.class())]), class)
}

/// Classes for a [`Badge`] with explicit `variant`, `style`, `size`, and `pill` shape.
pub fn badge_styled_class(
  variant: BadgeVariant,
  style: BadgeStyle,
  size: BadgeSize,
  pill: bool,
  class: &str,
) -> String {
  let radius = if pill { "rounded-full" } else { "rounded-md" };

  merge_classes(
    classes([
      Some(BADGE_CORE_CLASS),
      Some(radius),
      Some(size.class()),
      Some(badge_tone_class(variant, style)),
    ]),
    class,
  )
}

/// Compact indicator of a status, count, or category.
#[component]
pub fn Badge(
  #[props(default)] variant: BadgeVariant,
  #[props(default)] style: BadgeStyle,
  #[props(default)] size: BadgeSize,
  #[props(default)] pill: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = if style == BadgeStyle::Solid && size == BadgeSize::Md && !pill {
    badge_class(variant, &class)
  } else {
    badge_styled_class(variant, style, size, pill, &class)
  };

  rsx! {
    span {
      class,
      {children}
    }
  }
}
