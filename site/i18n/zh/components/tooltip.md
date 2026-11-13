## API 概览

- `Tooltip`
- `TooltipTrigger`
- `TooltipContent`
- `TooltipPrimitiveConfig`
- `TooltipDismissBehavior`, `TooltipSide`, `TooltipAlign`
- `tooltip_content_class`

## 行为与交互

`Tooltip` 持有工具提示是否打开的状态，内部各子部件均读取该状态，因此必须放置在 `Tooltip` 内部（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）：

```rust
rsx! {
  Tooltip {
    TooltipTrigger { "Save" }
    TooltipContent { "Saved 2 minutes ago" }
  }
}
```

可传入 `open` 与 `on_open_change` 进行受控管理，或传入 `default_open` 默认打开；两种模式下 `on_open_change` 都会接收每一次状态变化。

- 指针悬停在触发器上时，会在等待 `delay_ms`（默认 `700` 毫秒）后打开提示。键盘聚焦触发器时会立即打开。
- 当指针从触发器移入提示内容区域时，提示保持打开；当指针同时离开两者后，会在 100 毫秒宽限期结束后关闭。
- 触发器失焦（blur）时会关闭提示，除非此时指针正悬停在触发器或提示内容上方（此时改由指针移出时关闭）。点击触发器会关闭提示，且在指针离开触发器之前持续悬停不会重新打开提示。
- 触控指针（touch）不会触发打开工具提示。
- 打开期间，触发器带有指向提示内容的 `aria-describedby`。
- 内容容器锚定到 `TooltipTrigger`。
- 内容采用固定定位放置在锚点旁，首选方向空间不足时自动翻转至对侧，偏移以保持在视口内，并跟随窗口缩放与滚动。`side_offset` 默认为 `4` 像素。
- `side` 默认为 `Top`，`align` 默认为 `Center`。
- 按下 `Escape` 会根据 `dismiss`（默认 `DismissBehavior::tooltip_default()`）关闭提示；外部指针按下不会触发该关闭策略。
- `TooltipTrigger` 接收 `onclick` 执行业务操作，并透传其他按钮属性（例如为纯图标按钮提供 `aria-label`）。

Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试。

## 无障碍说明

工具提示在指针悬停和键盘聚焦时打开。切勿将关键的可交互内容放入工具提示中。相邻提示间的免延迟切换、触控长按以及非按钮类型的触发器均未实现（参见 [RFC 0022](../../../../docs/rfcs/0022-tooltip-hover-and-focus-opening.md)）。`Tooltip` 与 `HoverCard` 共享同一套悬停打开脚本（[RFC 0023](../../../../docs/rfcs/0023-hover-card-hover-and-focus-opening.md)）。
