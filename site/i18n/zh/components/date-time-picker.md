## API 概览

- `DateTimePicker { id, open, default_open, on_open_change, children }`
- `DateTimePickerTrigger { invalid, disabled, class, children }`
- `DateTimePickerValue { placeholder, class, children }`
- `DateTimePickerContent { side, align, side_offset, dismiss, class, children }`
- `DateTimePickerInput { value, include_seconds, placeholder, invalid, disabled, on_value_change, class }`
- `TimePicker { value, include_seconds, minute_step, hour_label, minute_label, second_label, on_value_change, class }`
- `DateTimeWheel { value, min_year, max_year, include_seconds, minute_step, year_label, month_label, day_label, hour_label, minute_label, second_label, on_value_change, class }`
- `CalendarDateTime`, `CalendarTime`, `format_date_time(dt, include_seconds)`, `parse_date_time(text)`, `format_time(t, include_seconds)`, `parse_time(text)`

样式辅助函数：

- `date_time_picker_trigger_class(invalid, class)`
- `date_time_picker_value_class(class)`
- `date_time_picker_content_class(class)`
- `date_time_picker_input_class(invalid, class)`
- `time_picker_class(class)`
- `date_time_wheel_class(class)`

## 行为与交互

`DateTimePicker` 负责管理浮层开关状态并关联触发器与内容区，实际的 `CalendarDateTime`（或 `CalendarTime`）值由应用受控管理。开发者可根据业务场景选用三种形态：

1. **桌面日历 + 时间滚轮组合版**：在 `DateTimePickerContent` 中将 `Calendar`（可搭配 `CalendarSelect` 年/月下拉快选）与 `TimePicker` 并排组合。
2. **全滚轮版（`DateTimeWheel`）**：年 / 月 / 日 / 时 / 分（/ 秒）多列滚轮联动选择，切换年份或月份时自动按闰年与大小月天数裁剪日期。
3. **键盘输入 + 浮层联动版（`DateTimePickerInput`）**：支持直接输入 `YYYY-MM-DD HH:MM[:SS]` 或 `YYYY-MM-DDTHH:MM[:SS]`，非法文本自动标记 `aria-invalid`，并与浮层选择双向同步。

每个滚轮列（`CalendarWheelColumn`）均支持鼠标滚轮滚动、触摸/指针滑动、点击选项以及 `ArrowUp` / `ArrowDown` / `PageUp` / `PageDown` / `Home` / `End` 键盘步进。

## 无障碍说明

`DateTimePickerTrigger` 暴露 `aria-expanded` 与 `aria-haspopup="dialog"` 属性。`DateTimePickerContent` 渲染 `role="dialog"`，通过 `aria-labelledby` 关联触发器，并在打开时捕获 `Tab` 焦点循环。每个滚轮列均暴露 `role="listbox"` 与 `aria-label`，内部选项暴露 `role="option"` 与 `aria-selected`。
