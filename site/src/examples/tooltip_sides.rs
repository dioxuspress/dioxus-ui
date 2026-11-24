use dioxus::prelude::*;
use dioxus_shadcn::{
  ButtonSize, ButtonVariant, Tooltip, TooltipContent, TooltipSide, TooltipTrigger, button_class,
  use_density,
};

const SIDES: [(&str, TooltipSide, &str); 4] = [
  ("Top", TooltipSide::Top, "Tooltip anchored above trigger"),
  ("Right", TooltipSide::Right, "Tooltip anchored to the right"),
  ("Bottom", TooltipSide::Bottom, "Tooltip anchored below trigger"),
  ("Left", TooltipSide::Left, "Tooltip anchored to the left"),
];

#[component]
pub fn TooltipSidesDemo() -> Element {
  let trigger_class = button_class(ButtonVariant::Outline, ButtonSize::Sm, use_density(), "");

  rsx! {
    div { class: "flex flex-wrap items-center gap-3",
      for (label, side, text) in SIDES {
        Tooltip { key: "{label}", delay_ms: 150,
          TooltipTrigger { class: "{trigger_class}", "{label}" }
          TooltipContent { side, "{text}" }
        }
      }
    }
  }
}
