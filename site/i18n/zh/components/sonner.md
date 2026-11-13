## API 概览

- `SonnerViewport`
- `SonnerToast`
- `SonnerIcon`
- `SonnerContent`
- `SonnerTitle`
- `SonnerDescription`
- `SonnerAction`
- `SonnerClose`
- `SonnerItem`
- `SonnerQueue`
- `SonnerPlacement`
- `SonnerVariant`
- `sonner_queue_push`
- `sonner_queue_dismiss`
- `sonner_is_expired`
- `SonnerDismissReason`
- `sonner_dismiss_reason_attribute`

## 行为与交互

应用自行维护通知队列，并为每个条目渲染一个 `SonnerToast`。请将同一个 `on_dismiss` 处理函数同时传给 `SonnerToast`、`SonnerAction` 和 `SonnerClose`；该函数会接收 `SonnerDismissReason`，随后通常配合 `sonner_queue_dismiss` 更新队列。

- `SonnerToast` 在打开满 `duration_ms`（默认 `5000` 毫秒）后调用 `on_dismiss(Timeout)`，当指针悬停在通知上或焦点位于其内部时倒计时会暂停。设为 `0` 可禁用倒计时，适用于加载中状态的通知。
- `open` 默认为 `true`；设为 `false` 可在不卸载组件的情况下隐藏通知。
- `SonnerAction` 会先执行 `onclick`，随后调用 `on_dismiss(Action)`。
- `SonnerClose` 会调用 `on_dismiss(Close)`。它是一个无障碍名称为 `"Close notification"` 的 28px 图标按钮，因此请传入符号或图标而非文字。

Sonner 与 Toast 共用同一套倒计时脚本；目前仅 Web 渲染器纳入了 `npm run verify:runtime-interactions` 自动化验证。

## 无障碍说明

`SonnerViewport` 是一个持久化的 `role="region"` 容器，标签为 `Notifications` 且带有 `aria-live="polite"`。请仅挂载一次并在其内部追加通知，以便辅助技术能够播报已跟踪区域内新增的内容。

通知根节点会根据变体紧急程度暴露状态语义与 `aria-live`。变体图标均为装饰性元素。悬停与聚焦时倒计时暂停，确保消息可被完整阅读。播报文案、异步状态（Promise）编排、Portal 挂载位置、`Escape` 键行为以及焦点策略由应用自行管理。
