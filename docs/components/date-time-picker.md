# Date Time Picker

Date Time Picker provides controlled trigger, value, input, and anchored dialog
parts for selecting both date and time, along with a standalone `TimePicker`
and a full-wheel `DateTimeWheel`.

## Source Copy

```bash
dxui add date-time-picker
```

This creates Date Time Picker and its Calendar dependency:

```text
src/components/ui/date_time_picker.rs
src/components/ui/calendar.rs
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["date-time-picker", "calendar"] }
```

```rust
use dioxus_shadcn::{
  CalendarDateTime, CalendarTime, DateTimePicker, DateTimePickerContent, DateTimePickerInput,
  DateTimePickerTrigger, DateTimePickerValue, DateTimeWheel, TimePicker,
};
```

## API Surface

- `DateTimePicker { id, open, default_open, on_open_change, children }`
- `DateTimePickerTrigger { invalid, disabled, class, children }`
- `DateTimePickerValue { placeholder, class, children }`
- `DateTimePickerContent { side, align, side_offset, dismiss, class, children }`
- `DateTimePickerInput { value, include_seconds, placeholder, invalid, disabled, on_value_change, class }`
- `TimePicker { value, include_seconds, minute_step, hour_label, minute_label, second_label, on_value_change, class }`
- `DateTimeWheel { value, min_year, max_year, include_seconds, minute_step, year_label, month_label, day_label, hour_label, minute_label, second_label, on_value_change, class }`
- `CalendarDateTime`, `CalendarTime`, `format_date_time(dt, include_seconds)`, `parse_date_time(text)`, `format_time(t, include_seconds)`, `parse_time(text)`

Class helpers:

- `date_time_picker_trigger_class(invalid, class)`
- `date_time_picker_value_class(class)`
- `date_time_picker_content_class(class)`
- `date_time_picker_input_class(invalid, class)`
- `time_picker_class(class)`
- `date_time_wheel_class(class)`

## Behavior

`DateTimePicker` owns the open state and links the trigger and content, while
your app owns the `CalendarDateTime` (or `CalendarTime`) value. Developers can
choose among three patterns depending on the layout:

1. **Calendar + Time Columns**: compose `Calendar` (optionally with
   `CalendarSelect` year/month dropdowns) beside `TimePicker` inside
   `DateTimePickerContent`.
2. **Full Wheel (`DateTimeWheel`)**: Year / Month / Day / Hour / Minute (/ Second)
   scroll-wheel columns that automatically clamp days for leap years and short
   months.
3. **Typed Input + Popover (`DateTimePickerInput`)**: parses
   `YYYY-MM-DD HH:MM[:SS]` or `YYYY-MM-DDTHH:MM[:SS]`, marks invalid input via
   `aria-invalid`, and syncs bidirectionally with the popover picker.

Each wheel column (`CalendarWheelColumn`) responds to mouse wheel scrolling,
pointer swipe, option click, and `ArrowUp` / `ArrowDown` / `PageUp` / `PageDown`
/ `Home` / `End` keys.

## Accessibility Notes

`DateTimePickerTrigger` carries `aria-expanded` and `aria-haspopup="dialog"`.
`DateTimePickerContent` renders `role="dialog"` with `aria-labelledby` pointing
to the trigger and traps Tab navigation while open. Each wheel column exposes
`role="listbox"` with `aria-label` and `role="option"` items carrying
`aria-selected`.
