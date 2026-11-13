use dioxus::prelude::*;
use dioxus_shadcn::{
  CalendarDateTime, CalendarTime, DateTimePickerInput, DateTimeWheel, Label, TimePicker,
  format_date_time, format_time,
};

#[component]
pub fn DateTimePickerWheelDemo() -> Element {
  let mut date_time = use_signal(|| CalendarDateTime::unchecked(2026, 10, 15, 14, 30, 0));
  let mut standalone_time = use_signal(|| CalendarTime::unchecked(9, 15, 0));

  rsx! {
    div { class: "grid gap-6 lg:grid-cols-[auto_auto]",
      div { class: "grid gap-3",
        div { class: "grid max-w-xs gap-2",
          Label { r#for: "dt-wheel-input", "Full wheel & typed input" }
          DateTimePickerInput {
            id: "dt-wheel-input",
            value: Some(date_time()),
            on_value_change: move |next: Option<CalendarDateTime>| {
              if let Some(dt) = next {
                date_time.set(dt);
              }
            },
          }
        }
        DateTimeWheel {
          value: date_time(),
          min_year: 2020,
          max_year: 2035,
          on_value_change: move |dt| date_time.set(dt),
        }
        p { class: "text-sm text-muted-foreground",
          "Selected: {format_date_time(date_time(), false)}"
        }
      }
      div { class: "grid content-start gap-3",
        p { class: "text-sm font-medium", "Standalone TimePicker" }
        TimePicker {
          value: standalone_time(),
          include_seconds: true,
          on_value_change: move |t| standalone_time.set(t),
        }
        p { class: "text-sm text-muted-foreground",
          "Time: {format_time(standalone_time(), true)}"
        }
      }
    }
  }
}
