## API 概览

- `Swap`
- `SwapEffect`
- `swap_class`
- `swap_layer_class`

```rust
let mut open = use_signal(|| false);

rsx! {
  Swap {
    "aria-label": "Menu",
    effect: SwapEffect::Rotate,
    active: open(),
    on_active_change: move |active| open.set(active),
    on: rsx! { CloseIcon {} },
    off: rsx! { MenuIcon {} },
  }
}
```

处于 `active` 状态时显示 `on`，否则显示 `off`。按下按钮会携带新状态调用 `on_active_change`。`SwapEffect` 支持 `Fade`（默认）、`Rotate` 或 `Flip`。可通过 `class` 自定义按钮的尺寸与样式。

## 无障碍说明

`Swap` 是带有 `aria-pressed` 的原生按钮，因此辅助技术会将其播报为切换按钮。隐藏层带有 `aria-hidden`；当两个图层均为图标时，请使用 `aria-label` 为按钮命名。在开启 `prefers-reduced-motion` 时过渡动画会自动停用（参见 [RFC 0060](../../../../docs/rfcs/0060-input-components.md)）。
