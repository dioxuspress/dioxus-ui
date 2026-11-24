//! Progress: a bar that shows how much of a bounded task is done.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// Fill tone of a [`Progress`] bar indicator.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProgressVariant {
  /// Primary fill (default).
  #[default]
  Default,
  /// Success fill for completed or healthy progress.
  Success,
  /// Warning fill for elevated usage or attention thresholds.
  Warning,
  /// Destructive fill for critical or over-quota usage.
  Destructive,
  /// Informational fill.
  Info,
}

impl ProgressVariant {
  /// The indicator background class for this tone.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => "bg-primary",
      Self::Success => "bg-success",
      Self::Warning => "bg-warning",
      Self::Destructive => "bg-destructive",
      Self::Info => "bg-info",
    }
  }
}

/// Track height of a [`Progress`] bar.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProgressSize {
  /// Hairline 6px track.
  Xs,
  /// Compact 10px track.
  Sm,
  /// Standard 16px track (default).
  #[default]
  Md,
  /// Prominent 20px track.
  Lg,
}

impl ProgressSize {
  /// The track height class for this size.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Xs => "h-1.5",
      Self::Sm => "h-2.5",
      Self::Md => "h-4",
      Self::Lg => "h-5",
    }
  }
}

const PROGRESS_BASE_CLASS: &str = "relative h-4 w-full overflow-hidden rounded-full bg-muted";
const PROGRESS_CORE_CLASS: &str = "relative w-full overflow-hidden rounded-full bg-muted";
const PROGRESS_INDICATOR_BASE_CLASS: &str = "h-full w-full flex-1 bg-primary transition-transform";
const PROGRESS_INDICATOR_CORE_CLASS: &str = "h-full w-full flex-1 transition-transform";

/// Classes for the track: base classes with `class` merged over them.
pub fn progress_class(class: &str) -> String {
  merge_classes(classes([Some(PROGRESS_BASE_CLASS)]), class)
}

/// Classes for the track with an explicit [`ProgressSize`].
pub fn progress_sized_class(size: ProgressSize, class: &str) -> String {
  merge_classes(classes([Some(PROGRESS_CORE_CLASS), Some(size.class())]), class)
}

/// Classes for the filled indicator inside the track: base classes with `class`
/// merged over them.
pub fn progress_indicator_class(class: &str) -> String {
  merge_classes(classes([Some(PROGRESS_INDICATOR_BASE_CLASS)]), class)
}

/// Classes for the filled indicator with an explicit [`ProgressVariant`].
pub fn progress_indicator_variant_class(variant: ProgressVariant, class: &str) -> String {
  merge_classes(classes([Some(PROGRESS_INDICATOR_CORE_CLASS), Some(variant.class())]), class)
}

fn progress_percent(value: f32, max: f32) -> f32 {
  if max <= 0.0 {
    return 0.0;
  }

  (value / max * 100.0).clamp(0.0, 100.0)
}

/// Horizontal progress bar with semantic tone and track size options.
#[component]
pub fn Progress(
  #[props(default)] value: f32,
  #[props(default = 100.0)] max: f32,
  #[props(default)] variant: ProgressVariant,
  #[props(default)] size: ProgressSize,
  #[props(default)] class: String,
  #[props(default)] indicator_class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
  let class = if size == ProgressSize::Md {
    progress_class(&class)
  } else {
    progress_sized_class(size, &class)
  };
  let indicator_class = if variant == ProgressVariant::Default {
    progress_indicator_class(&indicator_class)
  } else {
    progress_indicator_variant_class(variant, &indicator_class)
  };
  let percent = progress_percent(value, max);
  let transform = format!("translateX(-{}%)", 100.0 - percent);

  rsx! {
    div {
      role: "progressbar",
      class,
      "aria-valuemin": "0",
      "aria-valuemax": max.to_string(),
      "aria-valuenow": value.clamp(0.0, max).to_string(),
      ..attributes,
      div {
        class: indicator_class,
        style: "transform: {transform};",
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn progress_class_appends_user_class() {
    let actual = progress_class("h-2");

    assert_eq!(actual, "relative w-full overflow-hidden rounded-full bg-muted h-2");
    assert!(actual.ends_with("h-2"));
  }

  #[test]
  fn progress_percent_clamps_values() {
    assert_eq!(progress_percent(25.0, 100.0), 25.0);
    assert_eq!(progress_percent(150.0, 100.0), 100.0);
    assert_eq!(progress_percent(-10.0, 100.0), 0.0);
    assert_eq!(progress_percent(10.0, 0.0), 0.0);
  }

  #[test]
  fn progress_supports_semantic_variants_and_sizes() {
    let track = progress_sized_class(ProgressSize::Xs, "");
    let indicator = progress_indicator_variant_class(ProgressVariant::Success, "");

    assert!(track.contains("h-1.5"));
    assert!(indicator.contains("bg-success"));
  }
}
