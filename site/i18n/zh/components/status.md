## API 概览

- `Status`
- `StatusVariant`
- `StatusSize`
- `status_class`

```rust
rsx! {
  Status { variant: StatusVariant::Success, label: "Online" }
  span { class: "flex items-center gap-2", Status { variant: StatusVariant::Warning }, "Degraded" }
}
```

`StatusVariant` 支持 `Neutral`（默认）、`Primary`、`Success`、`Warning`、`Info` 和 `Destructive`；`StatusSize` 支持 `Sm`、`Md`（默认）和 `Lg`。

## 无障碍说明

传入 `label` 时，Status 渲染为带有对应 `aria-label` 的 `role="img"`。未传 `label` 时则渲染为 `aria-hidden`，适用于旁边已有文本说明状态的圆点。请勿仅靠颜色传达状态：要么为圆点提供标签，要么在相邻文本中写明状态（参见 [RFC 0059](../../../../docs/rfcs/0059-display-components.md)）。
