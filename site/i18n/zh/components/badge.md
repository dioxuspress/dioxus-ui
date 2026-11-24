## API 概览

- `Badge`（`variant`、`style`、`size`、`pill`、`class`）
- `BadgeVariant`（`Default`、`Secondary`、`Destructive`、`Outline`、`Success`、`Warning`、`Info`）
- `BadgeStyle`（`Solid`、`Soft`、`Outline`、`Dash`）
- `BadgeSize`（`Xs`、`Sm`、`Md`、`Lg`）
- `badge_class`
- `badge_styled_class`

`BadgeVariant` 提供 `Default`、`Secondary`、`Destructive` 与 `Outline` 四种基础变体，以及 `Success`、`Warning` 与 `Info` 三种状态变体；实心样式（`BadgeStyle::Solid`）会在对应状态色背景上绘制 `text-success-foreground` 等前景色令牌（[RFC 0058](../../../../docs/rfcs/0058-status-variants.md)）。`BadgeStyle::Soft`、`Outline` 与 `Dash` 仅对背景或边框着色并保持 `text-foreground`，确保全预设主题满足 WCAG AA 对比度。

## 无障碍说明

请将徽章作为辅助性说明文本使用，不要仅凭徽章颜色作为传达关键状态的唯一方式；带颜色编码的状态徽章应搭配清晰的文字表述。
