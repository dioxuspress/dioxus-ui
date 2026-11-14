//! Aura: soft semantic glow backdrop for spotlighting hero cards, badges, and feature panels.

use dioxus::prelude::*;
use super::utils::{classes, merge_classes};

/// Semantic color tone for the `Aura` glow layers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AuraTone {
  /// Uses the theme's primary and secondary accents (default).
  #[default]
  Primary,
  /// Uses secondary and accent tokens.
  Secondary,
  /// Uses accent and primary tokens.
  Accent,
  /// Uses destructive and warning-like tinting.
  Destructive,
}

impl AuraTone {
  /// Primary glow blob class using RFC 0051 semantic tokens.
  pub const fn primary_blob_class(self) -> &'static str {
    match self {
      Self::Primary => "bg-primary/25",
      Self::Secondary => "bg-secondary/40",
      Self::Accent => "bg-accent/40",
      Self::Destructive => "bg-destructive/20",
    }
  }

  /// Secondary glow blob class using RFC 0051 semantic tokens.
  pub const fn secondary_blob_class(self) -> &'static str {
    match self {
      Self::Primary => "bg-accent/25",
      Self::Secondary => "bg-primary/20",
      Self::Accent => "bg-secondary/35",
      Self::Destructive => "bg-primary/15",
    }
  }
}

/// Spread size of the `Aura` ambient glow.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AuraSize {
  /// Subtle tight glow.
  Sm,
  /// Balanced ambient glow (default).
  #[default]
  Md,
  /// Wide hero-scale glow.
  Lg,
}

impl AuraSize {
  /// Blur and inset sizing classes for the aura backdrop.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "-inset-2 blur-xl",
      Self::Md => "-inset-4 blur-2xl",
      Self::Lg => "-inset-8 blur-3xl",
    }
  }
}

const AURA_BASE_CLASS: &str = "relative isolate";

/// Classes for the `Aura` wrapper.
pub fn aura_class(class: &str) -> String {
  merge_classes(classes([Some(AURA_BASE_CLASS)]), class)
}

/// Wraps content with a soft, semantic-token ambient glow behind it.
#[component]
pub fn Aura(
  #[props(default)] tone: AuraTone,
  #[props(default)] size: AuraSize,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = aura_class(&class);
  let backdrop_size = size.class();
  let primary_blob = tone.primary_blob_class();
  let secondary_blob = tone.secondary_blob_class();

  rsx! {
    div {
      class,
      "data-aura-tone": format!("{tone:?}").to_lowercase(),
      ..attributes,
      div {
        class: "pointer-events-none absolute {backdrop_size} -z-10 overflow-hidden rounded-3xl",
        "aria-hidden": "true",
        div { class: "absolute -top-1/4 -start-1/4 size-3/4 rounded-full {primary_blob}" }
        div { class: "absolute -bottom-1/4 -end-1/4 size-3/4 rounded-full {secondary_blob}" }
      }
      {children}
    }
  }
}
