//! Textarea: a styled multi-line text field.

use crate::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// Minimum height and text scale of a [`Textarea`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextareaSize {
  /// Compact 64-pixel minimum height.
  Sm,
  /// Standard 96-pixel minimum height (default).
  #[default]
  Md,
  /// Spacious 128-pixel minimum height.
  Lg,
}

impl TextareaSize {
  /// The size's minimum height, padding, and text classes.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "min-h-16 px-2.5 py-1.5 text-xs",
      Self::Md => "min-h-24 px-3 py-2 text-sm",
      Self::Lg => "min-h-32 px-4 py-3 text-base",
    }
  }
}

/// Surface and border style of a [`Textarea`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextareaVariant {
  /// Standard bordered background field (default).
  #[default]
  Default,
  /// Borderless field that reveals its border and surface on focus.
  Ghost,
  /// Subtle muted fill that brightens to background on focus.
  Soft,
}

const TEXTAREA_BASE_CLASS: &str = "flex min-h-24 w-full rounded-md border bg-background px-3 py-2 text-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
const TEXTAREA_CORE_CLASS: &str = "flex w-full rounded-md border transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

const fn textarea_surface_class(variant: TextareaVariant, invalid: bool) -> &'static str {
  if invalid {
    match variant {
      TextareaVariant::Default => "bg-background border-destructive focus-visible:ring-destructive",
      TextareaVariant::Ghost => {
        "bg-transparent hover:bg-muted/40 focus-visible:bg-background border-destructive focus-visible:ring-destructive"
      }
      TextareaVariant::Soft => {
        "bg-muted/60 focus-visible:bg-background border-destructive focus-visible:ring-destructive"
      }
    }
  } else {
    match variant {
      TextareaVariant::Default => "bg-background border-input focus-visible:ring-ring",
      TextareaVariant::Ghost => {
        "bg-transparent hover:bg-muted/40 focus-visible:bg-background border-transparent focus-visible:border-input focus-visible:ring-ring"
      }
      TextareaVariant::Soft => {
        "bg-muted/60 focus-visible:bg-background border-transparent focus-visible:border-input focus-visible:ring-ring"
      }
    }
  }
}

/// Classes for the textarea: base classes, the destructive border and ring when
/// `invalid` or the plain ones otherwise, then `class` merged over them.
pub fn textarea_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(TEXTAREA_BASE_CLASS), Some(invalid_class)]), class)
}

/// Classes for a [`Textarea`] with explicit `size`, `variant`, and `invalid` state.
pub fn textarea_styled_class(
  size: TextareaSize,
  variant: TextareaVariant,
  invalid: bool,
  class: &str,
) -> String {
  merge_classes(
    classes([
      Some(TEXTAREA_CORE_CLASS),
      Some(size.class()),
      Some(textarea_surface_class(variant, invalid)),
    ]),
    class,
  )
}

/// A controlled textarea. Each `input` event calls `on_value_change` with the new
/// text; the app passes it back as `value`. Other attributes, such as `id`,
/// `name`, and `aria-describedby`, are passed to the textarea.
#[component]
pub fn Textarea(
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default)] size: TextareaSize,
  #[props(default)] variant: TextareaVariant,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(extends = GlobalAttributes, extends = textarea)] attributes: Vec<Attribute>,
) -> Element {
  let merged_user = with_density(density_control_class(use_density()), &class);
  let class = if size == TextareaSize::Md && variant == TextareaVariant::Default {
    textarea_class(invalid, &merged_user)
  } else {
    textarea_styled_class(size, variant, invalid, &merged_user)
  };

  rsx! {
    textarea {
      class,
      value,
      placeholder,
      disabled,
      "aria-invalid": invalid.to_string(),
      oninput: move |event: FormEvent| {
        if let Some(handler) = on_value_change {
          handler.call(event.value());
        }
      },
      ..attributes,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn textarea_class_adds_invalid_state() {
    let actual = textarea_class(true, "resize-none");

    assert!(actual.contains(TEXTAREA_BASE_CLASS));
    assert!(actual.contains("border-destructive focus-visible:ring-destructive"));
    assert!(actual.ends_with("resize-none"));
  }

  #[test]
  fn textarea_styled_class_supports_sizes_and_variants() {
    let sm_soft = textarea_styled_class(TextareaSize::Sm, TextareaVariant::Soft, false, "");
    let lg_ghost = textarea_styled_class(TextareaSize::Lg, TextareaVariant::Ghost, true, "w-72");

    assert!(sm_soft.contains("min-h-16 px-2.5 py-1.5 text-xs"));
    assert!(sm_soft.contains("bg-muted/60"));
    assert!(lg_ghost.contains("min-h-32 px-4 py-3 text-base"));
    assert!(lg_ghost.contains("bg-transparent"));
    assert!(lg_ghost.contains("border-destructive"));
    assert!(lg_ghost.ends_with("w-72"));
  }
}
