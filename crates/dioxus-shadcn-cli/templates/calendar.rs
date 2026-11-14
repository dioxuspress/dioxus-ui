//! Calendar: styled parts for a month grid of dates, with month navigation and single
//! or range selection that the app controls.
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::utils::{classes, merge_classes};
use super::density::{density_control_class, use_density, with_density};
use dioxus::prelude::*;
pub use super::calendar_grid::{
  CalendarDate, CalendarDateTime, CalendarDay as CalendarPrimitiveDay, CalendarKeyMove,
  CalendarMonth, CalendarMonthGrid, CalendarRangeState, CalendarTime, CalendarWeekday,
  calendar_clamp_date, calendar_month_grid, calendar_move_date, calendar_range_state,
  days_in_month, format_date_time, format_time, is_leap_year, parse_date_time, parse_time,
};

/// Which way a `CalendarNavButton` moves the month.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CalendarNavDirection {
  /// Back to the previous month.
  #[default]
  Previous,
  /// On to the next month.
  Next,
}

const CALENDAR_BASE_CLASS: &str =
  "w-fit rounded-md border border-border bg-background p-3 text-foreground";
const CALENDAR_HEADER_BASE_CLASS: &str = "mb-3 flex items-center justify-between gap-2";
const CALENDAR_CAPTION_BASE_CLASS: &str = "text-sm font-medium";
const CALENDAR_NAV_BASE_CLASS: &str = "flex items-center gap-1";
const CALENDAR_NAV_BUTTON_BASE_CLASS: &str = "inline-flex h-8 w-8 items-center justify-center rounded-md text-sm transition-colors hover:bg-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const CALENDAR_GRID_BASE_CLASS: &str = "grid gap-1";
const CALENDAR_HEAD_BASE_CLASS: &str = "grid grid-cols-7 gap-1";
const CALENDAR_HEAD_CELL_BASE_CLASS: &str =
  "flex h-8 w-8 items-center justify-center text-xs font-medium text-muted-foreground";
const CALENDAR_BODY_BASE_CLASS: &str = "grid gap-1";
const CALENDAR_ROW_BASE_CLASS: &str = "grid grid-cols-7 gap-1";
const CALENDAR_DAY_BASE_CLASS: &str = "inline-flex h-8 w-8 items-center justify-center rounded-md text-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const CALENDAR_DAY_SELECTED_CLASS: &str =
  "bg-primary text-primary-foreground hover:bg-primary/90";
const CALENDAR_DAY_TODAY_CLASS: &str = "border border-input";
const CALENDAR_DAY_OUTSIDE_CLASS: &str = "text-muted-foreground";
const CALENDAR_DAY_RANGE_CLASS: &str = "bg-accent text-accent-foreground";
const CALENDAR_SELECT_BASE_CLASS: &str = "h-8 rounded-md border border-input bg-background px-2 text-sm font-medium text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
const CALENDAR_WHEEL_BASE_CLASS: &str =
  "inline-flex items-center gap-1 rounded-md border border-border bg-background p-2 text-foreground";
const CALENDAR_WHEEL_COLUMN_BASE_CLASS: &str =
  "relative h-48 w-16 overflow-hidden rounded-md select-none focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
const CALENDAR_WHEEL_ITEM_HEIGHT_PX: isize = 32;
const CALENDAR_WHEEL_CENTER_OFFSET_PX: isize = 80;

fn calendar_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_BASE_CLASS)]), class)
}

fn calendar_header_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_HEADER_BASE_CLASS)]), class)
}

fn calendar_caption_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_CAPTION_BASE_CLASS)]), class)
}

fn calendar_nav_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_NAV_BASE_CLASS)]), class)
}

fn calendar_nav_button_class(disabled: bool, class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_NAV_BUTTON_BASE_CLASS), disabled.then_some("pointer-events-none opacity-50")]), class)
}

fn calendar_grid_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_GRID_BASE_CLASS)]), class)
}

fn calendar_head_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_HEAD_BASE_CLASS)]), class)
}

fn calendar_head_cell_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_HEAD_CELL_BASE_CLASS)]), class)
}

fn calendar_body_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_BODY_BASE_CLASS)]), class)
}

fn calendar_row_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_ROW_BASE_CLASS)]), class)
}

fn calendar_day_class(
  selected: bool,
  today: bool,
  outside_month: bool,
  disabled: bool,
  range_state: CalendarRangeState,
  class: &str,
) -> String {
  let in_range = matches!(
    range_state,
    CalendarRangeState::Start
      | CalendarRangeState::Middle
      | CalendarRangeState::End
      | CalendarRangeState::Single
  );

  merge_classes(classes([Some(CALENDAR_DAY_BASE_CLASS), (!in_range && !selected).then_some("hover:bg-accent"), (in_range && !selected).then_some(CALENDAR_DAY_RANGE_CLASS), selected.then_some(CALENDAR_DAY_SELECTED_CLASS), today.then_some(CALENDAR_DAY_TODAY_CLASS), outside_month.then_some(CALENDAR_DAY_OUTSIDE_CLASS), disabled.then_some("pointer-events-none opacity-50")]), class)
}

