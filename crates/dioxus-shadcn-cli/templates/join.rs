//! Join: a grouping container that collapses adjacent border radii for buttons, inputs, and selects.

use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

/// Direction in which `Join` arranges its children.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum JoinOrientation {
  /// Side by side in a row (default).
  #[default]
  Horizontal,
  /// Stacked in a column.
  Vertical,
}

impl JoinOrientation {
  /// Tailwind classes for the orientation's layout and edge radius collapsing.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Horizontal => {
        "flex-row [&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:-ml-px [&>*:not(:last-child)]:rounded-r-none"
      }
      Self::Vertical => {
        "flex-col [&>*:not(:first-child)]:rounded-t-none [&>*:not(:first-child)]:-mt-px [&>*:not(:last-child)]:rounded-b-none"
      }
    }
  }

  /// Value for the `data-orientation` attribute.
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

const JOIN_BASE_CLASS: &str = "inline-flex items-stretch [&>*]:focus-visible:z-10";
const JOIN_ITEM_BASE_CLASS: &str = "relative focus-visible:z-10";

/// Classes for the `Join` container.
pub fn join_class(orientation: JoinOrientation, class: &str) -> String {
  merge_classes(classes([Some(JOIN_BASE_CLASS), Some(orientation.class())]), class)
}

/// Helper classes for an explicit child item inside `Join`.
pub fn join_item_class(class: &str) -> String {
  merge_classes(classes([Some(JOIN_ITEM_BASE_CLASS)]), class)
}

/// Groups adjacent controls such as inputs, buttons, or badges into a seamless connected strip.
#[component]
pub fn Join(
  #[props(default)] orientation: JoinOrientation,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = join_class(orientation, &class);

  rsx! {
    div {
      class,
      "data-orientation": orientation.attribute(),
      ..attributes,
      {children}
    }
  }
}
