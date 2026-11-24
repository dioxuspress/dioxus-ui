use dioxus::prelude::*;
use dioxus_shadcn::{
  ContextMenu, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuLabel,
  ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSeparator, ContextMenuSub,
  ContextMenuSubContent, ContextMenuSubTrigger, ContextMenuTrigger,
};

#[component]
pub fn ContextMenuSubmenuDemo() -> Element {
  let mut zoom = use_signal(|| "100".to_string());
  let mut last_export = use_signal(|| "none".to_string());

  rsx! {
    ContextMenu {
      ContextMenuTrigger {
        class: "flex h-32 items-center justify-center rounded-md border border-dashed border-input text-sm text-muted-foreground",
        "Right-click canvas for submenu & zoom"
      }
      p { class: "mt-3 text-sm text-muted-foreground", "Zoom: {zoom}% · Export: {last_export}" }
      ContextMenuContent {
        ContextMenuGroup {
          ContextMenuLabel { "Canvas actions" }
          ContextMenuSub {
            ContextMenuSubTrigger { "Export as" }
            ContextMenuSubContent {
              ContextMenuItem { onclick: move |_| last_export.set("SVG".to_string()), "Vector SVG" }
              ContextMenuItem { onclick: move |_| last_export.set("PNG".to_string()), "Retina PNG (2x)" }
              ContextMenuItem { onclick: move |_| last_export.set("PDF".to_string()), "Print PDF" }
            }
          }
        }
        ContextMenuSeparator {}
        ContextMenuLabel { "Zoom level" }
        ContextMenuRadioGroup {
          value: Some(zoom()),
          on_value_change: move |next| zoom.set(next),
          ContextMenuRadioItem { value: "75", "75%" }
          ContextMenuRadioItem { value: "100", "100% (Actual)" }
          ContextMenuRadioItem { value: "150", "150%" }
        }
      }
    }
  }
}
