## API 概览

- `Alert`
- `AlertTitle`
- `AlertDescription`
- `AlertVariant`
- `alert_class`
- `alert_title_class`
- `alert_description_class`

`AlertVariant` 包含 `Default`、`Destructive` 以及状态变体 `Success`、`Warning` 和 `Info`。由于状态颜色直接作为正文文本时对比度偏浅，状态提示框会对背景表面与边框着色并保留前景色文本；请将相同的变体一并传给 `AlertDescription`（参见 [RFC 0058](../../../../docs/rfcs/0058-status-variants.md)）。

## 无障碍说明

`Alert` 始终渲染 `role="alert"`，因此仅用于需要立即播报的重要消息；对于无需打断用户的静态辅助说明，请改用普通的 `div`。

与 shadcn/ui v4 一致，`AlertTitle` 渲染为 `div`，以避免跳过页面的标题层级。当页面大纲确实需要标题语义时，请在其中包裹合适层级的标题元素（参见 [RFC 0054](../../../../docs/rfcs/0054-automated-accessibility-audit.md)）。
