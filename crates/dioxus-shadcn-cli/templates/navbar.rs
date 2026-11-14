//! Navbar: top-level application navigation bar with start, center, and end slots.

use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

const NAVBAR_BASE_CLASS: &str =
  "flex min-h-16 w-full items-center justify-between gap-4 border-b border-border bg-background px-4 text-foreground";
const NAVBAR_START_CLASS: &str = "flex flex-1 items-center justify-start gap-2";
const NAVBAR_CENTER_CLASS: &str = "flex shrink-0 items-center justify-center gap-2";
const NAVBAR_END_CLASS: &str = "flex flex-1 items-center justify-end gap-2";

/// Classes for the `Navbar` container.
pub fn navbar_class(class: &str) -> String {
  merge_classes(classes([Some(NAVBAR_BASE_CLASS)]), class)
}

/// Classes for `NavbarStart`.
pub fn navbar_start_class(class: &str) -> String {
  merge_classes(classes([Some(NAVBAR_START_CLASS)]), class)
}

/// Classes for `NavbarCenter`.
pub fn navbar_center_class(class: &str) -> String {
  merge_classes(classes([Some(NAVBAR_CENTER_CLASS)]), class)
}

/// Classes for `NavbarEnd`.
pub fn navbar_end_class(class: &str) -> String {
  merge_classes(classes([Some(NAVBAR_END_CLASS)]), class)
}

/// Top navigation bar container.
#[component]
pub fn Navbar(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = navbar_class(&class);
  rsx! {
    header { class, ..attributes, {children} }
  }
}

/// Leading slot inside `Navbar` (brand, menu trigger, primary links).
#[component]
pub fn NavbarStart(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = navbar_start_class(&class);
  rsx! {
    div { class, ..attributes, {children} }
  }
}

/// Centered slot inside `Navbar`.
#[component]
pub fn NavbarCenter(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = navbar_center_class(&class);
  rsx! {
    div { class, ..attributes, {children} }
  }
}

/// Trailing slot inside `Navbar` (search, theme switcher, account actions).
#[component]
pub fn NavbarEnd(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = navbar_end_class(&class);
  rsx! {
    div { class, ..attributes, {children} }
  }
}
