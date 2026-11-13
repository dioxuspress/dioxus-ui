//! Date Time Picker: controlled trigger, value, input, anchored popover,
//! standalone `TimePicker` wheel columns, and full-wheel `DateTimeWheel`
//! for selecting both date and time.

use super::anchored_overlay::{AnchoredPlacement, use_anchored_overlay};
pub use super::calendar::{
  CalendarDate, CalendarDateTime, CalendarTime, CalendarWheelColumn, calendar_clamp_date,
  days_in_month, format_date_time, format_time, parse_date_time, parse_time,
};
use super::density::{density_control_class, use_density, with_density};
use super::element_id::next_element_id;
use super::modal_focus::use_modal_focus_scope;
pub use super::overlay::{DismissBehavior, OverlayAlign, OverlaySide, PopoverPrimitiveConfig};
use super::root_state::{Controllable, use_controllable, use_root_context};
use super::utils::{classes, merge_classes};
use dioxus::prelude::*;

const DATE_TIME_PICKER_TRIGGER_BASE_CLASS: &str = "flex h-10 w-full items-center justify-between rounded-md border bg-background px-3 py-2 text-sm text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
const DATE_TIME_PICKER_VALUE_BASE_CLASS: &str =
  "truncate text-left data-[placeholder=true]:text-muted-foreground";
const DATE_TIME_PICKER_CONTENT_BASE_CLASS: &str = "z-50 w-auto rounded-md border border-border bg-popover p-0 text-popover-foreground shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
const DATE_TIME_PICKER_INPUT_BASE_CLASS: &str = "flex h-10 w-full rounded-md border bg-background px-3 py-2 text-sm tabular-nums transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
const TIME_PICKER_BASE_CLASS: &str =
  "inline-flex items-center gap-1 rounded-md border border-border bg-background p-2 text-foreground";
const DATE_TIME_WHEEL_BASE_CLASS: &str =
  "inline-flex flex-wrap items-center gap-1 rounded-md border border-border bg-background p-2 text-foreground";

/// Classes for the date-time trigger button.
pub fn date_time_picker_trigger_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(DATE_TIME_PICKER_TRIGGER_BASE_CLASS), Some(invalid_class)]), class)
}

/// Classes for the date-time value span inside `DateTimePickerTrigger`.
pub fn date_time_picker_value_class(class: &str) -> String {
  merge_classes(classes([Some(DATE_TIME_PICKER_VALUE_BASE_CLASS)]), class)
}

/// Classes for the anchored `DateTimePickerContent` dialog.
pub fn date_time_picker_content_class(class: &str) -> String {
  merge_classes(classes([Some(DATE_TIME_PICKER_CONTENT_BASE_CLASS)]), class)
}

/// Classes for the `DateTimePickerInput` text field.
pub fn date_time_picker_input_class(invalid: bool, class: &str) -> String {
  let invalid_class = if invalid {
    "border-destructive focus-visible:ring-destructive"
  } else {
    "border-input focus-visible:ring-ring"
  };

  merge_classes(classes([Some(DATE_TIME_PICKER_INPUT_BASE_CLASS), Some(invalid_class)]), class)
}

/// Classes for the `TimePicker` wheel container.
pub fn time_picker_class(class: &str) -> String {
  merge_classes(classes([Some(TIME_PICKER_BASE_CLASS)]), class)
}

/// Classes for the `DateTimeWheel` multi-column container.
pub fn date_time_wheel_class(class: &str) -> String {
  merge_classes(classes([Some(DATE_TIME_WHEEL_BASE_CLASS)]), class)
}

fn side_attribute(side: OverlaySide) -> &'static str {
  match side {
    OverlaySide::Top => "top",
    OverlaySide::Right => "right",
    OverlaySide::Bottom => "bottom",
    OverlaySide::Left => "left",
    OverlaySide::Inline => "inline",
  }
}

fn align_attribute(align: OverlayAlign) -> &'static str {
  match align {
    OverlayAlign::Start => "start",
    OverlayAlign::Center => "center",
    OverlayAlign::End => "end",
  }
}

#[derive(Clone)]
struct DateTimePickerContext {
  trigger_id: String,
  open: Controllable<bool>,
  set_open: Callback<bool>,
}

fn use_date_time_picker(part: &str) -> DateTimePickerContext {
  use_root_context::<DateTimePickerContext>(part, "DateTimePicker")
}

