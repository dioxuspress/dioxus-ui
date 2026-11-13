## API 概览

- `Bubble`
- `BubbleGroup`
- `BubbleContent`
- `BubbleReactions`
- `BubbleVariant`
- `BubbleAlign`
- `BubbleReactionSide`
- `BubbleReactionAlign`
- 各组成部分的样式类辅助函数

## 无障碍说明

Bubble 的各变体仅作用于视觉呈现。危险（destructive）、警告或特定状态的含义必须同时通过正文文本或上下文清晰呈现。表情回应按钮或纯图标内容需要由应用提供无障碍名称。

## 职责边界

Bubble 仅负责带边框的消息表面、对齐方式、变体样式以及表情回应（reactions）的摆放位置。头像、发送者名称、时间戳、送达状态、行级操作、Markdown 解析、代码渲染以及富文本安全清洗仍由应用自行负责，或交由更高层的消息组合部件处理。
