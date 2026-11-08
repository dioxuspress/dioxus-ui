//! Code Tabs: an underlined tab bar that switches between code panels for
//! side-by-side language, tool, or command comparisons.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

use crate::density::{density_control_class, use_density, with_density};
use crate::element_id::next_element_id;
use crate::root_state::{Controllable, use_controllable, use_root_context};
use crate::roving_group::{group_part_id, use_roving_group};

const CODE_TABS_BASE_CLASS: &str = "grid gap-2";
const CODE_TABS_LIST_BASE_CLASS: &str =
  "flex items-center gap-1 overflow-x-auto border-b border-border";
const CODE_TABS_TRIGGER_BASE_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap border-b-2 px-3 py-1.5 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const CODE_TABS_CONTENT_BASE_CLASS: &str = "mt-2 overflow-x-auto rounded-lg bg-muted p-4 font-mono text-sm text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";

/// Whether moving focus to a code tab trigger also selects it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeTabsActivation {
  /// Arrow, Home, and End keys select the trigger they move to.
  #[default]
  Automatic,
  /// Keys only move focus; Enter, Space, or a click selects.
  Manual,
}

impl CodeTabsActivation {
  /// The roving group script's `data-dxui-roving-activation` value.
  pub fn roving_attribute(self) -> &'static str {
    match self {
      Self::Automatic => "focus",
      Self::Manual => "manual",
    }
  }
}

/// Classes for the root container that stacks the tab bar above the active code panel.
pub fn code_tabs_class(class: &str) -> String {
  merge_classes(classes([Some(CODE_TABS_BASE_CLASS)]), class)
}

/// Classes for the underlined list that holds the code tab triggers.
pub fn code_tabs_list_class(class: &str) -> String {
  merge_classes(classes([Some(CODE_TABS_LIST_BASE_CLASS)]), class)
}

/// Classes for a code tab trigger: base classes, primary underline and text while
/// `active` and muted otherwise, with `class` merged over them.
pub fn code_tabs_trigger_class(active: bool, class: &str) -> String {
  let active_class = if active {
    "border-primary text-primary"
  } else {
    "border-transparent text-muted-foreground hover:text-foreground"
  };

  merge_classes(classes([Some(CODE_TABS_TRIGGER_BASE_CLASS), Some(active_class)]), class)
}

/// Classes for a code tab content panel.
pub fn code_tabs_content_class(class: &str) -> String {
  merge_classes(classes([Some(CODE_TABS_CONTENT_BASE_CLASS)]), class)
}

/// What `CodeTabs` shares with its parts (RFC 0077).
#[derive(Clone)]
struct CodeTabsContext {
  base_id: String,
  value: Controllable<Option<String>>,
  select: Callback<String>,
  activation: CodeTabsActivation,
}

/// The root of a code comparison tab set: it owns which code tab is selected and
/// links its triggers and panels. Pass `value` to control it, or `default_value`
/// to start it; `on_value_change` hears every change the user makes either way.
#[component]
pub fn CodeTabs(
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: Option<String>,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] activation: CodeTabsActivation,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = code_tabs_class(&class);
  let base_id = use_hook(|| format!("dxui-code-tabs-{}", next_element_id()));
  let change = use_callback(move |next: Option<String>| {
    if let (Some(handler), Some(next)) = (on_value_change, next) {
      handler.call(next);
    }
  });
  let value = use_controllable(move || value().map(Some), move || default_value, Some(change));
  let select = use_callback(move |next: String| value.set(Some(next)));
  use_context_provider(|| CodeTabsContext { base_id, value, select, activation });

  rsx! {
    div {
      class,
      {children}
    }
  }
}

/// The horizontal tab bar holding `CodeTabsTrigger` buttons.
#[component]
pub fn CodeTabsList(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let context = use_root_context::<CodeTabsContext>("CodeTabsList", "CodeTabs");
  let class = code_tabs_list_class(&class);
  let scope_id = use_roving_group(Some(context.select));

  rsx! {
    div {
      role: "tablist",
      class,
      "aria-orientation": "horizontal",
      "data-orientation": "horizontal",
      "data-dxui-roving-group": scope_id,
      "data-dxui-roving-orientation": "horizontal",
      "data-dxui-roving-loop": "true",
      "data-dxui-roving-activation": context.activation.roving_attribute(),
      ..attributes,
      {children}
    }
  }
}

