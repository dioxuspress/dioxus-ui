## API 概览

- `HoverCard`
- `HoverCardTrigger`
- `HoverCardContent`
- `HoverCardHeader`
- `HoverCardTitle`
- `HoverCardDescription`
- `HoverCardSide`
- `HoverCardAlign`
- `HoverCardPrimitiveConfig`
- `hover_card_content_class`

该模块还重导出了 `OverlaySide`、`OverlayAlign` 与 `PopoverPrimitiveConfig`，方便直接从 `dioxus_shadcn::hover_card` 导入使用。

## 行为与交互

`HoverCard` 持有悬浮卡片是否打开的状态，内部各子部件均读取该状态，因此必须放置在 `HoverCard` 内部（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）：

```rust
rsx! {
  HoverCard {
    HoverCardTrigger { href: "/users/dioxus", "@dioxus" }
    HoverCardContent { "Fullstack app framework for Rust." }
  }
}
```

可传入 `open` 与 `on_open_change` 进行受控管理，或传入 `default_open` 默认打开；两种模式下 `on_open_change` 都会接收每一次状态变化。

- 指针悬停在触发器上时，会在等待 `open_delay_ms`（默认 `700` 毫秒）后打开卡片。键盘聚焦触发器时会立即打开。
- 当指针或焦点位于触发器或卡片内部时，卡片会保持打开，以便指针能够移入卡片内部点击链接。当指针同时离开两者时，会在等待 `close_delay_ms`（默认 `300` 毫秒）后关闭。当焦点同时离开两者时，除非指针仍悬停在其中之一上方，否则会立即关闭。
- 点击触发器会正常跳转链接并保持卡片打开。
- 触控指针（touch）不会触发打开悬浮卡片。
- `HoverCardTrigger` 渲染带有必填 `href` 的 `a` 标签，并透传 `target`、`rel` 等链接属性。
- 内容容器锚定到 `HoverCardTrigger`。
- 内容采用固定定位放置在锚点旁，首选方向空间不足时自动翻转至对侧，偏移以保持在视口内，并跟随窗口缩放与滚动。`side_offset` 默认为 `4` 像素。
- `side` 默认为 `Bottom`，`align` 默认为 `Center`。定位完成后，渲染的 `data-side` 会更新为实际翻转后的方向。
- `Escape`、外部指针按下及焦点移出外部会根据 `dismiss`（默认 `DismissBehavior::popover_default()`）关闭卡片。

Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试。

## 无障碍说明

悬浮卡片是为有视觉的指针和键盘用户提供的补充预览；与 Radix Hover Card 一致，触发器不会建立指向卡片的 ARIA 关联，且内容容器不渲染特定角色（参见 [RFC 0055](../../../../docs/rfcs/0055-open-state-accessibility-audit.md)）。移动端交互不应依赖悬停行为；关键操作请改用点击展开，或优先选用 `Popover` / `Sheet`。相邻卡片间的免延迟切换、触控打开以及非链接类型的触发器均未实现（参见 [RFC 0023](../../../../docs/rfcs/0023-hover-card-hover-and-focus-opening.md)）。
