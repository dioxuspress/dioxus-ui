## API 概览

- `ToastViewport`
- `ToastRoot`
- `ToastTitle`
- `ToastDescription`
- `ToastAction`
- `ToastClose`
- `ToastItem`
- `ToastQueue`
- `ToastPlacement`
- `ToastVariant`
- `ToastDismissReason`
- `toast_queue_push`
- `toast_queue_dismiss`
- `toast_is_expired`

## 行为与交互

通知队列由应用自行管理。请将同一个 `on_dismiss` 处理函数传给 `ToastRoot`、`ToastAction` 和 `ToastClose`，它会接收 `ToastDismissReason`：

```rust
// A `Callback` is `Copy`, so the three parts can share it.
let dismiss = use_callback(move |reason: ToastDismissReason| {
  queue.set(toast_queue_dismiss(queue(), &id));
  log::info!("toast dismissed: {}", toast_dismiss_reason_attribute(reason));
});

rsx! {
  ToastRoot {
    on_dismiss: dismiss,
    ToastTitle { "Changes saved" }
    ToastAction { onclick: move |_| undo(), on_dismiss: dismiss, "Undo" }
    ToastClose { on_dismiss: dismiss, "×" }
  }
}
```

- `ToastRoot` 在打开满 `duration_ms`（默认 `5000` 毫秒）后调用 `on_dismiss(Timeout)`。当指针悬停在通知上或焦点位于其内部时倒计时暂停，移开后按剩余时间继续计时。设为 `0` 可禁用倒计时，适用于加载中状态的通知。
- `ToastAction` 会先执行 `onclick`，随后调用 `on_dismiss(Action)`。
- `ToastClose` 会调用 `on_dismiss(Close)`。它是一个无障碍名称为 `"Close notification"` 的 28px 图标按钮，因此请传入符号或图标而非文字。

倒计时通过 `document::eval` 运行，因此在 Web、Desktop 与 Mobile 渲染器中均可工作。Web 渲染器由 `npm run verify:runtime-interactions` 覆盖验证，Desktop 渲染器由 `npm run verify:desktop-interactions` 覆盖，iOS 模拟器由 `npm run verify:mobile-interactions` 覆盖，Android 模拟器由 `npm run verify:android-interactions` 覆盖。

## 无障碍说明

`ToastViewport` 是一个持久化的 `role="region"` 容器，标签为 `Notifications` 且带有 `aria-live="polite"`。请仅挂载一次并在其内部追加通知，以便辅助技术能够播报已跟踪区域内新增的内容。

根节点会根据变体紧急程度暴露状态语义与 `aria-live`。关闭与操作控件均为原生按钮。悬停与聚焦时倒计时暂停，确保消息可被完整阅读。播报文案、Portal 挂载位置、`Escape` 键行为以及是否将焦点移入通知操作按钮由应用自行决定。
