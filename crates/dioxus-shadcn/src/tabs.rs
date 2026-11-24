//! Tabs: a list of triggers that each select one content panel.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

use crate::density::{density_control_class, use_density, with_density};
use crate::element_id::next_element_id;
use crate::root_state::{Controllable, use_controllable, use_root_context};
use crate::roving_group::{group_part_id, use_roving_group};

const TABS_BASE_CLASS: &str = "data-[orientation=vertical]:flex data-[orientation=vertical]:gap-4";
const TABS_LIST_BASE_CLASS: &str = "inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground data-[orientation=vertical]:h-auto data-[orientation=vertical]:flex-col data-[orientation=vertical]:items-stretch data-[orientation=vertical]:justify-start";
const TABS_TRIGGER_BASE_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const TABS_TRIGGER_CORE_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const TABS_CONTENT_BASE_CLASS: &str = "mt-2 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring data-[orientation=vertical]:mt-0";

/// Visual presentation style shared across [`TabsList`] and [`TabsTrigger`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsVariant {
  /// Segmented muted pill container with a raised active trigger (default).
  #[default]
  Default,
  /// Minimalist tab strip with a primary underline (or side indicator when vertical).
  Underline,
  /// Outlined container with a bordered active trigger.
  Bordered,
}

impl TabsVariant {
  /// The `data-variant` attribute value for this style.
  pub const fn attribute(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Underline => "underline",
      Self::Bordered => "bordered",
    }
  }
}

/// Whether moving focus to a trigger also selects it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsActivation {
  /// Arrow, Home, and End keys select the trigger they move to.
  #[default]
  Automatic,
  /// Keys only move focus; Enter, Space, or a click selects.
  Manual,
}

impl TabsActivation {
  /// The roving group script's `data-dxui-roving-activation` value.
  pub fn roving_attribute(self) -> &'static str {
    match self {
      Self::Automatic => "focus",
      Self::Manual => "manual",
    }
  }
}

/// The direction the triggers are laid out and moved through.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsOrientation {
  /// Left and Right move between triggers.
  #[default]
  Horizontal,
  /// Up and Down move between triggers, and the list sits beside the panels.
  Vertical,
}

impl TabsOrientation {
  /// The value for `aria-orientation` and `data-orientation`.
  pub fn attribute(self) -> &'static str {
    match self {
      Self::Horizontal => "horizontal",
      Self::Vertical => "vertical",
    }
  }
}

/// Classes for the root, which lays the list beside the panels when vertical.
pub fn tabs_class(class: &str) -> String {
  merge_classes(classes([Some(TABS_BASE_CLASS)]), class)
}

/// Classes for the list that holds the triggers.
pub fn tabs_list_class(class: &str) -> String {
  merge_classes(classes([Some(TABS_LIST_BASE_CLASS)]), class)
}

/// Classes for a [`TabsList`] styled for a specific [`TabsVariant`].
pub fn tabs_list_variant_class(variant: TabsVariant, class: &str) -> String {
  match variant {
    TabsVariant::Default => tabs_list_class(class),
    TabsVariant::Underline => merge_classes(
      classes([Some(
        "inline-flex h-auto items-center justify-start gap-4 rounded-none border-b border-border bg-transparent p-0 text-muted-foreground data-[orientation=vertical]:flex-col data-[orientation=vertical]:items-stretch data-[orientation=vertical]:gap-1 data-[orientation=vertical]:border-b-0 data-[orientation=vertical]:border-e",
      )]),
      class,
    ),
    TabsVariant::Bordered => merge_classes(
      classes([Some(
        "inline-flex h-auto items-center justify-start gap-1 rounded-md border border-border bg-background p-1 text-muted-foreground data-[orientation=vertical]:flex-col data-[orientation=vertical]:items-stretch",
      )]),
      class,
    ),
  }
}

/// Classes for the trigger: base classes, raised while `active` and muted otherwise, with `class`
/// merged over them.
pub fn tabs_trigger_class(active: bool, class: &str) -> String {
  let active_class = if active {
    "bg-background text-foreground shadow-sm"
  } else {
    "text-muted-foreground hover:text-foreground"
  };

  merge_classes(classes([Some(TABS_TRIGGER_BASE_CLASS), Some(active_class)]), class)
}

