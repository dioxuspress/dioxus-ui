## API 概览

- `Countdown`
- `CountdownParts`
- `countdown_parts`
- `countdown_class`

```rust
rsx! {
  Countdown { remaining: remaining(), "aria-label": "Sale ends in" }
}
```

`remaining` 以秒为单位；达到一天及以上时会显示天数。时钟由应用自身维护并每秒传入新值（例如通过 `use_future` 循环驱动），因为 Dioxus 在不引入平台相关 crate 的情况下没有跨所有渲染器通用的定时器。`countdown_parts` 可将秒数拆分为天、时、分、秒，便于应用按自身语言定制带单位标签的布局。

## 无障碍说明

Countdown 使用 `role="timer"`，该角色不属于实时播报区域（live region），因此屏幕阅读器不会每秒重复播报；用户仅在焦点到达该组件时听到当前剩余时间。请使用 `aria-label` 为其提供无障碍名称（参见 [RFC 0059](../../../../docs/rfcs/0059-display-components.md)）。
