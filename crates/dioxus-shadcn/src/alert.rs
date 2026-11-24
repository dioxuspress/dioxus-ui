//! Alert: a callout for an important status or validation message, with
//! title and description parts.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// The tone of an alert, which sets its border, surface, and text colors.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AlertVariant {
  /// A neutral card for general notices.
  #[default]
  Default,
  /// An error or a failed action, in the destructive color.
  Destructive,
  /// A completed action, on a success tint.
  Success,
  /// Something that needs attention, on a warning tint.
  Warning,
  /// Neutral information, on an info tint.
  Info,
}

impl AlertVariant {
  /// The border, surface, and text colors for this tone.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Default => "border-border bg-card text-foreground",
      Self::Destructive => "border-destructive bg-card text-destructive",
      // The status colors are too light to be text (RFC 0058), so they tint
      // the surface and border instead.
      Self::Success => "border-success/50 bg-success/10 text-foreground",
      Self::Warning => "border-warning/50 bg-warning/10 text-foreground",
      Self::Info => "border-info/50 bg-info/10 text-foreground",
    }
  }
}

/// Visual surface style of an [`Alert`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AlertStyle {
  /// Standard alert appearance (default).
  #[default]
  Default,
  /// Soft tinted surface with subtle border and foreground text.
  Soft,
  /// Background surface with a solid semantic border and foreground text.
  Outline,
  /// Background surface with a dashed semantic border and foreground text.
  Dash,
  /// Full semantic background paired with its contrasting foreground token.
  Solid,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct AlertContext {
  style: AlertStyle,
}

const ALERT_BASE_CLASS: &str = "relative w-full rounded-md border p-4";
const ALERT_TITLE_BASE_CLASS: &str = "mb-1 font-medium leading-none tracking-normal";
const ALERT_DESCRIPTION_BASE_CLASS: &str = "text-sm";

const fn alert_surface_class(variant: AlertVariant, style: AlertStyle) -> &'static str {
  match style {
    AlertStyle::Default => variant.class(),
    AlertStyle::Soft => match variant {
      AlertVariant::Default => "border-border/60 bg-muted/50 text-foreground",
      AlertVariant::Destructive => "border-destructive/40 bg-destructive/10 text-foreground",
      AlertVariant::Success => "border-success/40 bg-success/10 text-foreground",
      AlertVariant::Warning => "border-warning/40 bg-warning/10 text-foreground",
      AlertVariant::Info => "border-info/40 bg-info/10 text-foreground",
    },
    AlertStyle::Outline => match variant {
      AlertVariant::Default => "border-border bg-background text-foreground",
      AlertVariant::Destructive => "border-destructive bg-background text-foreground",
      AlertVariant::Success => "border-success bg-background text-foreground",
      AlertVariant::Warning => "border-warning bg-background text-foreground",
      AlertVariant::Info => "border-info bg-background text-foreground",
    },
    AlertStyle::Dash => match variant {
      AlertVariant::Default => "border-dashed border-border bg-background text-foreground",
      AlertVariant::Destructive => "border-dashed border-destructive bg-background text-foreground",
      AlertVariant::Success => "border-dashed border-success bg-background text-foreground",
      AlertVariant::Warning => "border-dashed border-warning bg-background text-foreground",
      AlertVariant::Info => "border-dashed border-info bg-background text-foreground",
    },
    AlertStyle::Solid => match variant {
      AlertVariant::Default => "border-transparent bg-primary text-primary-foreground",
      AlertVariant::Destructive => "border-transparent bg-destructive text-destructive-foreground",
      AlertVariant::Success => "border-transparent bg-success text-success-foreground",
      AlertVariant::Warning => "border-transparent bg-warning text-warning-foreground",
      AlertVariant::Info => "border-transparent bg-info text-info-foreground",
    },
  }
}

/// Classes for the alert box: base classes, the variant's colors, then `class` merged
/// over them.
pub fn alert_class(variant: AlertVariant, class: &str) -> String {
  merge_classes(classes([Some(ALERT_BASE_CLASS), Some(variant.class())]), class)
}

/// Classes for an [`Alert`] with explicit `variant` and `style`.
pub fn alert_styled_class(variant: AlertVariant, style: AlertStyle, class: &str) -> String {
  merge_classes(classes([Some(ALERT_BASE_CLASS), Some(alert_surface_class(variant, style))]), class)
}

/// Classes for the title: base classes with `class` merged over them.
pub fn alert_title_class(class: &str) -> String {
  merge_classes(classes([Some(ALERT_TITLE_BASE_CLASS)]), class)
}

