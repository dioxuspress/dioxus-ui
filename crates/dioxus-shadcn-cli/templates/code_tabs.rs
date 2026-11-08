//! Code Tabs: an underlined tab bar that switches between code panels for
//! side-by-side language, tool, or command comparisons.
use super::density::{density_control_class, use_density, with_density};
use super::element_id::next_element_id;
use super::root_state::{Controllable, use_controllable, use_root_context};
use super::roving_group::{group_part_id, use_roving_group};
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

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