// Set when a day handles a navigation key, and taken by the day that becomes
// focused next, whether Dioxus reuses its element or mounts a new one.
static CALENDAR_FOCUS_PENDING: AtomicBool = AtomicBool::new(false);

/// Maps a key on a focused day to a calendar move: arrows move by a day or a
/// week, Page Up and Page Down by a month (a year with Shift), and Home and End
/// to the start and end of the week.
pub fn calendar_key_move(key: &Key, shift: bool) -> Option<CalendarKeyMove> {
  match key {
    Key::ArrowLeft => Some(CalendarKeyMove::PreviousDay),
    Key::ArrowRight => Some(CalendarKeyMove::NextDay),
    Key::ArrowUp => Some(CalendarKeyMove::PreviousWeek),
    Key::ArrowDown => Some(CalendarKeyMove::NextWeek),
    Key::PageUp if shift => Some(CalendarKeyMove::PreviousYear),
    Key::PageUp => Some(CalendarKeyMove::PreviousMonth),
    Key::PageDown if shift => Some(CalendarKeyMove::NextYear),
    Key::PageDown => Some(CalendarKeyMove::NextMonth),
    Key::Home => Some(CalendarKeyMove::StartOfWeek),
    Key::End => Some(CalendarKeyMove::EndOfWeek),
    _ => None,
  }
}

fn calendar_range_attribute(range_state: CalendarRangeState) -> &'static str {
  match range_state {
    CalendarRangeState::Outside => "outside",
    CalendarRangeState::Single => "single",
    CalendarRangeState::Start => "start",
    CalendarRangeState::Middle => "middle",
    CalendarRangeState::End => "end",
  }
}