/// Root of a date-time picker: owns open state and links trigger and content.
#[component]
pub fn DateTimePicker(
  #[props(default)] id: Option<String>,
  #[props(default)] open: ReadSignal<Option<bool>>,
  #[props(default)] default_open: bool,
  #[props(default)] on_open_change: Option<EventHandler<bool>>,
  children: Element,
) -> Element {
  let generated = use_hook(|| format!("dxui-date-time-picker-{}-trigger", next_element_id()));
  let trigger_id = id.unwrap_or(generated);
  let open = use_controllable(move || open.cloned(), move || default_open, on_open_change);
  let set_open = use_callback(move |next: bool| open.set(next));
  use_context_provider(|| DateTimePickerContext { trigger_id, open, set_open });

  rsx! { {children} }
}

/// Button that toggles `DateTimePickerContent` and anchors it.
#[component]
pub fn DateTimePickerTrigger(
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
  children: Element,
) -> Element {
  let context = use_date_time_picker("DateTimePickerTrigger");
  let class = date_time_picker_trigger_class(
    invalid,
    &with_density(density_control_class(use_density()), &class),
  );
  let open = context.open.get();
  let set_open = context.set_open;

  rsx! {
    button {
      r#type: "button",
      id: context.trigger_id,
      class,
      disabled,
      onclick: move |_| set_open.call(!open),
      "aria-expanded": open.to_string(),
      "aria-haspopup": "dialog",
      "aria-invalid": invalid.to_string(),
      "data-state": if open { "open" } else { "closed" },
      ..attributes,
      {children}
    }
  }
}

/// Displays the selected date-time text or `placeholder` inside `DateTimePickerTrigger`.
#[component]
pub fn DateTimePickerValue(
  #[props(default)] placeholder: String,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let class = date_time_picker_value_class(&class);
  let has_placeholder = !placeholder.is_empty();

  rsx! {
    span {
      class,
      "data-placeholder": has_placeholder.to_string(),
      "aria-label": placeholder,
      {children}
    }
  }
}

/// Anchored dialog popover that holds the date-time picker body.
#[component]
pub fn DateTimePickerContent(
  #[props(default = OverlaySide::Bottom)] side: OverlaySide,
  #[props(default = OverlayAlign::Center)] align: OverlayAlign,
  #[props(default = 4)] side_offset: i32,
  #[props(default = DismissBehavior::popover_default())] dismiss: DismissBehavior,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  let context = use_date_time_picker("DateTimePickerContent");
  let class = date_time_picker_content_class(&class);
  let open = context.open.get();
  let anchor_id = Some(context.trigger_id.clone());
  let focus_scope = use_modal_focus_scope(open, false);
  let anchored = use_anchored_overlay(
    open,
    AnchoredPlacement { anchor_id, anchor_point: None, side, align, side_offset },
    dismiss,
    Some(context.set_open),
  );

  rsx! {
    div {
      role: "dialog",
      class,
      tabindex: "-1",
      "aria-labelledby": context.trigger_id,
      hidden: !open,
      "data-dxui-anchored": anchored,
      "data-dxui-focus-scope": focus_scope,
      "data-align": align_attribute(align),
      "data-side": side_attribute(side),
      "data-state": if open { "open" } else { "closed" },
      {children}
    }
  }
}

/// Controlled text input for typing `YYYY-MM-DD HH:MM` (or `YYYY-MM-DD HH:MM:SS`).
#[component]
pub fn DateTimePickerInput(
  #[props(default)] value: Option<CalendarDateTime>,
  #[props(default)] include_seconds: bool,
  #[props(default)] placeholder: String,
  #[props(default)] invalid: bool,
  #[props(default)] disabled: bool,
  #[props(default)] class: String,
  #[props(default)] on_value_change: Option<EventHandler<Option<CalendarDateTime>>>,
  #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
  let mut text =
    use_signal(|| value.map(|dt| format_date_time(dt, include_seconds)).unwrap_or_default());

  use_effect(use_reactive((&value, &include_seconds), move |(value, include_seconds)| {
    let current = parse_date_time(&text.peek());
    if current != value {
      text.set(value.map(|dt| format_date_time(dt, include_seconds)).unwrap_or_default());
    }
  }));

  let typed = text();
  let unparsed = !typed.trim().is_empty() && parse_date_time(&typed).is_none();
  let class = date_time_picker_input_class(
    invalid || unparsed,
    &with_density(density_control_class(use_density()), &class),
  );
  let placeholder = if placeholder.is_empty() {
    if include_seconds {
      "YYYY-MM-DD HH:MM:SS".to_string()
    } else {
      "YYYY-MM-DD HH:MM".to_string()
    }
  } else {
    placeholder
  };

  rsx! {
    input {
      class,
      r#type: "text",
      inputmode: "numeric",
      autocomplete: "off",
      placeholder,
      disabled,
      value: typed,
      "aria-invalid": (invalid || unparsed).then_some("true"),
      oninput: move |event| {
        let next = event.value();
        let parsed = parse_date_time(&next);
        let cleared = next.trim().is_empty();
        text.set(next);
        if let Some(handler) = on_value_change.filter(|_| parsed.is_some() || cleared) {
          handler.call(parsed);
        }
      },
      onblur: move |_| {
        let parsed = parse_date_time(&text.peek());
        if let Some(dt) = parsed {
          text.set(format_date_time(dt, include_seconds));
        }
      },
      ..attributes,
    }
  }
}

