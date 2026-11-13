## API 概览

- `Attachment`
- `AttachmentGroup`
- `AttachmentMedia`
- `AttachmentContent`
- `AttachmentTitle`
- `AttachmentDescription`
- `AttachmentActions`
- `AttachmentAction`
- `AttachmentTrigger`
- `AttachmentState`
- `AttachmentSize`
- `AttachmentOrientation`
- `AttachmentMediaVariant`
- 各组成部分的样式类辅助函数

## 无障碍说明

`AttachmentAction` 与 `AttachmentTrigger` 渲染为接收 `onclick` 并透传其余属性的原生按钮。纯图标操作按钮需要由应用提供 `aria-label`（例如 `"Remove report.pdf"`）。错误状态必须在 `AttachmentDescription` 中包含可见的文字说明，不能仅依赖颜色表达。

`AttachmentGroup` 仅提供视觉分组。当分组可聚焦或可滚动时，分组的无障碍标签由应用负责提供。

## 职责边界

Attachment 不负责文件上传传输、Object URL 生命周期、拖拽上传行为、预览加载、进度数值、重试逻辑或网络状态。应用需将这些状态传入 `AttachmentState`，并渲染相应的描述文本或操作按钮。
