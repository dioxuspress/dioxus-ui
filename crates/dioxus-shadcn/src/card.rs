//! Card: a container that groups related content, with optional header, title,
//! description, content, footer, media, body, and action sections.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

const CARD_BASE_CLASS: &str =
  "rounded-md border border-border bg-card text-card-foreground shadow-sm";
const CARD_HEADER_BASE_CLASS: &str = "flex flex-col gap-1.5";
const CARD_TITLE_BASE_CLASS: &str = "font-semibold leading-none tracking-normal";
const CARD_DESCRIPTION_BASE_CLASS: &str = "text-sm";
const CARD_CONTENT_BASE_CLASS: &str = "pt-0";
const CARD_FOOTER_BASE_CLASS: &str = "flex items-center pt-0";
const CARD_BODY_BASE_CLASS: &str = "flex flex-1 flex-col gap-2";
const CARD_ACTIONS_BASE_CLASS: &str = "mt-2 flex flex-wrap items-center gap-2";

/// Visual surface and border style of a [`Card`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
  /// Standard elevated card with a border and subtle shadow.
  #[default]
  Default,
  /// Outlined card without elevation shadow (`card-border`).
  Border,
  /// Dashed-border card without elevation shadow (`card-dash`).
  Dash,
  /// High-emphasis primary surface card.
  Primary,
  /// Subtle muted surface card.
  Muted,
}

/// Padding and typography scale of a [`Card`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardSize {
  /// Extra-compact padding (`card-xs`).
  Xs,
  /// Compact padding (`card-sm`).
  Sm,
  /// Standard padding (`card-md`).
  #[default]
  Md,
  /// Spacious padding (`card-lg`).
  Lg,
  /// Extra-spacious padding (`card-xl`).
  Xl,
}

/// Arrangement of media and body inside a [`Card`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardLayout {
  /// Stacked vertical layout (default).
  #[default]
  Vertical,
  /// Horizontal side-by-side layout with media on the leading edge (`card-side`).
  Side,
  /// Full-bleed background media with foreground content overlaid (`image-full`).
  ImageFull,
}

/// Horizontal alignment of items inside [`CardActions`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardActionsJustify {
  /// Align actions to the start edge.
  Start,
  /// Align actions to the center.
  Center,
  /// Align actions to the end edge (default).
  #[default]
  End,
  /// Spread actions across the row.
  Between,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct CardContext {
  variant: CardVariant,
  size: CardSize,
  layout: CardLayout,
}

fn use_card_context() -> CardContext {
  try_use_context::<CardContext>().unwrap_or_default()
}

fn card_variant_tokens(variant: CardVariant) -> &'static str {
  match variant {
    CardVariant::Default => CARD_BASE_CLASS,
    CardVariant::Border => {
      "rounded-md border border-border bg-card text-card-foreground shadow-none"
    }
    CardVariant::Dash => {
      "rounded-md border border-dashed border-border bg-card text-card-foreground shadow-none"
    }
    CardVariant::Primary => {
      "rounded-md border border-primary/20 bg-primary text-primary-foreground shadow-sm"
    }
    CardVariant::Muted => {
      "rounded-md border border-border/60 bg-muted/50 text-foreground shadow-none"
    }
  }
}

fn card_layout_tokens(layout: CardLayout) -> Option<&'static str> {
  match layout {
    CardLayout::Vertical => None,
    CardLayout::Side => Some("flex flex-col overflow-hidden sm:flex-row"),
    CardLayout::ImageFull => Some("relative grid overflow-hidden"),
  }
}

fn card_padding_token(size: CardSize) -> &'static str {
  match size {
    CardSize::Xs => "p-3",
    CardSize::Sm => "p-4",
    CardSize::Md => "p-6",
    CardSize::Lg => "p-8",
    CardSize::Xl => "p-10",
  }
}

fn card_inset_media_token(size: CardSize) -> &'static str {
  match size {
    CardSize::Xs => "px-3 pt-3",
    CardSize::Sm => "px-4 pt-4",
    CardSize::Md => "px-6 pt-6",
    CardSize::Lg => "px-8 pt-8",
    CardSize::Xl => "px-10 pt-10",
  }
}

fn card_title_size_token(size: CardSize) -> &'static str {
  match size {
    CardSize::Xs => "text-base",
    CardSize::Sm => "text-lg",
    CardSize::Md | CardSize::Lg => "text-2xl",
    CardSize::Xl => "text-3xl",
  }
}

/// Classes for the card's bordered surface, with `class` merged over them.
pub fn card_class(class: &str) -> String {
  merge_classes(classes([Some(CARD_BASE_CLASS)]), class)
}