/// Multi-column Hour / Minute (/ Second) scroll-wheel time picker.
#[component]
pub fn TimePicker(
  value: CalendarTime,
  #[props(default)] include_seconds: bool,
  #[props(default = 1)] minute_step: u8,
  #[props(default = "Hour".to_string())] hour_label: String,
  #[props(default = "Minute".to_string())] minute_label: String,
  #[props(default = "Second".to_string())] second_label: String,
  #[props(default)] on_value_change: Option<EventHandler<CalendarTime>>,
  #[props(default)] class: String,
) -> Element {
  let step = minute_step.clamp(1, 30) as usize;
  let hours: Vec<String> = (0..24).map(|h| format!("{h:02}")).collect();
  let minute_values: Vec<u8> = (0..60).step_by(step).collect();
  let minutes: Vec<String> = minute_values.iter().map(|m| format!("{m:02}")).collect();
  let seconds: Vec<String> = (0..60).map(|s| format!("{s:02}")).collect();

  let clamped_hour = value.hour.min(23);
  let minute_index = minute_values
    .iter()
    .position(|&m| m >= value.minute.min(59))
    .unwrap_or_else(|| minute_values.len().saturating_sub(1));
  let clamped_minute = minute_values.get(minute_index).copied().unwrap_or(0);
  let clamped_second = value.second.min(59);
  let class = time_picker_class(&class);

  let emit_time = move |hour: u8, minute: u8, second: u8| {
    if let Some(handler) = on_value_change {
      handler.call(CalendarTime::unchecked(hour.min(23), minute.min(59), second.min(59)));
    }
  };

  rsx! {
    div {
      class,
      "data-time-picker": "",
      CalendarWheelColumn {
        label: hour_label,
        items: hours,
        selected_index: clamped_hour as usize,
        class: "w-14".to_string(),
        on_select: move |idx| emit_time(idx as u8, clamped_minute, clamped_second),
      }
      span { class: "text-sm font-semibold text-muted-foreground", "aria-hidden": "true", ":" }
      CalendarWheelColumn {
        label: minute_label,
        items: minutes,
        selected_index: minute_index,
        class: "w-14".to_string(),
        on_select: move |idx| {
          let next_minute = minute_values.get(idx).copied().unwrap_or(0);
          emit_time(clamped_hour, next_minute, clamped_second);
        },
      }
      if include_seconds {
        span { class: "text-sm font-semibold text-muted-foreground", "aria-hidden": "true", ":" }
        CalendarWheelColumn {
          label: second_label,
          items: seconds,
          selected_index: clamped_second as usize,
          class: "w-14".to_string(),
          on_select: move |idx| emit_time(clamped_hour, clamped_minute, idx as u8),
        }
      }
    }
  }
}

