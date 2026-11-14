//! Filter: a radio-style filter bar with an automatic reset button when a filter is active.

use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// A single selectable filter option.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilterOption {
  /// Value emitted when this option is selected.
  pub value: String,
  /// Visible label rendered inside the filter pill.
  pub label: String,
}

impl FilterOption {
  /// Creates a new `FilterOption`.
  pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
    Self { value: value.into(), label: label.into() }
  }
}

const FILTER_BASE_CLASS: &str = "inline-flex flex-wrap items-center gap-1.5";
const FILTER_ITEM_BASE_CLASS: &str =
  "inline-flex h-8 cursor-pointer items-center justify-center rounded-md border px-3 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
const FILTER_ITEM_UNSELECTED_CLASS: &str =
  "border-border bg-background text-foreground hover:bg-accent hover:text-accent-foreground";
const FILTER_ITEM_SELECTED_CLASS: &str =
  "border-primary bg-primary text-primary-foreground shadow-xs";
const FILTER_RESET_BASE_CLASS: &str =
  "inline-flex h-8 w-8 cursor-pointer items-center justify-center rounded-md border border-border bg-background text-sm font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";

/// Classes for the `Filter` container.
pub fn filter_class(class: &str) -> String {
  merge_classes(classes([Some(FILTER_BASE_CLASS)]), class)
}

/// Classes for a `Filter` option pill based on its selection state.
pub fn filter_item_class(selected: bool, class: &str) -> String {
  let state_class = if selected { FILTER_ITEM_SELECTED_CLASS } else { FILTER_ITEM_UNSELECTED_CLASS };
  merge_classes(classes([Some(FILTER_ITEM_BASE_CLASS), Some(state_class)]), class)
}

/// Classes for the `Filter` reset button.
pub fn filter_reset_class(class: &str) -> String {
  merge_classes(classes([Some(FILTER_RESET_BASE_CLASS)]), class)
}

/// A filter pill bar with a reset button shown whenever a non-empty option is active.
#[component]
pub fn Filter(
  #[props(default)] options: Vec<FilterOption>,
  #[props(default)] value: Option<String>,
  #[props(default)] default_value: String,
  #[props(default)] on_change: Option<EventHandler<String>>,
  #[props(default = "×".to_string())] reset_label: String,
  #[props(default = "Clear filter".to_string())] reset_aria_label: String,
  #[props(default)] label: Option<String>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
  let mut internal = use_signal(|| default_value);
  let active_value = value.unwrap_or_else(|| internal.read().clone());
  let has_selection = !active_value.is_empty();
  let group_class = filter_class(&class);

  rsx! {
    div {
      class: group_class,
      role: "radiogroup",
      "aria-label": label,
      ..attributes,
      if has_selection {
        button {
          r#type: "button",
          class: filter_reset_class(""),
          "aria-label": "{reset_aria_label}",
          onclick: move |_| {
            internal.set(String::new());
            if let Some(handler) = &on_change {
              handler.call(String::new());
            }
          },
          "{reset_label}"
        }
      }
      for option in options {
        {
          let option_value = option.value.clone();
          let option_label = option.label.clone();
          let is_selected = active_value == option_value;
          let item_class = filter_item_class(is_selected, "");
          rsx! {
            button {
              key: "{option_value}",
              r#type: "button",
              role: "radio",
              class: item_class,
              "aria-checked": if is_selected { "true" } else { "false" },
              "data-state": if is_selected { "checked" } else { "unchecked" },
              onclick: move |_| {
                internal.set(option_value.clone());
                if let Some(handler) = &on_change {
                  handler.call(option_value.clone());
                }
              },
              "{option_label}"
            }
          }
        }
      }
    }
  }
}
