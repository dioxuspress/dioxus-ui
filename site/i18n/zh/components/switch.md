## API 概览

- `Switch`
- `switch_class`
- `switch_thumb_class`

## 变更事件

```rust
let mut wifi = use_signal(|| false);

rsx! {
  Label { r#for: "wifi", "Wi-Fi" }
  Switch {
    id: "wifi",
    checked: wifi(),
    on_checked_change: move |checked| wifi.set(checked),
  }
}
```

`Switch` 是受控组件。点击开关、按下 `Space` 或 `Enter`，或点击其关联的 `Label`，都会以目标状态 `!checked` 调用 `on_checked_change`，由应用将其回传给 `checked`。禁用的开关不会触发该回调。

其他属性（如 `id`、`name`、`aria-label` 和 `aria-describedby`）会透传至底层按钮。按钮及其滑块圆点（thumb）会渲染 `data-state="checked"` 或 `"unchecked"`，以支持 `data-[state=checked]:` 样式变体。

## 无障碍说明

对于立即生效的设置项，请使用 `Switch` 开关语义；若值需要作为表单的一部分提交，请改用复选框（Checkbox）语义。

请为每个开关提供无障碍名称：可以使用 `for` 匹配开关 `id` 的 `Label`，或直接传入 `aria-label`。由于开关底层是按钮，不会随原生表单自动提交；如有需要，请根据同一状态额外渲染一个隐藏的 `input`。
