## API 概览

- `NumberInput`
- `number_input_class`

```rust
let mut quantity = use_signal(|| 1.0);

rsx! {
  NumberInput {
    "aria-label": "Quantity",
    min: 1.0,
    max: 99.0,
    value: quantity(),
    on_value_change: move |value| quantity.set(value),
  }
}
```

`value` 和 `step`（默认为 `1`）均为 `f64` 类型；`min` 和 `max` 为可选的上下界。输入框在用户输入过程中保留正在键入的原始文本（因此 `"-"` 或 `"1."` 在重渲染时不会被清空），并在每次构成完整数值时调用 `on_value_change`。失焦（blur）时，组件会将数值钳制在上下界范围内，并按 `step` 的小数位数四舍五入（因此 `0.1 + 0.2` 会显示为 `0.3`）。到达上下界时对应的增减按钮会自动禁用。`decrement_label` 和 `increment_label` 用于为增减按钮命名，默认值分别为 `"Decrease"` 和 `"Increase"`。`id`、`name` 等其他属性会透传至底层输入框。

## 无障碍说明

底层输入框是一个带有 `role="spinbutton"`、`inputmode="decimal"` 以及 `aria-valuenow`、`aria-valuemin` 和 `aria-valuemax` 的文本字段。`ArrowUp` 和 `ArrowDown` 按步长增减，`Home` 和 `End` 直接跳转到最小或最大边界。由于方向键已提供相同操作，两侧按钮设置了 `tabindex="-1"`，但仍保留无障碍名称以供触屏屏幕阅读器使用。组件刻意未使用原生的 `type="number"` 输入框，因为原生数字输入框在文本尚未输入完整时会返回空值（参见 [RFC 0060](../../../../docs/rfcs/0060-input-components.md)）。
