use dioxus::prelude::*;
use dioxus_shadcn::{ToggleGroup, ToggleGroupItem, ToggleGroupType};

#[component]
pub fn ToggleGroupFormattingDemo() -> Element {
  let mut formats = use_signal(|| vec!["bold".to_string(), "italic".to_string()]);
  let active = formats();
  let summary = if active.is_empty() { "none".to_string() } else { active.join(", ") };

  rsx! {
    div { class: "grid gap-3",
      ToggleGroup {
        selection_type: ToggleGroupType::Multiple,
        "aria-label": "Text formatting",
        default_values: vec!["bold".to_string(), "italic".to_string()],
        on_values_change: move |next| formats.set(next),
        ToggleGroupItem { value: "bold", "Bold" }
        ToggleGroupItem { value: "italic", "Italic" }
        ToggleGroupItem { value: "underline", "Underline" }
        ToggleGroupItem { value: "code", "Code" }
      }
      p { class: "text-sm text-muted-foreground", "Active styles: {summary}" }
    }
  }
}