fn card_surface_tokens(variant: CardVariant, layout: CardLayout) -> &'static str {
  if layout == CardLayout::ImageFull {
    "rounded-md border border-border bg-foreground text-background shadow-sm"
  } else {
    card_variant_tokens(variant)
  }
}

/// Classes for a [`Card`] with explicit `variant`, `size`, and `layout`.
pub fn card_variant_class(
  variant: CardVariant,
  _size: CardSize,
  layout: CardLayout,
  class: &str,
) -> String {
  merge_classes(
    classes([Some(card_surface_tokens(variant, layout)), card_layout_tokens(layout)]),
    class,
  )
}

fn card_header_class(ctx: CardContext, class: &str) -> String {
  let z = if ctx.layout == CardLayout::ImageFull { Some("relative z-10") } else { None };
  merge_classes(
    classes([Some(CARD_HEADER_BASE_CLASS), Some(card_padding_token(ctx.size)), z]),
    class,
  )
}

fn card_title_class(ctx: CardContext, class: &str) -> String {
  merge_classes(
    classes([Some(card_title_size_token(ctx.size)), Some(CARD_TITLE_BASE_CLASS)]),
    class,
  )
}

fn card_description_tokens(ctx: CardContext) -> &'static str {
  if ctx.variant == CardVariant::Primary || ctx.layout == CardLayout::ImageFull {
    "text-current/80"
  } else {
    "text-muted-foreground"
  }
}

fn card_description_class(ctx: CardContext, class: &str) -> String {
  merge_classes(
    classes([Some(CARD_DESCRIPTION_BASE_CLASS), Some(card_description_tokens(ctx))]),
    class,
  )
}

fn card_content_class(ctx: CardContext, class: &str) -> String {
  let z = if ctx.layout == CardLayout::ImageFull { Some("relative z-10") } else { None };
  merge_classes(
    classes([Some(card_padding_token(ctx.size)), Some(CARD_CONTENT_BASE_CLASS), z]),
    class,
  )
}

fn card_footer_class(ctx: CardContext, class: &str) -> String {
  let z = if ctx.layout == CardLayout::ImageFull { Some("relative z-10") } else { None };
  merge_classes(
    classes([Some(CARD_FOOTER_BASE_CLASS), Some(card_padding_token(ctx.size)), z]),
    class,
  )
}

fn card_body_class(ctx: CardContext, class: &str) -> String {
  let z = if ctx.layout == CardLayout::ImageFull { Some("relative z-10") } else { None };
  merge_classes(classes([Some(CARD_BODY_BASE_CLASS), Some(card_padding_token(ctx.size)), z]), class)
}

fn card_media_class(ctx: CardContext, inset: bool, class: &str) -> String {
  let layout_cls = match ctx.layout {
    CardLayout::Vertical => {
      if inset {
        card_inset_media_token(ctx.size)
      } else {
        "overflow-hidden rounded-t-md"
      }
    }
    CardLayout::Side => "shrink-0 overflow-hidden sm:w-48",
    CardLayout::ImageFull => "absolute inset-0 z-0 overflow-hidden",
  };
  merge_classes(classes([Some(layout_cls)]), class)
}

fn card_actions_class(justify: CardActionsJustify, class: &str) -> String {
  let justify_cls = match justify {
    CardActionsJustify::Start => "justify-start",
    CardActionsJustify::Center => "justify-center",
    CardActionsJustify::End => "justify-end",
    CardActionsJustify::Between => "justify-between",
  };
  merge_classes(classes([Some(CARD_ACTIONS_BASE_CLASS), Some(justify_cls)]), class)
}

