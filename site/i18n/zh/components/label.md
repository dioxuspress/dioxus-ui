## API 概览

- `Label`
- `label_class`

## 无障碍说明

请使用 `for`（或等价的 Dioxus `r#for` 属性）将标签与其目标控件关联。对于不属于原生可关联标签元素（labelable elements）的自定义控件（例如 `Slider`），请改用 `aria-labelledby`：为 `Label` 设置 `id`（`Label` 会将其与其他属性一并透传），并在控件上引用。请避免将标签样式用于无关的辅助提示文本。
