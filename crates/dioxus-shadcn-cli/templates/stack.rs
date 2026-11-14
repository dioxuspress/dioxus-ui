//! Stack: visually stacks child elements on top of one another with subtle depth offsets.

use dioxus::prelude::*;
use super::utils::{classes, merge_classes};

/// Direction in which stacked background items peek out behind the top item.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StackAlign {
  /// Stacked items peek below the top card (default).
  #[default]
  Bottom,
  /// Stacked items peek above the top card.
  Top,
  /// Stacked items peek toward the inline-start side.
  Start,
  /// Stacked items peek toward the inline-end side.
  End,
}

impl StackAlign {
  /// Child transform classes for the stacked layers behind the first child.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Bottom => {
        "[&>*:nth-child(2)]:translate-y-2 [&>*:nth-child(2)]:scale-95 [&>*:nth-child(3)]:translate-y-4 [&>*:nth-child(3)]:scale-90"
      }
      Self::Top => {
        "[&>*:nth-child(2)]:-translate-y-2 [&>*:nth-child(2)]:scale-95 [&>*:nth-child(3)]:-translate-y-4 [&>*:nth-child(3)]:scale-90"
      }
      Self::Start => {
        "[&>*:nth-child(2)]:-translate-x-2 [&>*:nth-child(2)]:scale-95 [&>*:nth-child(3)]:-translate-x-4 [&>*:nth-child(3)]:scale-90"
      }
      Self::End => {
        "[&>*:nth-child(2)]:translate-x-2 [&>*:nth-child(2)]:scale-95 [&>*:nth-child(3)]:translate-x-4 [&>*:nth-child(3)]:scale-90"
      }
    }
  }
}

const STACK_BASE_CLASS: &str = "inline-grid place-items-stretch [&>*]:col-start-1 [&>*]:row-start-1 [&>*]:transition-transform [&>*]:duration-200 [&>*:nth-child(1)]:z-3 [&>*:nth-child(2)]:z-2 [&>*:nth-child(3)]:z-1 [&>*:nth-child(n+4)]:z-0 [&>*:nth-child(2)]:opacity-90 [&>*:nth-child(3)]:opacity-70 [&>*:nth-child(n+4)]:opacity-0";

/// Classes for `Stack`, combining the stacked grid rules with `StackAlign`.
pub fn stack_class(align: StackAlign, class: &str) -> String {
  merge_classes(classes([Some(STACK_BASE_CLASS), Some(align.class())]), class)
}

/// Stacks child cards or elements on top of each other with subtle depth offsets.
#[component]
pub fn Stack(
  #[props(default)] align: StackAlign,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = stack_class(align, &class);

  rsx! {
    div { class, ..attributes, {children} }
  }
}