/// A tab button that selects the `CodeTabsContent` panel with the matching `value`.
#[component]
pub fn CodeTabsTrigger(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let context = use_root_context::<CodeTabsContext>("CodeTabsTrigger", "CodeTabs");
  let active = context.value.get().as_deref() == Some(value.as_str());
  let class =
    code_tabs_trigger_class(active, &with_density(density_control_class(use_density()), &class));
  let id = group_part_id(&context.base_id, "trigger", &value);
  let controls = group_part_id(&context.base_id, "content", &value);

  rsx! {
    button {
      r#type: "button",
      role: "tab",
      id,
      class,
      disabled,
      "aria-selected": active.to_string(),
      "aria-controls": controls,
      "data-value": value,
      "data-dxui-roving-item": "",
      {children}
    }
  }
}

/// The code panel shown when its `value` is selected in the enclosing `CodeTabs`.
#[component]
pub fn CodeTabsContent(
  value: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let context = use_root_context::<CodeTabsContext>("CodeTabsContent", "CodeTabs");
  let active = context.value.get().as_deref() == Some(value.as_str());
  let class = code_tabs_content_class(&class);
  let id = group_part_id(&context.base_id, "content", &value);
  let labelledby = group_part_id(&context.base_id, "trigger", &value);

  rsx! {
    div {
      role: "tabpanel",
      id,
      class,
      tabindex: "0",
      hidden: !active,
      "aria-labelledby": labelledby,
      "data-value": value,
      {children}
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

  fn sample_code_tabs() -> Element {
    rsx! {
      CodeTabsList {
        CodeTabsTrigger { value: "npm", "npm" }
        CodeTabsTrigger { value: "yarn", "Yarn" }
        CodeTabsTrigger { value: "pnpm", "pnpm" }
      }
      CodeTabsContent { value: "npm", "npm run swizzle" }
      CodeTabsContent { value: "yarn", "yarn swizzle" }
      CodeTabsContent { value: "pnpm", "pnpm run swizzle" }
    }
  }

  #[test]
  fn ssr_code_tabs_select_the_default_and_link_their_parts() {
    fn app() -> Element {
      rsx! { CodeTabs { default_value: "npm", {sample_code_tabs()} } }
    }
    let html = render(app);

    assert!(html.contains(r#"id="dxui-code-tabs-0-trigger-npm""#), "{html}");
    assert!(html.contains(r#"aria-controls="dxui-code-tabs-0-content-npm""#));
    assert!(html.contains(r#"aria-labelledby="dxui-code-tabs-0-trigger-npm""#));
    assert_eq!(html.matches(r#"aria-selected="true""#).count(), 1);
    assert!(html.contains(r#"aria-selected="true" aria-controls="dxui-code-tabs-0-content-npm""#));
    assert_eq!(html.matches("hidden").count(), 2);
  }

  #[test]
  fn ssr_controlled_value_wins_over_the_default() {
    fn app() -> Element {
      rsx! { CodeTabs { value: "pnpm", default_value: "npm", {sample_code_tabs()} } }
    }
    let html = render(app);

    assert!(html.contains(r#"aria-selected="true" aria-controls="dxui-code-tabs-0-content-pnpm""#));
    assert_eq!(html.matches(r#"aria-selected="true""#).count(), 1);
  }

  #[test]
  fn a_part_outside_its_code_tabs_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        CodeTabsTrigger { value: "npm", "npm" }
        p { "after" }
      }
    }
    let html = render(app);

    assert_eq!(html, "<p>before</p><p>after</p>");
  }

  #[test]
  fn code_tabs_trigger_class_reflects_active_state() {
    let active = code_tabs_trigger_class(true, "px-4");
    let inactive = code_tabs_trigger_class(false, "");

    assert!(active.contains("border-primary text-primary"));
    assert!(active.ends_with("px-4"));
    assert!(inactive.contains("border-transparent text-muted-foreground hover:text-foreground"));
  }

  #[test]
  fn code_tabs_activation_maps_to_roving_attribute() {
    assert_eq!(CodeTabsActivation::default().roving_attribute(), "focus");
    assert_eq!(CodeTabsActivation::Manual.roving_attribute(), "manual");
  }

  #[test]
  fn code_tabs_content_class_appends_user_class() {
    let actual = code_tabs_content_class("p-6");

    assert!(actual.contains("rounded-lg bg-muted"));
    assert!(actual.ends_with("p-6"));
  }
}
