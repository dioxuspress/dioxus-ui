## API 概览

- `Calendar { class, children }`
- `CalendarHeader { class, children }`
- `CalendarCaption { class, children }`
- `CalendarNav { class, children }`
- `CalendarNavButton { direction, disabled, onclick, class, children }`
- `CalendarGrid { class, children }`
- `CalendarHead { class, children }`
- `CalendarHeadCell { class, children }`
- `CalendarBody { class, children }`
- `CalendarRow { class, children }`
- `CalendarDay { date, selected, today, outside_month, disabled, range_state, focused, on_key_move, on_select, class, children }`
- `CalendarSelect { value, disabled, onchange, class, children }`（放在 `CalendarHeader` 内的紧凑型原生 `<select>`，用于年份与月份快速跳转）
- `CalendarWheelColumn { label, items, selected_index, on_select, class }`（支持鼠标滚轮、触摸/拖拽滑动、点击与键盘上下键的单列滚轮）
- `CalendarWheel { value, min_year, max_year, year_label, month_label, day_label, on_value_change, class }`（年/月/日三列联动滚轮选择器，自动按闰年与大小月裁剪天数）

属性与辅助函数使用的类型：

- `CalendarNavDirection`（`CalendarNavButton` 的 `direction` 属性类型）
- `CalendarKeyMove`（`on_key_move` 回调载荷）
- `CalendarMonth`, `CalendarWeekday`, `CalendarMonthGrid`, `CalendarPrimitiveDay`
- `CalendarTime`, `CalendarDateTime`

原语辅助函数：

- `calendar_month_grid(...)`
- `calendar_move_date(...)`
- `calendar_clamp_date(year, month, day)`
- `calendar_key_move(key, shift)`
- `calendar_range_state(...)`
- `days_in_month(year, month)`
- `is_leap_year(year)`

## 键盘交互

当前可见月份、聚焦日期和选中日期均由应用受控管理。为每个日期单元格传入 `on_key_move` 即可启用网格的键盘导航管理：

```rust
let mut month = use_signal(|| CalendarMonth::unchecked(2026, 10));
let mut focused = use_signal(|| CalendarDate::unchecked(2026, 10, 15));
let mut selected = use_signal(|| None::<CalendarDate>);
let mut move_focus = move |date: CalendarDate| {
  focused.set(date);
  month.set(CalendarMonth::unchecked(date.year, date.month));
};

// Inside the week loop over `calendar_month_grid(month(), ...)`:
rsx! {
  CalendarDay {
    date: day.date,
    selected: day.selected,
    outside_month: day.outside_month,
    focused: day.date == focused(),
    on_key_move: move |key_move| {
      move_focus(calendar_move_date(focused(), key_move, CalendarWeekday::Sunday));
    },
    on_select: move |date| {
      selected.set(Some(date));
      move_focus(date);
    },
    "{day.date.day}"
  }
}
```

- 仅有 `focused` 为真的日期单元格拥有 `tabindex="0"`，因此整个日历网格只占一个 `Tab` 停靠点。
- `ArrowLeft` 和 `ArrowRight` 按天移动，`ArrowUp` 和 `ArrowDown` 按周移动，`Page Up` 和 `Page Down` 按月移动（配合 `Shift` 按年移动），`Home` 和 `End` 跳转到当周的起始日和结束日。`calendar_key_move` 公开了这一按键映射逻辑。
- 按下导航键后，新变为 `focused` 的日期单元格会自动获取 DOM 焦点——无论 Dioxus 是复用了原有元素还是挂载了新元素（按日期设置 `key`）。在没有按键操作的情况下由应用代码直接修改 `focused` 不会移动 DOM 焦点，从而确保日历在渲染时绝不会抢占焦点。
- 点击、`Enter` 和 `Space` 会携带当天日期调用 `on_select`。
- `CalendarNavButton` 接收 `onclick` 以处理月份切换导航。
- 源码拷贝（source-copy）模板包含 `CalendarKeyMove` 与 `calendar_key_move`，但不内置日期运算逻辑；自行构建网格的应用可使用自身的日期库应用移动偏移。
- 移动到禁用日期上会导致目标变为无法接收焦点的禁用按钮；若有需要，请在应用逻辑中跳过禁用日期。

## 无障碍说明

`Calendar` 暴露了 `grid`、`row`、`columnheader` 和 `gridcell` 角色。选中与禁用状态分别渲染 `aria-selected` 和 `aria-disabled`；今日、非当月日期以及区间状态仅渲染 `data-today`、`data-outside-month` 和 `data-range`。启用键盘管理的日期单元格采用漫游 `tabindex`（roving tabindex），并使 DOM 焦点跟随聚焦日期同步移动。左右方向键遵循视觉上的左右方向，暂未实现从右到左（RTL）镜像翻转。

`CalendarGrid` 和 `CalendarCaption` 均支持属性透传，请为标题设置 `id` 并将网格的 `aria-labelledby` 指向该 `id`（参见 [RFC 0040](../../../../docs/rfcs/0040-composite-widget-names.md)）。
