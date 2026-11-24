use dioxus::prelude::*;
use dioxus_shadcn::{
  Stat, StatDescription, StatGroup, StatGroupOrientation, StatTitle, StatValue,
};

#[component]
pub fn StatVerticalDemo() -> Element {
  rsx! {
    StatGroup { orientation: StatGroupOrientation::Vertical, class: "w-full max-w-xs",
      Stat {
        StatTitle { "Active nodes" }
        StatValue { "48" }
        StatDescription { "Across 4 edge regions" }
      }
      Stat {
        StatTitle { "Median build time" }
        StatValue { "14.2s" }
        StatDescription { "-3.8s after workspace cache" }
      }
      Stat {
        StatTitle { "Error budget left" }
        StatValue { "99.4%" }
        StatDescription { "30-day rolling SLO" }
      }
    }
  }
}
