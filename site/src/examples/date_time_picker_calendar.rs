use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, Calendar, CalendarBody, CalendarDate, CalendarDateTime,
  CalendarDay, CalendarGrid, CalendarHead, CalendarHeadCell, CalendarHeader, CalendarMonth,
  CalendarNav, CalendarNavButton, CalendarNavDirection, CalendarRow, CalendarSelect, CalendarTime,
  CalendarWeekday, DateTimePicker, DateTimePickerContent, DateTimePickerTrigger,
  DateTimePickerValue, Label, TimePicker, calendar_month_grid, calendar_move_date,
  format_date_time,
};

#[component]
pub fn DateTimePickerCalendarDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut month = use_signal(|| CalendarMonth::unchecked(2026, 10));
  let mut focused = use_signal(|| CalendarDate::unchecked(2026, 10, 15));
  let mut value = use_signal(|| CalendarDateTime::unchecked(2026, 10, 15, 14, 30, 0));

  let grid = calendar_month_grid(
    month(),
    CalendarWeekday::Sunday,
    None,
    Some(value().date),
    None,
    None,
    &[],
  );
  let formatted = format_date_time(value(), false);

  rsx! {
    DateTimePicker {
      id: "dt-calendar-trigger",
      open: open(),
      on_open_change: move |next| open.set(next),
      div { class: "grid max-w-xs gap-2",
        Label { r#for: "dt-calendar-trigger", "Meeting time" }
        DateTimePickerTrigger {
          DateTimePickerValue { placeholder: "Pick date & time", "{formatted}" }
        }
      }
      DateTimePickerContent {
        div { class: "flex flex-col gap-3 p-3 sm:flex-row sm:items-start",
          Calendar { class: "border-0 p-0".to_string(),
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
                  for y in 2020..=2035 {
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
            CalendarGrid { "aria-label": "Meeting date",
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
                          focused.set(date);
                          value.set(CalendarDateTime::new(date, value().time));
                        },
                        "{day.date.day}"
                      }
                    }
                  }
                }
              }
            }
          }
          div { class: "flex flex-col items-center gap-2 border-t border-border pt-3 sm:border-s sm:border-t-0 sm:ps-3 sm:pt-0",
            TimePicker {
              value: value().time,
              minute_step: 5,
              on_value_change: move |time: CalendarTime| {
                value.set(CalendarDateTime::new(value().date, time));
              },
            }
            div { class: "flex w-full justify-end gap-2 pt-1",
              Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Sm,
                onclick: move |_| {
                  let default_dt = CalendarDateTime::unchecked(2026, 10, 15, 9, 0, 0);
                  value.set(default_dt);
                  focused.set(default_dt.date);
                  month.set(CalendarMonth::unchecked(2026, 10));
                },
                "Reset"
              }
              Button {
                size: ButtonSize::Sm,
                onclick: move |_| open.set(false),
                "Done"
              }
            }
          }
        }
      }
    }
  }
}
