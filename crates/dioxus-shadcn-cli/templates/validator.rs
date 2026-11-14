//! Validator: declarative form validation wrapper and hint message.

use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// Validation state for `Validator` and `ValidatorHint`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ValidatorState {
  /// Default neutral state before validation or without errors.
  #[default]
  Neutral,
  /// Validated state.
  Valid,
  /// Invalid state highlighting the control and hint.
  Invalid,
}

impl ValidatorState {
  /// Value for the `data-state` attribute.
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Neutral => "neutral",
      Self::Valid => "valid",
      Self::Invalid => "invalid",
    }
  }

  /// Tailwind classes applied to the `Validator` container.
  pub const fn container_class(self) -> &'static str {
    match self {
      Self::Neutral => "",
      Self::Valid => {
        "[&>input]:border-primary [&>textarea]:border-primary [&>select]:border-primary"
      }
      Self::Invalid => {
        "[&>input]:border-destructive [&>textarea]:border-destructive [&>select]:border-destructive"
      }
    }
  }

  /// Tailwind classes applied to `ValidatorHint`.
  pub const fn hint_class(self) -> &'static str {
    match self {
      Self::Neutral => "text-muted-foreground",
      Self::Valid => "text-foreground",
      Self::Invalid => "text-destructive",
    }
  }
}

const VALIDATOR_BASE_CLASS: &str = "flex flex-col gap-1.5";
const VALIDATOR_HINT_BASE_CLASS: &str = "text-xs leading-snug";

/// Classes for `Validator`.
pub fn validator_class(state: ValidatorState, class: &str) -> String {
  merge_classes(classes([Some(VALIDATOR_BASE_CLASS), Some(state.container_class())]), class)
}

/// Classes for `ValidatorHint`.
pub fn validator_hint_class(state: ValidatorState, class: &str) -> String {
  merge_classes(classes([Some(VALIDATOR_HINT_BASE_CLASS), Some(state.hint_class())]), class)
}

/// Wraps a form control and highlights its border according to `ValidatorState`.
#[component]
pub fn Validator(
  #[props(default)] state: ValidatorState,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = validator_class(state, &class);

  rsx! {
    div {
      class,
      "data-state": state.attribute(),
      ..attributes,
      {children}
    }
  }
}

/// Validation hint or error message paired with a `Validator` field.
#[component]
pub fn ValidatorHint(
  #[props(default)] state: ValidatorState,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = validator_hint_class(state, &class);
  let role = (state == ValidatorState::Invalid).then_some("alert");

  rsx! {
    p {
      class,
      role,
      "data-state": state.attribute(),
      ..attributes,
      {children}
    }
  }
}
