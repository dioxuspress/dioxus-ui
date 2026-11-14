//! Megamenu: a multi-column navigation panel triggered from a header or toolbar.

use super::safe_url::safe_href;
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

const MEGAMENU_BASE_CLASS: &str = "relative inline-block";
const MEGAMENU_TRIGGER_BASE_CLASS: &str =
  "inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md border border-border bg-background px-3 text-sm font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
const MEGAMENU_CONTENT_BASE_CLASS: &str =
  "absolute left-0 top-full z-40 mt-2 grid w-max max-w-[calc(100vw-2rem)] grid-cols-1 gap-6 rounded-lg border border-border bg-popover p-6 text-popover-foreground shadow-lg md:grid-cols-3";
const MEGAMENU_COLUMN_BASE_CLASS: &str = "flex min-w-36 flex-col gap-1.5";
const MEGAMENU_HEADING_BASE_CLASS: &str =
  "mb-1 text-xs font-semibold uppercase tracking-wider text-muted-foreground";
const MEGAMENU_LINK_BASE_CLASS: &str =
  "rounded-sm px-2 py-1 text-sm text-popover-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";

/// Classes for the `Megamenu` root wrapper.
pub fn megamenu_class(class: &str) -> String {
  merge_classes(classes([Some(MEGAMENU_BASE_CLASS)]), class)
}

/// Classes for `MegamenuTrigger`.
pub fn megamenu_trigger_class(class: &str) -> String {
  merge_classes(classes([Some(MEGAMENU_TRIGGER_BASE_CLASS)]), class)
}

/// Classes for `MegamenuContent`.
pub fn megamenu_content_class(class: &str) -> String {
  merge_classes(classes([Some(MEGAMENU_CONTENT_BASE_CLASS)]), class)
}

/// Classes for `MegamenuColumn`.
pub fn megamenu_column_class(class: &str) -> String {
  merge_classes(classes([Some(MEGAMENU_COLUMN_BASE_CLASS)]), class)
}

/// Classes for `MegamenuHeading`.
pub fn megamenu_heading_class(class: &str) -> String {
  merge_classes(classes([Some(MEGAMENU_HEADING_BASE_CLASS)]), class)
}

/// Classes for `MegamenuLink`.
pub fn megamenu_link_class(class: &str) -> String {
  merge_classes(classes([Some(MEGAMENU_LINK_BASE_CLASS)]), class)
}

#[derive(Clone, Copy)]
struct MegamenuContext {
  open: Signal<bool>,
}

/// Root container managing open/close state for a multi-column `Megamenu`.
#[component]
pub fn Megamenu(
  #[props(default)] default_open: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let mut open = use_signal(|| default_open);
  use_context_provider(|| MegamenuContext { open });
  let class = megamenu_class(&class);

  rsx! {
    div {
      class,
      "data-state": if open() { "open" } else { "closed" },
      onkeydown: move |event| {
        if event.key() == Key::Escape {
          open.set(false);
        }
      },
      ..attributes,
      {children}
    }
  }
}

/// Button that toggles `MegamenuContent`.
#[component]
pub fn MegamenuTrigger(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let mut ctx = use_context::<MegamenuContext>();
  let is_open = (ctx.open)();
  let class = megamenu_trigger_class(&class);

  rsx! {
    button {
      r#type: "button",
      class,
      "aria-expanded": if is_open { "true" } else { "false" },
      "aria-haspopup": "true",
      onclick: move |_| {
        let current = (ctx.open)();
        ctx.open.set(!current);
      },
      ..attributes,
      {children}
    }
  }
}

/// Multi-column dropdown panel rendered when the `Megamenu` is open.
#[component]
pub fn MegamenuContent(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let ctx = use_context::<MegamenuContext>();
  if !(ctx.open)() {
    return rsx! {};
  }
  let class = megamenu_content_class(&class);

  rsx! {
    div { class, ..attributes, {children} }
  }
}

/// A single vertical column inside `MegamenuContent`.
#[component]
pub fn MegamenuColumn(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = megamenu_column_class(&class);
  rsx! {
    div { class, ..attributes, {children} }
  }
}

/// Column heading inside `MegamenuColumn`.
#[component]
pub fn MegamenuHeading(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = megamenu_heading_class(&class);
  rsx! {
    div { class, ..attributes, {children} }
  }
}

/// Safe navigation link inside `MegamenuColumn`.
#[component]
pub fn MegamenuLink(
  href: String,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = megamenu_link_class(&class);
  let resolved_href = safe_href(href);
  rsx! {
    a { class, href: resolved_href, ..attributes, {children} }
  }
}