/// Classes for the description: base classes, a text color that suits the variant,
/// then `class` merged over them.
pub fn alert_description_class(variant: AlertVariant, class: &str) -> String {
  let variant_class = match variant {
    AlertVariant::Default => "text-muted-foreground",
    AlertVariant::Destructive => "text-destructive",
    // Muted text is checked only on plain surfaces, not on the status tints.
    AlertVariant::Success | AlertVariant::Warning | AlertVariant::Info => "text-foreground",
  };

  merge_classes(classes([Some(ALERT_DESCRIPTION_BASE_CLASS), Some(variant_class)]), class)
}

/// Callout banner that announces an important status or validation message.
#[component]
pub fn Alert(
  #[props(default)] variant: AlertVariant,
  #[props(default)] style: AlertStyle,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  use_context_provider(|| AlertContext { style });
  let class = if style == AlertStyle::Default {
    alert_class(variant, &class)
  } else {
    alert_styled_class(variant, style, &class)
  };

  rsx! {
    div {
      role: "alert",
      class,
      {children}
    }
  }
}

/// Title section of an [`Alert`].
#[component]
pub fn AlertTitle(#[props(default)] class: String, children: Element) -> Element {
  let class = alert_title_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// Supporting body copy of an [`Alert`].
#[component]
pub fn AlertDescription(
  #[props(default)] variant: AlertVariant,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let ctx = try_use_context::<AlertContext>().unwrap_or_default();
  let class = match ctx.style {
    AlertStyle::Default => alert_description_class(variant, &class),
    AlertStyle::Solid => {
      merge_classes(classes([Some(ALERT_DESCRIPTION_BASE_CLASS), Some("text-current")]), &class)
    }
    AlertStyle::Soft | AlertStyle::Outline | AlertStyle::Dash => {
      merge_classes(classes([Some(ALERT_DESCRIPTION_BASE_CLASS), Some("text-foreground")]), &class)
    }
  };

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_title_is_not_a_heading() {
    fn app() -> Element {
      rsx! { AlertTitle { "Heads up" } }
    }
    let html = render(app);

    assert!(html.starts_with("<div"));
    assert!(!html.contains("<h"));
  }

  #[test]
  fn alert_class_includes_variant_and_user_class() {
    let actual = alert_class(AlertVariant::Destructive, "mt-4");

    assert!(actual.contains(ALERT_BASE_CLASS));
    assert!(actual.contains("border-destructive bg-card text-destructive"));
    assert!(actual.ends_with("mt-4"));
  }

  #[test]
  fn status_alerts_tint_the_surface_and_keep_foreground_text() {
    for (variant, expected) in [
      (AlertVariant::Success, "border-success/50 bg-success/10 text-foreground"),
      (AlertVariant::Warning, "border-warning/50 bg-warning/10 text-foreground"),
      (AlertVariant::Info, "border-info/50 bg-info/10 text-foreground"),
    ] {
      let actual = alert_class(variant, "");

      assert!(actual.contains(expected));
      assert!(!actual.contains("bg-card"));
    }
  }

  #[test]
  fn alert_description_class_reflects_variant() {
    let actual = alert_description_class(AlertVariant::Destructive, "leading-6");

    assert!(actual.contains(ALERT_DESCRIPTION_BASE_CLASS));
    assert!(actual.contains("text-destructive"));
    assert!(actual.ends_with("leading-6"));
  }

  #[test]
  fn alert_styled_class_supports_soft_outline_dash_and_solid() {
    let soft = alert_styled_class(AlertVariant::Info, AlertStyle::Soft, "");
    let dash = alert_styled_class(AlertVariant::Warning, AlertStyle::Dash, "");
    let solid = alert_styled_class(AlertVariant::Success, AlertStyle::Solid, "");

    assert!(soft.contains("bg-info/10 text-foreground"));
    assert!(dash.contains("border-dashed border-warning"));
    assert!(solid.contains("bg-success text-success-foreground"));
  }

  #[test]
  fn solid_alert_description_inherits_contrasting_foreground() {
    fn app() -> Element {
      rsx! {
        Alert { variant: AlertVariant::Destructive, style: AlertStyle::Solid,
          AlertTitle { "Outage" }
          AlertDescription { "Write traffic is paused." }
        }
      }
    }
    let html = render(app);

    assert!(html.contains("bg-destructive text-destructive-foreground"), "{html}");
    assert!(html.contains("text-current"), "{html}");
    assert!(!html.contains("text-muted-foreground"), "{html}");
  }
}
