//! Avatar: a user's or entity's round image, with a fallback for initials when
//! there is no image.
use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

/// Diameter of an [`Avatar`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AvatarSize {
  /// Extra small: 24px square.
  Xs,
  /// Small: 32px square.
  Sm,
  /// Medium: 40px square (default).
  #[default]
  Md,
  /// Large: 48px square.
  Lg,
  /// Extra large: 64px square.
  Xl,
}

impl AvatarSize {
  /// The width and height classes for this size.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Xs => "h-6 w-6",
      Self::Sm => "h-8 w-8",
      Self::Md => "h-10 w-10",
      Self::Lg => "h-12 w-12",
      Self::Xl => "h-16 w-16",
    }
  }

  /// Matching text size for [`AvatarFallback`] initials.
  pub const fn fallback_text_class(self) -> &'static str {
    match self {
      Self::Xs => "text-[10px]",
      Self::Sm => "text-xs",
      Self::Md => "text-sm",
      Self::Lg => "text-base",
      Self::Xl => "text-lg",
    }
  }
}

/// Corner radius of an [`Avatar`] and its [`AvatarFallback`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AvatarShape {
  /// Circular avatar frame (default).
  #[default]
  Circle,
  /// Rounded-square avatar frame.
  Square,
}

impl AvatarShape {
  /// The border-radius class for this shape.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Circle => "rounded-full",
      Self::Square => "rounded-md",
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct AvatarContext {
  size: AvatarSize,
  shape: AvatarShape,
}

const AVATAR_BASE_CLASS: &str = "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full";
const AVATAR_CORE_CLASS: &str = "relative flex shrink-0 overflow-hidden";
const AVATAR_IMAGE_BASE_CLASS: &str = "aspect-square h-full w-full object-cover";
const AVATAR_FALLBACK_BASE_CLASS: &str = "flex h-full w-full items-center justify-center rounded-full bg-muted text-sm font-medium text-foreground";
const AVATAR_FALLBACK_CORE_CLASS: &str =
  "flex h-full w-full items-center justify-center bg-muted font-medium text-foreground";
const AVATAR_GROUP_BASE_CLASS: &str =
  "flex items-center -space-x-2 [&>*]:ring-2 [&>*]:ring-background";

/// Classes for the round, clipped frame, with `class` merged over them.
pub fn avatar_class(class: &str) -> String {
  merge_classes(classes([Some(AVATAR_BASE_CLASS)]), class)
}

/// Classes for an [`Avatar`] with explicit `size` and `shape`.
pub fn avatar_styled_class(size: AvatarSize, shape: AvatarShape, class: &str) -> String {
  merge_classes(classes([Some(AVATAR_CORE_CLASS), Some(size.class()), Some(shape.class())]), class)
}

/// Classes for the image, which fills and covers the frame, with `class` merged over them.
pub fn avatar_image_class(class: &str) -> String {
  merge_classes(classes([Some(AVATAR_IMAGE_BASE_CLASS)]), class)
}

/// Classes for the fallback: centered text on a muted fill, with `class` merged over them.
pub fn avatar_fallback_class(class: &str) -> String {
  merge_classes(classes([Some(AVATAR_FALLBACK_BASE_CLASS)]), class)
}

/// Classes for an [`AvatarGroup`] stack container.
pub fn avatar_group_class(class: &str) -> String {
  merge_classes(classes([Some(AVATAR_GROUP_BASE_CLASS)]), class)
}

/// Frame for a user or entity avatar image and fallback initials.
#[component]
pub fn Avatar(
  #[props(default)] size: AvatarSize,
  #[props(default)] shape: AvatarShape,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  use_context_provider(|| AvatarContext { size, shape });
  let class = if size == AvatarSize::Md && shape == AvatarShape::Circle {
    avatar_class(&class)
  } else {
    avatar_styled_class(size, shape, &class)
  };

  rsx! {
    span {
      class,
      {children}
    }
  }
}

/// Image element inside an [`Avatar`].
#[component]
pub fn AvatarImage(
  src: String,
  #[props(default)] alt: String,
  #[props(default)] class: String,
) -> Element {
  let class = avatar_image_class(&class);

  rsx! {
    img {
      class,
      src,
      alt,
    }
  }
}

/// Fallback initials or icon displayed inside an [`Avatar`].
#[component]
pub fn AvatarFallback(#[props(default)] class: String, children: Element) -> Element {
  let ctx = try_use_context::<AvatarContext>().unwrap_or_default();
  let class = if ctx.size == AvatarSize::Md && ctx.shape == AvatarShape::Circle {
    avatar_fallback_class(&class)
  } else {
    merge_classes(
      classes([
        Some(AVATAR_FALLBACK_CORE_CLASS),
        Some(ctx.shape.class()),
        Some(ctx.size.fallback_text_class()),
      ]),
      &class,
    )
  };

  rsx! {
    span {
      class,
      {children}
    }
  }
}

/// Horizontally stacks [`Avatar`] children with overlap and a background ring.
#[component]
pub fn AvatarGroup(#[props(default)] class: String, children: Element) -> Element {
  let class = avatar_group_class(&class);

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
  fn avatar_class_appends_user_class() {
    let actual = avatar_class("h-12 w-12");

    assert_eq!(actual, "relative flex shrink-0 overflow-hidden rounded-full h-12 w-12");
    assert!(actual.ends_with("h-12 w-12"));
  }

  #[test]
  fn avatar_fallback_class_appends_user_class() {
    let actual = avatar_fallback_class("bg-blue-100");

    assert_eq!(
      actual,
      "flex h-full w-full items-center justify-center rounded-full text-sm font-medium text-foreground bg-blue-100"
    );
    assert!(actual.ends_with("bg-blue-100"));
  }

  #[test]
  fn avatar_styled_and_group_render_expected_classes() {
    fn app() -> Element {
      rsx! {
        AvatarGroup {
          Avatar { size: AvatarSize::Sm, shape: AvatarShape::Square,
            AvatarFallback { "DX" }
          }
          Avatar { size: AvatarSize::Xl,
            AvatarFallback { "UI" }
          }
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains("-space-x-2"), "{html}");
    assert!(html.contains("h-8 w-8 rounded-md"), "{html}");
    assert!(html.contains("text-xs"), "{html}");
    assert!(html.contains("h-16 w-16 rounded-full"), "{html}");
    assert!(html.contains("text-lg"), "{html}");
  }
}
