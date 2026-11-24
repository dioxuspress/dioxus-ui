use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Stat, StatDescription, StatFigure,
  StatGroup, StatTitle, StatValue,
};

#[component]
pub fn StatActionsDemo() -> Element {
  let mut credits = use_signal(|| 1240_u32);

  rsx! {
    StatGroup { class: "max-w-full",
      Stat {
        StatTitle { "Compute credits" }
        StatValue { "${credits}" }
        StatDescription {
          div { class: "mt-2 flex gap-2",
            Button { size: ButtonSize::Sm, onclick: move |_| credits += 100, "Add $100" }
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Usage" }
          }
        }
        StatFigure {
          Badge { variant: BadgeVariant::Success, "Healthy" }
        }
      }
      Stat {
        StatTitle { "Pending payout" }
        StatValue { "$860" }
        StatDescription {
          div { class: "mt-2 flex gap-2",
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Withdraw" }
          }
        }
        StatFigure {
          Badge { variant: BadgeVariant::Outline, "Net-15" }
        }
      }
    }
  }
}