/// Basic bordered card container (v0.6-compatible props). For variant, size,
/// and layout props, use [`CardSurface`].
#[component]
pub fn Card(#[props(default)] class: String, children: Element) -> Element {
  let class = card_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// Surface container that groups related content, supporting border/color variants,
/// padding sizes, and vertical, side-by-side, or full-bleed image layouts.
#[component]
pub fn CardSurface(
  #[props(default)] variant: CardVariant,
  #[props(default)] size: CardSize,
  #[props(default)] layout: CardLayout,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  use_context_provider(|| CardContext { variant, size, layout });
  let class = if variant == CardVariant::Default && layout == CardLayout::Vertical {
    card_class(&class)
  } else {
    card_variant_class(variant, size, layout, &class)
  };

  rsx! {
    div {
      class,
      "data-variant": match variant {
        CardVariant::Default => "default",
        CardVariant::Border => "border",
        CardVariant::Dash => "dash",
        CardVariant::Primary => "primary",
        CardVariant::Muted => "muted",
      },
      "data-size": match size {
        CardSize::Xs => "xs",
        CardSize::Sm => "sm",
        CardSize::Md => "md",
        CardSize::Lg => "lg",
        CardSize::Xl => "xl",
      },
      "data-layout": match layout {
        CardLayout::Vertical => "vertical",
        CardLayout::Side => "side",
        CardLayout::ImageFull => "image-full",
      },
      {children}
    }
  }
}

/// Optional top section of a [`Card`] typically containing title and description.
#[component]
pub fn CardHeader(#[props(default)] class: String, children: Element) -> Element {
  let ctx = use_card_context();
  let class = card_header_class(ctx, &class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// A `div`, as in shadcn/ui v4; wrap the text in a heading at the level the
/// page outline needs.
#[component]
pub fn CardTitle(#[props(default)] class: String, children: Element) -> Element {
  let ctx = use_card_context();
  let class = card_title_class(ctx, &class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// Supporting description text for a [`Card`].
#[component]
pub fn CardDescription(#[props(default)] class: String, children: Element) -> Element {
  let ctx = use_card_context();
  let class = card_description_class(ctx, &class);

  rsx! {
    p {
      class,
      {children}
    }
  }
}

/// Main content section of a [`Card`].
#[component]
pub fn CardContent(#[props(default)] class: String, children: Element) -> Element {
  let ctx = use_card_context();
  let class = card_content_class(ctx, &class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// Bottom footer row of a [`Card`].
#[component]
pub fn CardFooter(#[props(default)] class: String, children: Element) -> Element {
  let ctx = use_card_context();
  let class = card_footer_class(ctx, &class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// Media (`<figure>`) slot of a [`Card`] that adapts to vertical, side-by-side,
/// or full-bleed background layouts.
#[component]
pub fn CardMedia(
  #[props(default)] inset: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let ctx = use_card_context();
  let class = card_media_class(ctx, inset, &class);

  rsx! {
    figure {
      class,
      {children}
      if ctx.layout == CardLayout::ImageFull {
        div { class: "pointer-events-none absolute inset-0 bg-foreground/65" }
      }
    }
  }
}

/// Unified body container (`.card-body` equivalent) for title, copy, and [`CardActions`].
#[component]
pub fn CardBody(#[props(default)] class: String, children: Element) -> Element {
  let ctx = use_card_context();
  let class = card_body_class(ctx, &class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// Action bar (`.card-actions` equivalent) inside [`CardBody`] or [`Card`].
#[component]
pub fn CardActions(
  #[props(default)] justify: CardActionsJustify,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = card_actions_class(justify, &class);

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

  #[test]
  fn ssr_title_renders_a_div() {
    fn app() -> Element {
      rsx! { CardTitle { "Create project" } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.starts_with("<div"));
    assert!(!html.contains("<h3"));
  }

  #[test]
  fn card_class_appends_user_class() {
    let actual = card_class("max-w-sm");

    assert!(actual.contains(CARD_BASE_CLASS));
    assert!(actual.ends_with("max-w-sm"));
  }

  #[test]
  fn card_footer_class_appends_user_class() {
    let actual = card_footer_class(CardContext::default(), "justify-end");

    assert!(actual.contains("p-6"));
    assert!(actual.contains(CARD_FOOTER_BASE_CLASS));
    assert!(actual.ends_with("justify-end"));
  }

  #[test]
  fn card_variant_class_supports_border_dash_and_layouts() {
    let dash = card_variant_class(CardVariant::Dash, CardSize::Md, CardLayout::Vertical, "");
    assert!(dash.contains("border-dashed"));

    let side = card_variant_class(CardVariant::Border, CardSize::Sm, CardLayout::Side, "max-w-lg");
    assert!(side.contains("sm:flex-row"));
    assert!(side.contains("shadow-none"));
    assert!(side.ends_with("max-w-lg"));
  }

  #[test]
  fn ssr_card_body_and_media_respect_context_size_and_layout() {
    fn app() -> Element {
      rsx! {
        CardSurface { variant: CardVariant::Border, size: CardSize::Sm, layout: CardLayout::ImageFull,
          CardMedia { div { "bg" } }
          CardBody {
            CardTitle { "Shoes!" }
            CardActions { justify: CardActionsJustify::Between, "Action" }
          }
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains("data-variant=\"border\""));
    assert!(html.contains("data-size=\"sm\""));
    assert!(html.contains("data-layout=\"image-full\""));
    assert!(html.contains("<figure"));
    assert!(html.contains("bg-foreground/65"));
    assert!(html.contains("p-4"));
    assert!(html.contains("justify-between"));
  }
}
