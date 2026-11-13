## API 概览

- `Rating`
- `rating_class`

```rust
let mut rating = use_signal(|| 0_u8);

rsx! {
  Rating {
    "aria-label": "Product rating",
    value: rating(),
    on_value_change: move |value| rating.set(value),
  }
}
```

`value` 的取值范围为 `0`（未评分）到 `max`（默认为 `5`）；小于等于该值的星标会以警告色填充。评分变更时会以选中的星级调用 `on_value_change`。`name` 用于设置单选组的表单字段名；未提供时，每个 `Rating` 会自动获得一个唯一名称。`disabled` 会禁用所有星标。

## 无障碍说明

`Rating` 是由原生单选输入框组成的 `role="radiogroup"`，因此可通过 `Tab` 聚焦、通过方向键在星标之间移动，且其值可随表单一并提交。每颗星标命名为 `"{n} of {max}"`；请使用 `aria-label` 或 `aria-labelledby` 为单选组命名。获得焦点的星标周围会显示焦点环（参见 [RFC 0060](../../../../docs/rfcs/0060-input-components.md)）。