/// Classes for a [`TabsTrigger`] styled for a specific [`TabsVariant`].
pub fn tabs_trigger_variant_class(variant: TabsVariant, active: bool, class: &str) -> String {
  match variant {
    TabsVariant::Default => tabs_trigger_class(active, class),
    TabsVariant::Underline => {
      let state_class = if active {
        "rounded-none border-b-2 border-primary bg-transparent px-1 pb-2.5 pt-1.5 text-foreground data-[orientation=vertical]:border-b-0 data-[orientation=vertical]:border-e-2 data-[orientation=vertical]:px-3 data-[orientation=vertical]:py-2"
      } else {
        "rounded-none border-b-2 border-transparent bg-transparent px-1 pb-2.5 pt-1.5 text-muted-foreground hover:text-foreground data-[orientation=vertical]:border-b-0 data-[orientation=vertical]:border-e-2 data-[orientation=vertical]:px-3 data-[orientation=vertical]:py-2"
      };
      merge_classes(classes([Some(TABS_TRIGGER_CORE_CLASS), Some(state_class)]), class)
    }
    TabsVariant::Bordered => {
      let state_class = if active {
        "rounded-sm border border-border bg-muted/60 px-3 py-1.5 text-foreground"
      } else {
        "rounded-sm border border-transparent bg-transparent px-3 py-1.5 text-muted-foreground hover:bg-muted/40 hover:text-foreground"
      };
      merge_classes(classes([Some(TABS_TRIGGER_CORE_CLASS), Some(state_class)]), class)
    }
  }
}

/// Classes for the content panel.
pub fn tabs_content_class(class: &str) -> String {
  merge_classes(classes([Some(TABS_CONTENT_BASE_CLASS)]), class)
}

/// What `Tabs` shares with its parts (RFC 0077).
#[derive(Clone)]
struct TabsContext {
  base_id: String,
  value: Controllable<Option<String>>,
  select: Callback<String>,
  activation: TabsActivation,
  orientation: TabsOrientation,
  variant: TabsVariant,
}

/// The root of a tab set: it owns which tab is selected and links its parts.
/// Pass `value` to control it, or `default_value` to start it; the change
/// callback hears every change the user makes either way.
#[component]
pub fn Tabs(
  #[props(default)] value: ReadSignal<Option<String>>,
  #[props(default)] default_value: Option<String>,
  #[props(default)] on_value_change: Option<EventHandler<String>>,
  #[props(default)] activation: TabsActivation,
  #[props(default)] orientation: TabsOrientation,
  #[props(default)] variant: TabsVariant,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = tabs_class(&class);
  let base_id = use_hook(|| format!("dxui-tabs-{}", next_element_id()));
  let change = use_callback(move |next: Option<String>| {
    if let (Some(handler), Some(next)) = (on_value_change, next) {
      handler.call(next);
    }
  });
  let value = use_controllable(move || value().map(Some), move || default_value, Some(change));
  let select = use_callback(move |next: String| value.set(Some(next)));
  use_context_provider(|| TabsContext { base_id, value, select, activation, orientation, variant });

  rsx! {
    div {
      class,
      "data-orientation": orientation.attribute(),
      "data-variant": variant.attribute(),
      {children}
    }
  }
}

/// Container that arranges [`TabsTrigger`] controls with roving keyboard focus.
#[component]
pub fn TabsList(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let context = use_root_context::<TabsContext>("TabsList", "Tabs");
  let class = tabs_list_variant_class(context.variant, &class);
  let orientation = context.orientation.attribute();
  let scope_id = use_roving_group(Some(context.select));

  rsx! {
    div {
      role: "tablist",
      class,
      "aria-orientation": orientation,
      "data-orientation": orientation,
      "data-variant": context.variant.attribute(),
      "data-dxui-roving-group": scope_id,
      "data-dxui-roving-orientation": orientation,
      "data-dxui-roving-loop": "true",
      "data-dxui-roving-activation": context.activation.roving_attribute(),
      ..attributes,
      {children}
    }
  }
}

/// Interactive button inside [`TabsList`] that selects a matching [`TabsContent`] panel.
#[component]
pub fn TabsTrigger(
  value: String,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let context = use_root_context::<TabsContext>("TabsTrigger", "Tabs");
  let active = context.value.get().as_deref() == Some(value.as_str());
  let class = tabs_trigger_variant_class(
    context.variant,
    active,
    &with_density(density_control_class(use_density()), &class),
  );
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
      "data-orientation": context.orientation.attribute(),
      "data-variant": context.variant.attribute(),
      "data-value": value,
      "data-dxui-roving-item": "",
      {children}
    }
  }
}

