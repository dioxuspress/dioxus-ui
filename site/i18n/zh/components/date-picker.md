## API 概览

- `DatePicker { id, open, default_open, on_open_change, children }`
- `DatePickerTrigger { invalid, disabled, class, children }`
- `DatePickerValue { placeholder, class, children }`
- `DatePickerContent { side, align, side_offset, dismiss, class, children }`
- `DatePickerInput { value, order, placeholder, invalid, disabled, on_value_change, class }`
- `DateOrder`, `parse_date(text, order)`, `format_date(date, order)`
- `DatePickerPrimitiveConfig`
- `DatePickerDismissBehavior`, `DatePickerSide`, `DatePickerAlign`

样式类名辅助函数：

- `date_picker_trigger_class(invalid, class)`
- `date_picker_value_class(class)`
- `date_picker_content_class(class)`
- `date_picker_input_class(invalid, class)`

## 行为与交互

`DatePicker` 内部维护日历浮层的展开状态，并将位于其内部的触发器与浮层内容关联起来（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）；其 `id` 用于命名触发器，以便与 `Label` 关联。选中日期、聚焦日期和可见月份均由应用持有，应用基于这些状态构建 `Calendar`，并为其启用键盘导航管理（参见 [Calendar](calendar.md)）。选中日期后自动收起浮层同样由应用逻辑决定，因此可通过受控属性 `open` 进行管理：

```rust
let mut open = use_signal(|| false);

rsx! {
  DatePicker { id: "due-date-trigger", open: open(), on_open_change: move |next| open.set(next),
    DatePickerTrigger { DatePickerValue { "{label}" } }
    DatePickerContent {
      Calendar {
        // CalendarDay { focused, on_key_move, on_select: move |date| {
        //   selected.set(Some(date));
        //   open.set(false);
        // }, ... }
      }
    }
  }
}
```

- 点击触发器会切换日历浮层的展开与收起。
- 浮层内容紧靠触发器的指定 `side`（默认为 `Bottom`）、`align`（默认为 `Center`）和 `side_offset`（默认为 `4`）定位，并像 `Popover` 一样支持边界翻转与偏移避让。
- 展开时会将焦点移至 `Calendar` 当前聚焦的日期（带有 `data-dxui-autofocus` 标记）或第一个可聚焦元素。`Tab` 和 `Shift+Tab` 会在浮层内容内部循环包裹。
- 按 `Escape` 及外部交互会根据 `dismiss` 配置（默认为 `DismissBehavior::popover_default()`）关闭浮层。关闭时焦点会回到触发器，除非外部点击已将焦点转移到了其他控件上。
- 选择日期由应用代码处理：在 `on_select` 中保存日期并关闭选择器。

Web 渲染器的交互行为已由 `npm run verify:runtime-interactions` 覆盖，Desktop 渲染器由 `npm run verify:desktop-interactions` 覆盖，iOS 模拟器由 `npm run verify:mobile-interactions` 覆盖，Android 模拟器由 `npm run verify:android-interactions` 覆盖。

### 手动输入日期

`DatePickerInput` 是用于手动输入日期的文本框，通过 `value: Option<CalendarDate>` 受控管理。可将其与日历共享同一个值，并在旁边放置一个带有 `aria-label` 的图标按钮作为触发器：

```rust
DatePickerInput {
  "aria-label": "Start date",
  order: DateOrder::MonthDayYear,
  value: selected(),
  on_value_change: move |date| selected.set(date),
}
```

`parse_date(text, order)` 会解析由 `-`、`/`、`.` 或空格分隔的三组数字：若第一组数字为四位数，则无论 `order` 如何均按 ISO 年-月-日格式解析；否则由 `DateOrder`（`YearMonthDay`、`MonthDayYear` 或 `DayMonthYear`）决定字段顺序。年份必须为四位数，且不存在的日期（如 2 月 30 日）会被拒绝。输入框在用户输入过程中保留原始文本，一旦文本成功解析为合法日期则携带该日期调用 `on_value_change`（清空文本时传入 `None`），对无法解析的文本标记 `aria-invalid` 但不清空输入内容，并在失焦（blur）时通过 `format_date` 将有效日期格式化重写。占位符默认采用对应 `order` 的格式模式（参见 [RFC 0064](../../../../docs/rfcs/0064-typed-date-input.md)）。

## 无障碍说明

触发器使用带有 `aria-haspopup="dialog"` 以及展开/校验失败状态的按钮语义。浮层内容使用对话框（dialog）语义，支持焦点进入、内部 `Tab` 焦点锁定与焦点归还，并通过 `aria-labelledby` 继承触发器的名称（参见 [RFC 0055](../../../../docs/rfcs/0055-open-state-accessibility-audit.md)）。文本日期解析逻辑仍由应用按需管理。

当平台原生移动端体验是首要诉求时，请优先使用原生日期输入框；当需要常驻展示的日期网格时，请直接使用 `Calendar`；当需要从紧凑表单控件弹出可视化日历面板时，请使用 `DatePicker`。