/// Full scroll-wheel picker combining Year / Month / Day and Hour / Minute (/ Second).
#[component]
pub fn DateTimeWheel(
  value: CalendarDateTime,
  #[props(default = 1950)] min_year: i32,
  #[props(default = 2050)] max_year: i32,
  #[props(default)] include_seconds: bool,
  #[props(default = 1)] minute_step: u8,
  #[props(default = "Year".to_string())] year_label: String,
  #[props(default = "Month".to_string())] month_label: String,
  #[props(default = "Day".to_string())] day_label: String,
  #[props(default = "Hour".to_string())] hour_label: String,
  #[props(default = "Minute".to_string())] minute_label: String,
  #[props(default = "Second".to_string())] second_label: String,
  #[props(default)] on_value_change: Option<EventHandler<CalendarDateTime>>,
  #[props(default)] class: String,
) -> Element {
  let max_year = max_year.max(min_year);
  let clamped_year = value.date.year.clamp(min_year, max_year);
  let clamped_month = value.date.month.clamp(1, 12);
  let max_day = days_in_month(clamped_year, clamped_month);
  let clamped_day = value.date.day.clamp(1, max_day);

  let step = minute_step.clamp(1, 30) as usize;
  let years: Vec<String> = (min_year..=max_year).map(|y| format!("{y:04}")).collect();
  let months: Vec<String> = (1..=12).map(|m| format!("{m:02}")).collect();
  let days: Vec<String> = (1..=max_day).map(|d| format!("{d:02}")).collect();
  let hours: Vec<String> = (0..24).map(|h| format!("{h:02}")).collect();
  let minute_values: Vec<u8> = (0..60).step_by(step).collect();
  let minutes: Vec<String> = minute_values.iter().map(|m| format!("{m:02}")).collect();
  let seconds: Vec<String> = (0..60).map(|s| format!("{s:02}")).collect();

  let year_index = (clamped_year - min_year) as usize;
  let month_index = (clamped_month - 1) as usize;
  let day_index = (clamped_day - 1) as usize;
  let clamped_hour = value.time.hour.min(23);
  let minute_index = minute_values
    .iter()
    .position(|&m| m >= value.time.minute.min(59))
    .unwrap_or_else(|| minute_values.len().saturating_sub(1));
  let clamped_minute = minute_values.get(minute_index).copied().unwrap_or(0);
  let clamped_second = value.time.second.min(59);
  let class = date_time_wheel_class(&class);

  let emit_dt = move |year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8| {
    if let Some(handler) = on_value_change {
      handler.call(CalendarDateTime::new(
        calendar_clamp_date(year, month, day),
        CalendarTime::unchecked(hour.min(23), minute.min(59), second.min(59)),
      ));
    }
  };

  rsx! {
    div {
      class,
      "data-date-time-wheel": "",
      CalendarWheelColumn {
        label: year_label,
        items: years,
        selected_index: year_index,
        class: "w-16".to_string(),
        on_select: move |idx| {
          emit_dt(
            min_year + idx as i32,
            clamped_month,
            clamped_day,
            clamped_hour,
            clamped_minute,
            clamped_second,
          )
        },
      }
      CalendarWheelColumn {
        label: month_label,
        items: months,
        selected_index: month_index,
        class: "w-12".to_string(),
        on_select: move |idx| {
          emit_dt(
            clamped_year,
            (idx + 1) as u8,
            clamped_day,
            clamped_hour,
            clamped_minute,
            clamped_second,
          )
        },
      }
      CalendarWheelColumn {
        label: day_label,
        items: days,
        selected_index: day_index,
        class: "w-12".to_string(),
        on_select: move |idx| {
          emit_dt(
            clamped_year,
            clamped_month,
            (idx + 1) as u8,
            clamped_hour,
            clamped_minute,
            clamped_second,
          )
        },
      }
      div { class: "mx-1 h-28 w-px bg-border", "aria-hidden": "true" }
      CalendarWheelColumn {
        label: hour_label,
        items: hours,
        selected_index: clamped_hour as usize,
        class: "w-12".to_string(),
        on_select: move |idx| {
          emit_dt(
            clamped_year,
            clamped_month,
            clamped_day,
            idx as u8,
            clamped_minute,
            clamped_second,
          )
        },
      }
      span { class: "text-sm font-semibold text-muted-foreground", "aria-hidden": "true", ":" }
      CalendarWheelColumn {
        label: minute_label,
        items: minutes,
        selected_index: minute_index,
        class: "w-12".to_string(),
        on_select: move |idx| {
          let next_minute = minute_values.get(idx).copied().unwrap_or(0);
          emit_dt(
            clamped_year,
            clamped_month,
            clamped_day,
            clamped_hour,
            next_minute,
            clamped_second,
          );
        },
      }
      if include_seconds {
        span { class: "text-sm font-semibold text-muted-foreground", "aria-hidden": "true", ":" }
        CalendarWheelColumn {
          label: second_label,
          items: seconds,
          selected_index: clamped_second as usize,
          class: "w-12".to_string(),
          on_select: move |idx| {
            emit_dt(
              clamped_year,
              clamped_month,
              clamped_day,
              clamped_hour,
              clamped_minute,
              idx as u8,
            )
          },
        }
      }
    }
  }
}
