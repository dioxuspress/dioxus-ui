use dioxus::prelude::*;
use dioxus_shadcn::{
  Calendar, CalendarBody, CalendarDate, CalendarDay, CalendarGrid, CalendarHead, CalendarHeadCell,
  CalendarHeader, CalendarMonth, CalendarNav, CalendarNavButton, CalendarNavDirection, CalendarRow,
  CalendarSelect, CalendarWeekday, CalendarWheel, calendar_clamp_date, calendar_month_grid,
  calendar_move_date,
};

#[component]
pub fn CalendarWheelDemo() -> Element {
  let mut selected = use_signal(|| CalendarDate::unchecked(2026, 10, 15));
  let mut month = use_signal(|| CalendarMonth::unchecked(2026, 10));
  let mut focused = use_signal(|| CalendarDate::unchecked(2026, 10, 15));
  let mut sync_date = move |date: CalendarDate| {
    selected.set(date);
    focused.set(date);
    month.set(CalendarMonth::unchecked(date.year, date.month));
  };
  let grid =
    calendar_month_grid(month(), CalendarWeekday::Sunday, None, Some(selected()), None, None, &[]);

  rsx! {
    div { class: "flex flex-wrap items-start gap-6",
      div { class: "grid gap-2",
        p { class: "text-xs font-medium text-muted-foreground", "Year / Month dropdown" }
        Calendar {
          CalendarHeader {
            div { class: "flex items-center gap-1.5",
              CalendarSelect {
                "aria-label": "Year",
                value: month().year.to_string(),
                onchange: move |event: FormEvent| {
                  if let Ok(year) = event.value().parse::<i32>() {
                    month.set(CalendarMonth::unchecked(year, month().month));
                  }
                },
                for y in 1990..=2040 {
                  option { key: "{y}", value: "{y}", selected: y == month().year, "{y}" }
                }
              }
              CalendarSelect {
                "aria-label": "Month",
                value: month().month.to_string(),
                onchange: move |event: FormEvent| {
                  if let Ok(m) = event.value().parse::<u8>() {
                    month.set(CalendarMonth::unchecked(month().year, m));
                  }
                },
                for m in 1..=12_u8 {
                  option { key: "{m}", value: "{m}", selected: m == month().month, "{m:02}" }
                }
              }
            }
            CalendarNav {
              CalendarNavButton {
                direction: CalendarNavDirection::Previous,
                onclick: move |_| month.set(month().add_months(-1)),
                "<"
              }
              CalendarNavButton {
                direction: CalendarNavDirection::Next,
                onclick: move |_| month.set(month().add_months(1)),
                ">"
              }
            }
          }
          CalendarGrid { "aria-label": "Calendar with year and month dropdowns",
            CalendarHead {
              for day_name in ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
                CalendarHeadCell { key: "{day_name}", "{day_name}" }
              }
            }
            CalendarBody {
              for week in grid.weeks {
                CalendarRow {
                  for day in week {
                    CalendarDay {
                      key: "{day.date.year}-{day.date.month}-{day.date.day}",
                      date: day.date,
                      selected: day.selected,
                      outside_month: day.outside_month,
                      focused: day.date == focused(),
                      on_key_move: move |key_move| {
                        let next = calendar_move_date(focused(), key_move, CalendarWeekday::Sunday);
                        focused.set(next);
                        month.set(CalendarMonth::unchecked(next.year, next.month));
                      },
                      on_select: move |date| {
                        selected.set(date);
                        focused.set(date);
                        month.set(CalendarMonth::unchecked(date.year, date.month));
                      },
                      "{day.date.day}"
                    }
                  }
                }
              }
            }
          }
        }
      }
      div { class: "grid gap-2",
        p { class: "text-xs font-medium text-muted-foreground", "Year / Month / Day wheel" }
        CalendarWheel {
          value: selected(),
          min_year: 1990,
          max_year: 2040,
          on_value_change: move |date: CalendarDate| {
            sync_date(calendar_clamp_date(date.year, date.month, date.day));
          },
        }
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground",
      "Selected: {selected().year}-{selected().month:02}-{selected().day:02}"
    }
  }
}
