//! Native Select: a styled wrapper around the platform `select`, `optgroup`,
//! and `option` elements, for simple forms and native form submission.

use crate::density::{use_density, with_density};
use dioxus::prelude::*;
use dioxus_shadcn_core::{UiDensity, classes, merge_classes};

/// Height and text scale of a [`NativeSelect`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NativeSelectSize {
  /// Compact 32-pixel select.
  Sm,
  /// Standard 40-pixel select (default).
  #[default]
  Md,
  /// Spacious 48-pixel select.
  Lg,
}

impl NativeSelectSize {
  /// The size's height, padding, and text classes.
  pub const fn class(self) -> &'static str {
    match self {
      Self::Sm => "h-8 px-2.5 py-1 text-xs",
      Self::Md => "h-10 px-3 py-2 text-sm",
      Self::Lg => "h-12 px-4 py-2.5 text-base",
    }
  }
}

const NATIVE_SELECT_BASE_CLASS: &str = "h-10 w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
const NATIVE_SELECT_CORE_CLASS: &str = "w-full rounded-md border bg-background text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
const NATIVE_SELECT_GROUP_BASE_CLASS: &str = "text-sm font-medium text-foreground";
const NATIVE_SELECT_OPTION_BASE_CLASS: &str = "text-sm text-foreground";

/// Classes for the `select`: base classes, a destructive border and ring when
/// `invalid`, then `class` merged over them.
pub fn native_select_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(NATIVE_SELECT_BASE_CLASS), Some(invalid_class)]), class)
}

/// Classes for a [`NativeSelect`] with an explicit [`NativeSelectSize`] and `invalid` state.
pub fn native_select_sized_class(size: NativeSelectSize, invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(
    classes([Some(NATIVE_SELECT_CORE_CLASS), Some(size.class()), Some(invalid_class)]),
    class,
  )
}

/// Classes for an `optgroup`, with `class` merged over them.
pub fn native_select_group_class(class: &str) -> String {
  merge_classes(classes([Some(NATIVE_SELECT_GROUP_BASE_CLASS)]), class)
}

/// Classes for an `option`, with `class` merged over them.
pub fn native_select_option_class(class: &str) -> String {
  merge_classes(classes([Some(NATIVE_SELECT_OPTION_BASE_CLASS)]), class)
}

/// A change calls `on_value_change` with the chosen option's `value`; the app
/// marks that option `selected`. Other attributes, such as `id`, `name`, and
/// `required`, are passed to the select.
#[component]
pub fn NativeSelect(
  #[props(default)] size: NativeSelectSize,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(extends = GlobalAttributes, extends = select)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  // WebKit keeps a native select's height over a `min-height`.
  let touch_class = match use_density() {
    UiDensity::Touch => "h-11",
    UiDensity::Compact | UiDensity::Comfortable => "",
  };
  let merged_user = with_density(touch_class, &class);
  let class = if size == NativeSelectSize::Md {
    native_select_class(invalid, &merged_user)
  } else {
    native_select_sized_class(size, invalid, &merged_user)
  };

  rsx! {
    select {
      class,
      disabled,
      "aria-invalid": invalid.to_string(),
      onchange: move |event: FormEvent| {
        if let Some(handler) = on_value_change {
          handler.call(event.value());
        }
      },
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn NativeSelectGroup(
  #[props(default)] label: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = native_select_group_class(&class);

  rsx! {
    optgroup {
      class,
      label,
      {children}
    }
  }
}

#[component]
pub fn NativeSelectOption(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] selected: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = native_select_option_class(&class);

  rsx! {
    option {
      class,
      value,
      disabled,
      selected,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_select_class_reflects_invalid_state() {
    let actual = native_select_class(true, "w-48");

    assert_eq!(
      actual,
      "h-10 rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50 border-destructive focus-visible:ring-destructive w-48"
    );
    assert!(actual.contains("border-destructive focus-visible:ring-destructive"));
    assert!(actual.ends_with("w-48"));
  }

  #[test]
  fn native_select_sized_class_supports_sizes_and_invalid() {
    let sm = native_select_sized_class(NativeSelectSize::Sm, false, "");
    let lg = native_select_sized_class(NativeSelectSize::Lg, true, "w-56");

    assert!(sm.contains("h-8 px-2.5 py-1 text-xs") && !sm.contains("h-10"), "{sm}");
    assert!(sm.contains("border-input focus-visible:ring-ring"), "{sm}");
    assert!(lg.contains("h-12 px-4 py-2.5 text-base"), "{lg}");
    assert!(lg.contains("border-destructive") && lg.ends_with("w-56"), "{lg}");
  }

  #[test]
  fn ssr_native_select_applies_size_prop() {
    fn app() -> Element {
      rsx! {
        NativeSelect { size: NativeSelectSize::Sm,
          NativeSelectOption { value: "a", "Alpha" }
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains("h-8 px-2.5 py-1 text-xs"), "{html}");
    assert!(!html.contains("h-10"), "{html}");
  }

  #[test]
  fn native_select_option_class_extends_user_class() {
    let actual = native_select_option_class("font-medium");

    assert!(actual.contains(NATIVE_SELECT_OPTION_BASE_CLASS));
    assert!(actual.ends_with("font-medium"));
  }
}
