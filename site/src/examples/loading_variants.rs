use dioxus::prelude::*;
use dioxus_shadcn::{Loading, LoadingSize, LoadingVariant};

#[component]
pub fn LoadingVariantsDemo() -> Element {
  rsx! {
    div { class: "flex flex-wrap items-center gap-6 p-4 text-primary",
      Loading { variant: LoadingVariant::Spinner, size: LoadingSize::Md, label: "Spinner".to_string() }
      Loading { variant: LoadingVariant::Dots, size: LoadingSize::Lg, label: "Dots".to_string() }
      Loading { variant: LoadingVariant::Ring, size: LoadingSize::Md, label: "Ring".to_string() }
      Loading { variant: LoadingVariant::Ball, size: LoadingSize::Md, label: "Ball".to_string() }
      Loading { variant: LoadingVariant::Bars, size: LoadingSize::Lg, label: "Bars".to_string() }
      Loading { variant: LoadingVariant::Infinity, size: LoadingSize::Xl, label: "Infinity".to_string() }
    }
  }
}
