//! Hover3D: interactive 3D perspective tilt card that responds to pointer zones without JS eval.

use dioxus::prelude::*;
use super::utils::{classes, merge_classes};

/// Tilt intensity preset for `Hover3D`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Hover3dIntensity {
  /// Gentle 5-degree tilt.
  Subtle,
  /// Balanced 10-degree tilt (default).
  #[default]
  Default,
  /// Pronounced 15-degree tilt.
  Strong,
}

impl Hover3dIntensity {
  /// Maximum tilt angle in degrees for this intensity.
  pub const fn max_degrees(self) -> i8 {
    match self {
      Self::Subtle => 5,
      Self::Default => 10,
      Self::Strong => 15,
    }
  }
}

const HOVER_3D_BASE_CLASS: &str = "group relative inline-block select-none";
const HOVER_3D_SURFACE_BASE_CLASS: &str =
  "relative overflow-hidden rounded-xl transition-transform duration-200 ease-out will-change-transform";

/// Classes for the outer `Hover3D` wrapper.
pub fn hover_3d_class(class: &str) -> String {
  merge_classes(classes([Some(HOVER_3D_BASE_CLASS)]), class)
}

/// Classes for the tilted inner `Hover3D` surface.
pub fn hover_3d_surface_class(class: &str) -> String {
  merge_classes(classes([Some(HOVER_3D_SURFACE_BASE_CLASS)]), class)
}

/// Computes the CSS `transform` value for a `(rx_factor, ry_factor)` grid cell in `[-1, 0, 1]`.
pub fn hover_3d_transform(
  rx_factor: i8,
  ry_factor: i8,
  intensity: Hover3dIntensity,
  perspective_px: u32,
) -> String {
  if rx_factor == 0 && ry_factor == 0 {
    return format!("perspective({perspective_px}px) rotateX(0deg) rotateY(0deg) scale3d(1, 1, 1)");
  }
  let deg = intensity.max_degrees();
  let rx = i16::from(rx_factor) * i16::from(deg);
  let ry = i16::from(ry_factor) * i16::from(deg);
  format!("perspective({perspective_px}px) rotateX({rx}deg) rotateY({ry}deg) scale3d(1.02, 1.02, 1.02)")
}

/// Wraps card or showcase content in a 3D perspective container that tilts toward the hovered zone.
#[component]
pub fn Hover3D(
  #[props(default)] intensity: Hover3dIntensity,
  #[props(default = 900)] perspective: u32,
  #[props(default = true)] shine: bool,
  #[props(default)] class: String,
  #[props(default)] surface_class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let mut tilt = use_signal(|| (0i8, 0i8));
  let (rx_factor, ry_factor) = tilt();
  let wrapper_class = hover_3d_class(&class);
  let inner_class = hover_3d_surface_class(&surface_class);
  let transform = hover_3d_transform(rx_factor, ry_factor, intensity, perspective);
  let active = rx_factor != 0 || ry_factor != 0;

  let zones: [(i8, i8); 9] = [
    (1, -1),
    (1, 0),
    (1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
  ];

  rsx! {
    div {
      class: wrapper_class,
      "data-hover-3d-active": active.to_string(),
      onmouseleave: move |_| tilt.set((0, 0)),
      ..attributes,
      div {
        class: inner_class,
        style: "transform: {transform};",
        {children}
        if shine {
          div {
            class: if active {
              "pointer-events-none absolute inset-0 bg-linear-to-tr from-transparent via-background/10 to-background/25 opacity-100 transition-opacity duration-200"
            } else {
              "pointer-events-none absolute inset-0 opacity-0 transition-opacity duration-200"
            },
            "aria-hidden": "true",
          }
        }
      }
      div {
        class: "absolute inset-0 z-10 grid grid-cols-3 grid-rows-3",
        "aria-hidden": "true",
        for (idx, (rx, ry)) in zones.into_iter().enumerate() {
          span {
            key: "zone-{idx}",
            onmouseenter: move |_| tilt.set((rx, ry)),
          }
        }
      }
    }
  }
}
