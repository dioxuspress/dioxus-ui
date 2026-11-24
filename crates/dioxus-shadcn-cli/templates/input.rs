//! Input: a styled single-line text field with an invalid state.
use super::density::{density_control_class, use_density, with_density};
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// Height and text scale of an [`Input`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InputSize {
  /// Compact 32-pixel input.
  Sm,
  /// Standard 40-pixel input (default).
  #[default]
  Md,
  /// Spacious 48-pixel input.
  Lg,
}

impl InputSize {
  /// The size's height, padding, and text classes.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "h-8 px-2.5 py-1 text-xs",
      Self::Md => "h-10 px-3 py-2 text-sm",
      Self::Lg => "h-12 px-4 py-2.5 text-base",
    }
  }
}

/// Surface and border style of an [`Input`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InputVariant {
  /// Standard bordered background field (default).
  #[default]
  Default,
  /// Borderless field that reveals its border and surface on focus.
  Ghost,
  /// Subtle muted fill that brightens to background on focus.
  Soft,
}

const INPUT_BASE_CLASS: &str = "flex h-10 w-full rounded-md border bg-background px-3 py-2 text-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
const INPUT_CORE_CLASS: &str = "flex w-full rounded-md border transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";

const fn input_surface_class(variant: InputVariant, invalid: bool) -> &'static str {
  if invalid {
    match variant {
      InputVariant::Default => "bg-background border-destructive focus-visible:ring-destructive",
      InputVariant::Ghost => {
        "bg-transparent hover:bg-muted/40 focus-visible:bg-background border-destructive focus-visible:ring-destructive"
      }
      InputVariant::Soft => {
        "bg-muted/60 focus-visible:bg-background border-destructive focus-visible:ring-destructive"
      }
    }
  } else {
    match variant {
      InputVariant::Default => "bg-background border-input focus-visible:ring-ring",
      InputVariant::Ghost => {
        "bg-transparent hover:bg-muted/40 focus-visible:bg-background border-transparent focus-visible:border-input focus-visible:ring-ring"
      }
      InputVariant::Soft => {
        "bg-muted/60 focus-visible:bg-background border-transparent focus-visible:border-input focus-visible:ring-ring"
      }
    }
  }
}

/// Classes for the field: base classes, a destructive border and ring when `invalid`,
/// then `class` merged over them.
pub fn input_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(INPUT_BASE_CLASS), Some(invalid_class)]), class)
}

/// Classes for an [`Input`] with explicit `size`, `variant`, and `invalid` state.
pub fn input_styled_class(
  size: InputSize,
  variant: InputVariant,
  invalid: bool,
  class: &str,
) -> String {
  merge_classes(
    classes([
      Some(INPUT_CORE_CLASS),
      Some(size.class()),
      Some(input_surface_class(variant, invalid)),
    ]),
    class,
  )
}

/// A controlled input. Each `input` event calls `on_value_change` with the new
/// text; the app passes it back as `value`. Other attributes, such as `id`,
/// `name`, and `aria-describedby`, are passed to the input.
#[component]
pub fn Input(
  #[props(default)] value: String,
  #[props(default)] placeholder: String,
  #[props(default)] size: InputSize,
  #[props(default)] variant: InputVariant,
  #[props(default)] class: String,
  #[props(default)] disabled: bool,
  #[props(default)] invalid: bool,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let merged_user = with_density(density_control_class(use_density()), &class);
  let class = if size == InputSize::Md && variant == InputVariant::Default {
    input_class(invalid, &merged_user)
  } else {
    input_styled_class(size, variant, invalid, &merged_user)
  };

  rsx! {
    input {
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