/// Content panel associated with a [`TabsTrigger`] of the same `value`.
#[component]
pub fn TabsContent(value: String, #[props(default)] class: String, children: Element) -> Element {
  let context = use_root_context::<TabsContext>("TabsContent", "Tabs");
  let active = context.value.get().as_deref() == Some(value.as_str());
  let class = tabs_content_class(&class);
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
      "data-orientation": context.orientation.attribute(),
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

  fn account_tabs() -> Element {
    rsx! {
      TabsList {
        TabsTrigger { value: "account", "Account" }
        TabsTrigger { value: "password", "Password" }
      }
      TabsContent { value: "account", "Account settings" }
      TabsContent { value: "password", "Password settings" }
    }
  }

  #[test]
  fn ssr_tabs_select_the_default_and_link_their_parts() {
    fn app() -> Element {
      rsx! { Tabs { default_value: "account", {account_tabs()} } }
    }
    let html = render(app);

    assert!(html.contains(r#"id="dxui-tabs-0-trigger-account""#), "{html}");
    assert!(html.contains(r#"aria-controls="dxui-tabs-0-content-account""#));
    assert!(html.contains(r#"aria-labelledby="dxui-tabs-0-trigger-account""#));
    assert_eq!(html.matches(r#"aria-selected="true""#).count(), 1);
    assert!(html.contains(r#"aria-selected="true" aria-controls="dxui-tabs-0-content-account""#));
    assert_eq!(html.matches("hidden").count(), 1);
    assert!(
      html.contains(r#"hidden data-orientation="horizontal" data-value="password""#)
        || html.contains(r#"hidden=true"#)
        || html.contains("hidden")
    );
  }

  #[test]
  fn ssr_controlled_value_wins_over_the_default() {
    fn app() -> Element {
      rsx! { Tabs { value: "password", default_value: "account", {account_tabs()} } }
    }
    let html = render(app);

    assert!(html.contains(r#"aria-selected="true" aria-controls="dxui-tabs-0-content-password""#));
    assert_eq!(html.matches(r#"aria-selected="true""#).count(), 1);
  }

  #[test]
  fn two_renders_write_the_same_ids() {
    fn app() -> Element {
      rsx! {
        Tabs { default_value: "account", {account_tabs()} }
        Tabs { default_value: "password", {account_tabs()} }
      }
    }
    let first = render(app);

    assert_eq!(first, render(app));
    assert_eq!(first.matches(r#"role="tablist""#).count(), 2);
    assert!(first.contains(r#"id="dxui-tabs-0-trigger-account""#));
  }

  #[test]
  fn a_part_outside_its_tabs_renders_nothing() {
    fn app() -> Element {
      rsx! {
        p { "before" }
        TabsTrigger { value: "account", "Account" }
        p { "after" }
      }
    }
    let html = render(app);

    assert_eq!(html, "<p>before</p><p>after</p>");
  }

  #[test]
  fn tabs_trigger_class_reflects_active_state() {
    let actual = tabs_trigger_class(true, "min-w-24");

    assert!(actual.contains(TABS_TRIGGER_BASE_CLASS));
    assert!(actual.contains("bg-background text-foreground shadow-sm"));
    assert!(actual.ends_with("min-w-24"));
  }

  #[test]
  fn tabs_activation_maps_to_roving_attribute() {
    assert_eq!(TabsActivation::default().roving_attribute(), "focus");
    assert_eq!(TabsActivation::Manual.roving_attribute(), "manual");
  }

  #[test]
  fn tabs_orientation_maps_to_attribute() {
    assert_eq!(TabsOrientation::default().attribute(), "horizontal");
    assert_eq!(TabsOrientation::Vertical.attribute(), "vertical");
  }

  #[test]
  fn tabs_content_class_appends_user_class() {
    let actual = tabs_content_class("p-4");

    assert!(actual.contains(TABS_CONTENT_BASE_CLASS));
    assert!(actual.ends_with("p-4"));
  }

  #[test]
  fn tabs_variant_flows_through_context_to_list_and_triggers() {
    fn app() -> Element {
      rsx! {
        Tabs { variant: TabsVariant::Underline, default_value: "account", {account_tabs()} }
      }
    }
    let html = render(app);

    assert!(html.contains(r#"data-variant="underline""#), "{html}");
    assert!(html.contains("border-b-2 border-primary"), "{html}");
    assert!(html.contains("border-b-2 border-transparent"), "{html}");

    let bordered = tabs_trigger_variant_class(TabsVariant::Bordered, true, "");
    assert!(bordered.contains("border-border bg-muted/60"));
  }
}