#[component]
pub fn Calendar(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_header_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarCaption(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = calendar_caption_class(&class);

  rsx! {
    div {
      class,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn CalendarNav(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_nav_class(&class);

  rsx! {
    div {
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarNavButton(
  #[props(default)] direction: CalendarNavDirection,
  #[props(default)] disabled: bool,
  #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = calendar_nav_button_class(disabled, &with_density(density_control_class(use_density()), &class));
  let label = match direction {
    CalendarNavDirection::Previous => "Go to previous month",
    CalendarNavDirection::Next => "Go to next month",
  };

  rsx! {
    button {
      r#type: "button",
      class,
      disabled,
      "aria-label": label,
      "data-direction": match direction {
        CalendarNavDirection::Previous => "previous",
        CalendarNavDirection::Next => "next",
      },
      onclick: move |event| {
        if let Some(handler) = onclick {
          handler.call(event);
        }
      },
      {children}
    }
  }
}

#[component]
pub fn CalendarGrid(
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = calendar_grid_class(&class);

  rsx! {
    div {
      role: "grid",
      class,
      ..attributes,
      {children}
    }
  }
}

#[component]
pub fn CalendarHead(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_head_class(&class);

  rsx! {
    div {
      role: "row",
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarHeadCell(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_head_cell_class(&class);

  rsx! {
    div {
      role: "columnheader",
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarBody(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_body_class(&class);

  rsx! {
    div {
      role: "rowgroup",
      class,
      {children}
    }
  }
}

#[component]
pub fn CalendarRow(#[props(default)] class: String, children: Element) -> Element {
  let class = calendar_row_class(&class);

  rsx! {
    div {
      role: "row",
      class,
      {children}
    }
  }
}

/// With `on_key_move` the grid is one Tab stop: only the `focused` day has
/// `tabindex="0"`. Navigation keys report a `CalendarKeyMove`; the app applies
/// it with `calendar_move_date` and moves `focused`, and the newly focused day
/// takes DOM focus. Click calls `on_select`.
#[component]
pub fn CalendarDay(
  date: CalendarDate,
  #[props(default)] selected: bool,
  #[props(default)] today: bool,
  #[props(default)] outside_month: bool,
  #[props(default)] disabled: bool,
  #[props(default)] range_state: CalendarRangeState,
  #[props(default)] focused: bool,
  #[props(default)] on_key_move: Option<EventHandler<CalendarKeyMove>>,
  #[props(default)] on_select: Option<EventHandler<CalendarDate>>,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = calendar_day_class(selected, today, outside_month, disabled, range_state, &with_density(density_control_class(use_density()), &class));
  let keyboard_managed = on_key_move.is_some();
  let mut mounted = use_signal(|| None::<Rc<MountedData>>);

  // Runs on mount (after `onmounted`) and whenever this element shows another
  // focused date, so it covers days that Dioxus reuses and days keyed by date
  // that mount when the month changes. Only a pending key move moves focus,
  // so rendering or app-driven changes to `focused` never steal it.
  use_effect(use_reactive((&focused, &date), move |(focused, _)| {
    if !focused || !CALENDAR_FOCUS_PENDING.swap(false, Ordering::Relaxed) {
      return;
    }
    if let Some(element) = mounted.peek().clone() {
      spawn(async move {
        // A focus error means the day is hidden or gone; nothing to focus.
        let _ = element.set_focus(true).await;
      });
    }
  }));

  rsx! {
    button {
      r#type: "button",
      role: "gridcell",
      class,
      disabled,
      tabindex: keyboard_managed.then_some(if focused { "0" } else { "-1" }),
      "data-dxui-autofocus": (keyboard_managed && focused).then_some("true"),
      onmounted: move |event| mounted.set(Some(event.data())),
      onkeydown: move |event| {
        let key_move = calendar_key_move(&event.key(), event.modifiers().shift());
        if let (Some(handler), Some(key_move)) = (on_key_move, key_move) {
          event.prevent_default();
          CALENDAR_FOCUS_PENDING.store(true, Ordering::Relaxed);
          handler.call(key_move);
        }
      },
      onclick: move |_| {
        if let Some(handler) = on_select {
          handler.call(date);
        }
      },
      "aria-disabled": disabled.to_string(),
      "aria-selected": selected.to_string(),
      "data-date": format!("{:04}-{:02}-{:02}", date.year, date.month, date.day),
      "data-outside-month": outside_month.to_string(),
      "data-range": calendar_range_attribute(range_state),
      "data-selected": selected.to_string(),
      "data-today": today.to_string(),
      {children}
    }
  }
}

/// Classes for a compact native `<select>` inside `CalendarHeader` for quick year/month jumps.
pub fn calendar_select_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_SELECT_BASE_CLASS)]), class)
}

/// Classes for the `CalendarWheel` container.
pub fn calendar_wheel_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_WHEEL_BASE_CLASS)]), class)
}

/// Classes for a single `CalendarWheelColumn`.
pub fn calendar_wheel_column_class(class: &str) -> String {
  merge_classes(classes([Some(CALENDAR_WHEEL_COLUMN_BASE_CLASS)]), class)
}

/// Compact native `<select>` for jumping directly to a year or month in `CalendarHeader`.
#[component]
pub fn CalendarSelect(
  #[props(default)] value: String,
  #[props(default)] disabled: bool,
  #[props(default)] onchange: Option<EventHandler<FormEvent>>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = select)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let class = calendar_select_class(&class);

  rsx! {
    select {
      class,
      value,
      disabled,
      onchange: move |event| {
        if let Some(handler) = onchange {
          handler.call(event);
        }
      },
      ..attributes,
      {children}
    }
  }
}

fn wheel_step_index(current: usize, delta: isize, len: usize) -> usize {
  if len == 0 {
    return 0;
  }
  let max = (len - 1) as isize;
  (current as isize + delta).clamp(0, max) as usize
}

/// A single vertical drum/wheel column supporting mouse wheel, pointer swipe,
/// item click, and keyboard (`ArrowUp` / `ArrowDown` / `PageUp` / `PageDown` /
/// `Home` / `End`) selection.
#[component]
pub fn CalendarWheelColumn(
  label: String,
  items: Vec<String>,
  selected_index: usize,
  #[props(default)] on_select: Option<EventHandler<usize>>,
  #[props(default)] class: String,
) -> Element {
  let len = items.len();
  let clamped = if len == 0 { 0 } else { selected_index.min(len - 1) };
  let offset_px = CALENDAR_WHEEL_CENTER_OFFSET_PX - (clamped as isize * CALENDAR_WHEEL_ITEM_HEIGHT_PX);
  let class = calendar_wheel_column_class(&class);
  let mut wheel_acc = use_signal(|| 0.0_f64);
  let mut drag_start_y = use_signal(|| None::<f64>);

  let emit_step = move |delta: isize| {
    if len == 0 {
      return;
    }
    let next = wheel_step_index(clamped, delta, len);
    if next != clamped {
      if let Some(handler) = on_select {
        handler.call(next);
      }
    }
  };

  rsx! {
    div {
      role: "listbox",
      "aria-label": "{label}",
      tabindex: "0",
      class,
      onwheel: move |event| {
        event.prevent_default();
        let dy = event.delta().strip_units().y;
        let next_acc = wheel_acc() + dy;
        let step_px = 20.0;
        if next_acc >= step_px {
          wheel_acc.set(0.0);
          emit_step(1);
        } else if next_acc <= -step_px {
          wheel_acc.set(0.0);
          emit_step(-1);
        } else {
          wheel_acc.set(next_acc);
        }
      },
      onpointerdown: move |event| {
        drag_start_y.set(Some(event.client_coordinates().y));
      },
      onpointermove: move |event| {
        if let Some(start_y) = drag_start_y() {
          let current_y = event.client_coordinates().y;
          let diff = start_y - current_y;
          if diff.abs() >= 24.0 {
            let steps = (diff / 24.0).trunc() as isize;
            drag_start_y.set(Some(current_y));
            emit_step(steps);
          }
        }
      },
      onpointerup: move |_| drag_start_y.set(None),
      onpointercancel: move |_| drag_start_y.set(None),
      onkeydown: move |event| {
        let delta = match event.key() {
          Key::ArrowUp => Some(-1),
          Key::ArrowDown => Some(1),
          Key::PageUp => Some(-5),
          Key::PageDown => Some(5),
          Key::Home => Some(-(clamped as isize)),
          Key::End => Some((len.saturating_sub(1) as isize) - (clamped as isize)),
          _ => None,
        };
        if let Some(delta) = delta {
          event.prevent_default();
          emit_step(delta);
        }
      },
      div {
        class: "pointer-events-none absolute inset-x-0.5 top-1/2 h-8 -translate-y-1/2 rounded-md border border-border bg-accent/50",
        "aria-hidden": "true",
      }
      div {
        class: "transition-transform duration-150 ease-out",
        style: "transform: translateY({offset_px}px);",
        for (idx, item) in items.iter().enumerate() {
          {
            let is_selected = idx == clamped;
            let item_class = if is_selected {
              "relative z-10 flex h-8 w-full items-center justify-center text-sm font-semibold text-foreground tabular-nums"
            } else {
              "relative z-10 flex h-8 w-full items-center justify-center text-sm text-muted-foreground tabular-nums hover:text-foreground"
            };
            rsx! {
              button {
                key: "{idx}-{item}",
                r#type: "button",
                role: "option",
                tabindex: "-1",
                "aria-selected": is_selected.to_string(),
                "data-selected": is_selected.to_string(),
                class: item_class,
                onclick: move |_| {
                  if let Some(handler) = on_select {
                    handler.call(idx);
                  }
                },
                "{item}"
              }
            }
          }
        }
      }
      div {
        class: "pointer-events-none absolute inset-x-0 top-0 h-12 bg-gradient-to-b from-background to-transparent",
        "aria-hidden": "true",
      }
      div {
        class: "pointer-events-none absolute inset-x-0 bottom-0 h-12 bg-gradient-to-t from-background to-transparent",
        "aria-hidden": "true",
      }
    }
  }
}

/// Three-column Year / Month / Day scroll-wheel date picker that clamps days
/// automatically when changing months or leap years.
#[component]
pub fn CalendarWheel(
  value: CalendarDate,
  #[props(default = 1950)] min_year: i32,
  #[props(default = 2050)] max_year: i32,
  #[props(default = "Year".to_string())] year_label: String,
  #[props(default = "Month".to_string())] month_label: String,
  #[props(default = "Day".to_string())] day_label: String,
  #[props(default)] on_value_change: Option<EventHandler<CalendarDate>>,
  #[props(default)] class: String,
) -> Element {
  let max_year = max_year.max(min_year);
  let clamped_year = value.year.clamp(min_year, max_year);
  let clamped_month = value.month.clamp(1, 12);
  let max_day = days_in_month(clamped_year, clamped_month);
  let clamped_day = value.day.clamp(1, max_day);

  let years: Vec<String> = (min_year..=max_year).map(|y| format!("{y:04}")).collect();
  let months: Vec<String> = (1..=12).map(|m| format!("{m:02}")).collect();
  let days: Vec<String> = (1..=max_day).map(|d| format!("{d:02}")).collect();

  let year_index = (clamped_year - min_year) as usize;
  let month_index = (clamped_month - 1) as usize;
  let day_index = (clamped_day - 1) as usize;
  let class = calendar_wheel_class(&class);

  let emit_date = move |year: i32, month: u8, day: u8| {
    if let Some(handler) = on_value_change {
      handler.call(calendar_clamp_date(year, month, day));
    }
  };

  rsx! {
    div {
      class,
      "data-calendar-wheel": "",
      CalendarWheelColumn {
        label: year_label,
        items: years,
        selected_index: year_index,
        class: "w-20".to_string(),
        on_select: move |idx| emit_date(min_year + idx as i32, clamped_month, clamped_day),
      }
      CalendarWheelColumn {
        label: month_label,
        items: months,
        selected_index: month_index,
        on_select: move |idx| emit_date(clamped_year, (idx + 1) as u8, clamped_day),
      }
      CalendarWheelColumn {
        label: day_label,
        items: days,
        selected_index: day_index,
        on_select: move |idx| emit_date(clamped_year, clamped_month, (idx + 1) as u8),
      }
    }
  }
}
