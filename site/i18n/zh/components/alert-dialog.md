## API 概览

- `AlertDialog`
- `AlertDialogTrigger`
- `AlertDialogOverlay`
- `AlertDialogContent`
- `AlertDialogHeader`
- `AlertDialogFooter`
- `AlertDialogTitle`
- `AlertDialogDescription`
- `AlertDialogAction`
- `AlertDialogCancel`
- `AlertDialogActionVariant`
- `AlertDialogPrimitiveConfig`
- `alert_dialog_overlay_class`
- `alert_dialog_content_class`
- `alert_dialog_action_class`

该模块还重导出了 `DialogPrimitiveConfig`，方便直接从 `dioxus_shadcn::alert_dialog` 导入使用。

## 行为与交互

`AlertDialog` 持有警告对话框是否打开的状态，内部各子部件均读取该状态，因此必须放置在 `AlertDialog` 内部（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。`AlertDialogTrigger` 用于打开对话框；它渲染一个无默认样式的 `button`，请通过 `class` 添加样式。若需受控管理，请传入 `open` 与 `on_open_change`；若需默认打开，请传入 `default_open`；两种模式下 `on_open_change` 都会接收每一次状态变化。

打开期间，警告对话框会锁定页面滚动，并为隐藏滚动条后的根元素补充内边距以防止布局抖动；嵌套模态层共享同一把滚动锁，当最后一个模态层关闭时恢复页面滚动（参见 [RFC 0068](../../../../docs/rfcs/0068-modal-scroll-lock.md)）。

```rust
rsx! {
  AlertDialog {
    AlertDialogTrigger {
      class: button_class(ButtonVariant::Destructive, ButtonSize::Md, use_density(), ""),
      "Delete project"
    }
    AlertDialogOverlay {}
    AlertDialogContent {
      AlertDialogTitle { "Delete project?" }
      AlertDialogCancel { "Cancel" }
      AlertDialogAction { onclick: move |_| delete_project(), "Delete" }
    }
  }
}
```

- 当启用了 `dismiss.escape_key` 时，在对话框内容区域按下 `Escape` 会关闭对话框。
- 点击遮罩层（overlay）永远不会关闭警告对话框。
- `AlertDialogCancel` 与 `AlertDialogAction` 会先执行各自的 `onclick` 回调，随后关闭对话框。
- 打开时会优先聚焦标记了 `data-dxui-autofocus` 的元素（例如由键盘管理的 `Calendar` 日期格），否则聚焦第一个可聚焦元素，若均不存在则聚焦内容容器本身。
- 打开期间，`Tab` 与 `Shift+Tab` 会在内容区域内部循环移动焦点，并跳过 `tabindex="-1"` 的元素。
- 关闭对话框或将内容从组件树中移除时，焦点会恢复到打开前聚焦的元素，除非焦点已经主动移到了内容区域之外的其他控件。

请将取消按钮渲染在破坏性操作按钮之前，使其默认获得初始焦点。目前仅 Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试。

## 无障碍说明

内容容器使用 `role="alertdialog"` 与 `aria-modal="true"`。使用方应用应为破坏性操作提供清晰明确的标签，并确保用户在确认执行破坏性操作前能够方便地访问取消按钮。

Portal 挂载与过渡动画仍由应用自行管理。

`AlertDialogContent` 会自动从内部的 `AlertDialogTitle` 获取无障碍名称，并从 `AlertDialogDescription` 获取描述：这两个子部件会自动生成唯一 ID，内容容器会将 `aria-labelledby` 和 `aria-describedby` 指向当前已挂载的对应部件。在 `AlertDialogContent` 上显式传入 `aria-label`、`aria-labelledby` 或 `aria-describedby` 会覆盖自动生成的值；因此当不包含标题部件时，请直接传入 `aria-label`（参见 [RFC 0039](../../../../docs/rfcs/0039-dialog-names.md)）。
