## API 概览

- `Badge`
- `BadgeVariant`
- `badge_class`

`BadgeVariant` 提供 `Default`、`Secondary`、`Destructive` 与 `Outline` 四种基础变体，以及 `Success`、`Warning` 与 `Info` 三种实心状态变体；状态变体会在对应状态色背景上绘制 `text-success-foreground` 等前景色令牌（[RFC 0058](../../../../docs/rfcs/0058-status-variants.md)）。

## 无障碍说明

请将徽章作为辅助性说明文本使用，不要仅凭徽章颜色作为传达关键状态的唯一方式；带颜色编码的状态徽章应搭配清晰的文字表述。
