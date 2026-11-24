use dioxus::prelude::*;
use dioxus_shadcn::{Badge, BadgeSize, BadgeStyle, BadgeVariant};

#[component]
pub fn BadgeVariantsDemo() -> Element {
  rsx! {
    div { class: "grid gap-3",
      div { class: "flex flex-wrap items-center gap-2",
        Badge { "Default" }
        Badge { variant: BadgeVariant::Secondary, "Secondary" }
        Badge { variant: BadgeVariant::Destructive, "Destructive" }
        Badge { variant: BadgeVariant::Outline, "Outline" }
      }
      div { class: "flex flex-wrap items-center gap-2",
        Badge { variant: BadgeVariant::Default, style: BadgeStyle::Soft, "Soft primary" }
        Badge { variant: BadgeVariant::Success, style: BadgeStyle::Soft, pill: true, "Soft success" }
        Badge { variant: BadgeVariant::Warning, style: BadgeStyle::Outline, pill: true, "Outline warning" }
        Badge { variant: BadgeVariant::Info, style: BadgeStyle::Dash, "Dash info" }
      }
      div { class: "flex flex-wrap items-center gap-2",
        Badge { size: BadgeSize::Xs, pill: true, "XS" }
        Badge { size: BadgeSize::Sm, pill: true, "SM" }
        Badge { size: BadgeSize::Md, pill: true, "MD" }
        Badge { size: BadgeSize::Lg, pill: true, "LG" }
      }
    }
  }
}
