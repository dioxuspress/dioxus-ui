//! Rating: lets the user pick a score from one to five stars, or to another
//! maximum.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

use crate::density::{density_hit_area_class, use_density};
use crate::element_id::next_element_id;

/// Star dimensions of a [`Rating`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RatingSize {
  /// Compact 16-pixel stars for inline cards and tables.
  Sm,
  /// Standard 24-pixel stars (default).
  #[default]
  Md,
  /// Prominent 32-pixel stars for review prompts.
  Lg,
}

impl RatingSize {
  /// The width and height class for each star.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "size-4",
      Self::Md => "size-6",
      Self::Lg => "size-8",
    }
  }
}

const RATING_BASE_CLASS: &str = "inline-flex items-center gap-1";
// The mask that draws the star would clip a focus outline, so the wrapper
// shows the ring for the focused radio.
const RATING_STAR_WRAPPER_CLASS: &str = "inline-flex rounded-sm has-focus-visible:ring-2 has-focus-visible:ring-ring has-focus-visible:ring-offset-2 has-focus-visible:ring-offset-background";
const RATING_STAR_CLASS: &str = "size-6 cursor-pointer appearance-none bg-muted-foreground/40 [mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2024%2024%27%3E%3Cpath%20d=%27M12%202l3.09%206.26L22%209.27l-5%204.87%201.18%206.88L12%2017.77l-6.18%203.25L7%2014.14%202%209.27l6.91-1.01L12%202z%27/%3E%3C/svg%3E)_center/contain_no-repeat] focus-visible:outline-none data-[filled=true]:bg-warning disabled:cursor-not-allowed disabled:opacity-50";
const RATING_STAR_CORE_CLASS: &str = "appearance-none bg-muted-foreground/40 [mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2024%2024%27%3E%3Cpath%20d=%27M12%202l3.09%206.26L22%209.27l-5%204.87%201.18%206.88L12%2017.77l-6.18%203.25L7%2014.14%202%209.27l6.91-1.01L12%202z%27/%3E%3C/svg%3E)_center/contain_no-repeat] focus-visible:outline-none data-[filled=true]:bg-warning";
const RATING_STAR_INTERACTIVE_CLASS: &str =
  "cursor-pointer disabled:cursor-not-allowed disabled:opacity-50";
const RATING_STAR_READONLY_CLASS: &str = "inline-block shrink-0";

/// Classes for the row of stars, with `class` merged over them.
pub fn rating_class(class: &str) -> String {
  merge_classes(classes([Some(RATING_BASE_CLASS)]), class)
}

/// Classes for an individual star with explicit `size` and `read_only` mode.
pub fn rating_star_class(size: RatingSize, read_only: bool, class: &str) -> String {
  let mode_class =
    if read_only { RATING_STAR_READONLY_CLASS } else { RATING_STAR_INTERACTIVE_CLASS };
  merge_classes(
    classes([Some(size.class()), Some(mode_class), Some(RATING_STAR_CORE_CLASS)]),
    class,
  )
}

fn rating_has_accessible_name(attributes: &[Attribute]) -> bool {
  attributes
    .iter()
    .any(|attribute| attribute.name == "aria-label" || attribute.name == "aria-labelledby")
}

/// A star rating from 0 (none) to `max`, as a radio group: the arrow keys
/// move between stars, and a change calls `on_value_change` with the chosen
/// star. Name the group with `aria-label`; each star is named "{n} of {max}".
/// Set `read_only: true` to display a static score at full contrast without
/// interactive radio controls.
#[component]
pub fn Rating(
  #[props(default)] value: u8,
  #[props(default = 5)] max: u8,
  #[props(default)] size: RatingSize,
  #[props(default)] read_only: bool,
  #[props(default)] name: Option<String>,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] on_value_change: Option<EventHandler<u8>>,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
  let class = rating_class(&class);
  let generated_name = use_hook(|| format!("dxui-rating-{}", next_element_id()));

  if read_only {
    let fallback_label =
      (!rating_has_accessible_name(&attributes)).then(|| format!("{value} of {max}"));
    let star_item_class = rating_star_class(size, true, "");

    return rsx! {
      div {
        class,
        role: "img",
        "aria-label": fallback_label,
        "data-readonly": "true",
        ..attributes,
        for star in 1..=max {
          span {
            key: "{star}",
            class: star_item_class.clone(),
            "aria-hidden": "true",
            "data-filled": star <= value,
          }
        }
      }
    };
  }

  // A label passes a press on the star's hit area to its radio (RFC 0078).
  let star_class = merge_classes(
    classes([Some(RATING_STAR_WRAPPER_CLASS)]),
    density_hit_area_class(use_density()),
  );
  let star_input_class = if size == RatingSize::Md {
    RATING_STAR_CLASS.to_string()
  } else {
    rating_star_class(size, false, "")
  };
  let name = name.unwrap_or(generated_name);

  rsx! {
    div { class, role: "radiogroup", ..attributes,
      for star in 1..=max {
        label { key: "{star}", class: star_class.clone(),
          input {
            class: star_input_class.clone(),
            r#type: "radio",
            name: name.clone(),
            value: "{star}",
            checked: star == value,
            disabled,
            "aria-label": "{star} of {max}",
            "data-filled": star <= value,
            onchange: move |_| {
              if let Some(handler) = on_value_change {
                handler.call(star);
              }
            },
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
  }

  #[test]
  fn ssr_rating_is_a_group_of_named_radios() {
    fn app() -> Element {
      rsx! { Rating { value: 3, name: "product", "aria-label": "Product rating" } }
    }
    let html = render(app);

    assert!(html.contains("role=\"radiogroup\""));
    assert!(html.contains("aria-label=\"Product rating\""));
    assert_eq!(html.matches("type=\"radio\"").count(), 5);
    assert_eq!(html.matches("name=\"product\"").count(), 5);
    assert!(html.contains("aria-label=\"3 of 5\""));
    assert_eq!(
      html.matches("data-filled=true").count() + html.matches("data-filled=\"true\"").count(),
      3
    );
    assert_eq!(html.matches("checked").count(), 1);
  }

  #[test]
  fn ssr_ratings_get_distinct_names() {
    fn app() -> Element {
      rsx! {
        Rating { max: 1 }
        Rating { max: 1 }
      }
    }
    let html = render(app);
    let names =
      html.split("name=\"").skip(1).filter_map(|rest| rest.split('"').next()).collect::<Vec<_>>();

    assert_eq!(names.len(), 2);
    assert_ne!(names[0], names[1]);
  }

  #[test]
  fn ssr_read_only_rating_renders_static_img_and_sizes() {
    fn app() -> Element {
      rsx! { Rating { value: 4, size: RatingSize::Sm, read_only: true } }
    }
    let html = render(app);

    assert!(html.contains("role=\"img\""), "{html}");
    assert!(html.contains("aria-label=\"4 of 5\""), "{html}");
    assert!(html.contains("data-readonly=\"true\""), "{html}");
    assert!(html.contains("size-4"), "{html}");
    assert!(!html.contains("type=\"radio\""), "{html}");
    assert!(!html.contains("opacity-50"), "{html}");
  }
}
