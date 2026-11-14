//! Footer: a semantic page footer with multi-column navigation and centered layout modes.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// Layout arrangement for `Footer`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FooterLayout {
  /// Multi-column responsive footer (default).
  #[default]
  Columns,
  /// Centered single-column footer.
  Centered,
}

impl FooterLayout {
  /// Tailwind layout classes for this `FooterLayout`.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Columns => "grid-flow-row place-items-start md:grid-flow-col",
      Self::Centered => "grid-flow-row place-items-center text-center",
    }
  }
}

const FOOTER_BASE_CLASS: &str =
  "grid w-full gap-8 border-t border-border bg-muted/30 p-8 text-sm text-foreground";
const FOOTER_NAV_BASE_CLASS: &str = "flex flex-col gap-2";
const FOOTER_TITLE_BASE_CLASS: &str =
  "mb-1 text-xs font-semibold uppercase tracking-wider text-muted-foreground";

/// Classes for `Footer`.
pub fn footer_class(layout: FooterLayout, class: &str) -> String {
  merge_classes(classes([Some(FOOTER_BASE_CLASS), Some(layout.class())]), class)
}

/// Classes for `FooterNav`.
pub fn footer_nav_class(class: &str) -> String {
  merge_classes(classes([Some(FOOTER_NAV_BASE_CLASS)]), class)
}

/// Classes for `FooterTitle`.
pub fn footer_title_class(class: &str) -> String {
  merge_classes(classes([Some(FOOTER_TITLE_BASE_CLASS)]), class)
}

/// Semantic footer container.
#[component]
pub fn Footer(
  #[props(default)] layout: FooterLayout,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = footer_class(layout, &class);
  rsx! {
    footer { class, ..attributes, {children} }
  }
}

/// A column of links or metadata inside `Footer`.
#[component]
pub fn FooterNav(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = footer_nav_class(&class);
  rsx! {
    div { class, ..attributes, {children} }
  }
}

/// Section heading inside `FooterNav`.
#[component]
pub fn FooterTitle(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = footer_title_class(&class);
  rsx! {
    span { class, ..attributes, {children} }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dioxus_ssr::render_element;

  #[test]
  fn footer_renders_columns_and_centered_layouts() {
    let cols = footer_class(FooterLayout::Columns, "");
    let centered = footer_class(FooterLayout::Centered, "py-6");
    assert!(cols.contains("md:grid-flow-col"));
    assert!(centered.contains("place-items-center"));
    assert!(centered.ends_with("py-6"));

    let html = render_element(rsx! {
      Footer {
        FooterNav {
          FooterTitle { "Product" }
          a { href: "/docs", "Docs" }
        }
      }
    });
    assert!(html.contains("<footer"));
    assert!(html.contains("Product"));
  }
}
