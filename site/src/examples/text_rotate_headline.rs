use dioxus::prelude::*;
use dioxus_shadcn::TextRotate;

#[component]
pub fn TextRotateHeadlineDemo() -> Element {
  rsx! {
    div { class: "p-4",
      h2 { class: "text-2xl font-bold tracking-tight text-foreground",
        "Build "
        TextRotate {
          class: "text-primary px-1",
          items: vec![
            "faster".to_string(),
            "type-safe".to_string(),
            "accessible".to_string(),
            "full-stack".to_string(),
          ],
        }
        " Dioxus apps"
      }
    }
  }
}
