## API 概览

- `Popover`
- `PopoverTrigger`
- `PopoverContent`
- `PopoverHeader`
- `PopoverTitle`
- `PopoverDescription`
- `PopoverPrimitiveConfig`
- `PopoverDismissBehavior`, `OverlaySide`, `OverlayAlign`
- `popover_content_class`

## 行为与交互

`Popover` 持有弹出框是否打开的状态，内部各子部件均读取该状态，因此必须放置在 `Popover` 内部（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。`PopoverTrigger` 用于切换展开状态并作为内容容器的锚点；它渲染一个无默认样式的 `button`，请通过 `class` 添加样式：

```rust
rsx! {
  Popover {
    PopoverTrigger {
      class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
      "Share"
    }
    PopoverContent { PopoverTitle { "Share link" } }
  }
}
```

可传入 `open` 与 `on_open_change` 进行受控管理，或传入 `default_open` 默认打开；两种模式下 `on_open_change` 都会接收每一次状态变化。触发器会设置 `aria-expanded` 并将 `aria-controls` 指向内容容器。

- 内容采用固定定位放置在触发器旁的 `side`（默认 `Bottom`）方向，配合 `align`（默认 `Center`）对齐与 `side_offset`（默认 `4`）像素偏移。首选方向空间不足时会自动翻转至对侧，并沿交叉轴偏移以保持距离视口边缘至少 8 像素。打开期间会跟随窗口缩放与滚动更新位置。
- `Escape`、在内容与触发器之外按下指针，以及焦点移出外部时，会根据 `dismiss`（默认 `DismissBehavior::popover_default()`）关闭弹出框。在触发器上按下指针视为内部交互，因此点击触发器可持续正常切换开关。
- 打开时焦点不会自动移入弹出框内部。

Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试，Desktop 渲染器由 `npm run verify:desktop-interactions` 覆盖，iOS 模拟器由 `npm run verify:mobile-interactions` 覆盖，Android 模拟器由 `npm run verify:android-interactions` 覆盖。

## 无障碍说明

请将弹出框用于补充性的可交互内容。请合理配置关闭行为，确保键盘与指针用户都能以符合预期的方式关闭浮层。

`PopoverContent` 会自动从内部的 `PopoverTitle` 获取无障碍名称，并从 `PopoverDescription` 获取描述：这两个子部件会自动生成唯一 ID，内容容器会将 `aria-labelledby` 和 `aria-describedby` 指向当前已挂载的对应部件。在 `PopoverContent` 上显式传入 `aria-label`、`aria-labelledby` 或 `aria-describedby` 会覆盖自动生成的值；因此当不包含标题部件时，请直接传入 `aria-label`（参见 [RFC 0039](../../../../docs/rfcs/0039-dialog-names.md)）。
