## API 概览

- `Toggle`
- `ToggleSize`
- `ToggleVariant`
- `toggle_class`

## 变更事件

```rust
let mut bold = use_signal(|| false);

rsx! {
  Toggle {
    "aria-label": "Bold",
    pressed: bold(),
    on_pressed_change: move |pressed| bold.set(pressed),
    "B"
  }
}
```

`Toggle` 为受控组件。点击、`Enter` 或 `Space` 会以目标状态 `!pressed` 调用 `on_pressed_change`，再由应用将其回传给 `pressed`。禁用状态下的 `Toggle` 不会触发该回调。其他属性（例如 `aria-label`）会透传至底层按钮元素。

## 无障碍说明

`Toggle` 渲染带有 `aria-pressed` 的按钮。请将其用于按下状态本身即代表开关状态的二元命令，而不要用于视图之间的导航切换。
