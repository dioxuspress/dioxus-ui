use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeStyle, Button, ButtonSize, ButtonVariant, FileDropzone, FileInput, Label};

#[component]
pub fn FileInputDropzoneDemo() -> Element {
  let mut files = use_signal(|| vec!["architecture-spec.pdf".to_string(), "tokens.json".to_string()]);
  let mut disabled = use_signal(|| false);

  rsx! {
    div { class: "grid max-w-md gap-3",
      FileDropzone {
        disabled: disabled(),
        "aria-label": "Release asset upload area",
        on_files: move |dropped: Vec<dioxus::html::FileData>| {
          let names = dropped.iter().map(|file| file.name()).collect::<Vec<_>>();
          files.write().extend(names);
        },
        div { class: "grid gap-2",
          p { class: "font-medium text-foreground", "Drag and drop files here" }
          p { class: "text-xs text-muted-foreground",
            "Supports PDF, PNG, and JSON up to 10 MB. Keyboard users can pick files below."
          }
          div { class: "mx-auto w-full max-w-xs pt-1",
            Label { r#for: "file-dropzone-picker", class: "sr-only", "Choose files to upload" }
            FileInput {
              id: "file-dropzone-picker",
              multiple: true,
              disabled: disabled(),
              onchange: move |event: FormEvent| {
                let names = event.files().iter().map(|file| file.name()).collect::<Vec<_>>();
                files.write().extend(names);
              },
            }
          }
        }
      }
      div { class: "flex flex-wrap items-center justify-between gap-2",
        div { class: "flex flex-wrap gap-1.5",
          for name in files() {
            Badge { key: "{name}", style: BadgeStyle::Soft, "{name}" }
          }
        }
        Button {
          size: ButtonSize::Xs,
          variant: ButtonVariant::Outline,
          onclick: move |_| disabled.toggle(),
          if disabled() { "Enable dropzone" } else { "Disable dropzone" }
        }
      }
    }
  }
}
