use dioxus::prelude::*;
use dioxus_shadcn::{
  Combobox, ComboboxContent, ComboboxGroup, ComboboxItem, ComboboxList, ComboboxStatus,
  ComboboxTrigger, ComboboxValue, Label,
};

const MODELS: [(&str, &str, &str); 6] = [
  ("Reasoning", "r1", "DeepSeek R1"),
  ("Reasoning", "o3", "OpenAI o3"),
  ("General", "gemini", "Gemini 2.5 Pro"),
  ("General", "claude", "Claude Sonnet"),
  ("Edge", "qwen", "Qwen 3 8B"),
  ("Edge", "gemma", "Gemma 3 12B"),
];

#[component]
pub fn ComboboxGroupsDemo() -> Element {
  let mut selected = use_signal(|| "gemini".to_string());
  let selected_label = MODELS
    .iter()
    .find(|(_, id, _)| *id == selected())
    .map_or("Select a model", |(_, _, label)| *label);

  rsx! {
    div { class: "grid max-w-xs gap-2",
      Label { r#for: "combobox-groups-trigger", "Default model" }
      Combobox {
        id: "combobox-groups-trigger",
        value: Some(selected()),
        on_value_change: move |next| selected.set(next),
        ComboboxTrigger {
          ComboboxValue { "{selected_label}" }
          span { class: "ms-2 text-xs text-muted-foreground", "aria-hidden": "true", "▾" }
        }
        ComboboxStatus { "Selected model: {selected_label}" }
        ComboboxContent {
          ComboboxList {
            for group in ["Reasoning", "General", "Edge"] {
              ComboboxGroup { key: "{group}",
                div { class: "px-2 py-1 text-xs font-medium text-muted-foreground", "{group}" }
                for (_, id, label) in MODELS.iter().filter(|(g, _, _)| *g == group) {
                  ComboboxItem { key: "{id}", value: *id, "{label}" }
                }
              }
            }
          }
        }
      }
      p { class: "text-sm text-muted-foreground", "Active model: {selected_label}" }
    }
  }
}
