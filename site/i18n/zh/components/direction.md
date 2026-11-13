## API 概览

- `Direction`
- `TextDirection`
- `direction_class`

## 无障碍说明

`Direction` 会设置原生的 `dir` 属性以及与之对应的 `data-direction` 属性。语言区域（locale）检测、文档级方向策略、文本塑形（text shaping）以及混合方向内容编排仍由应用自行负责。当内容需要时，可以嵌套多层 `Direction` 包装容器。

`Tabs`、`RadioGroup`、`ToggleGroup`、`Menubar` 与 `NavigationMenu` 在按下方向键时会读取计算后的文本方向。在从右到左（RTL）的包装容器内部，`ArrowLeft` 会移至下一项，而 `ArrowRight` 会移至上一项（参见 [RFC 0025](../../../../docs/rfcs/0025-right-to-left-arrow-mirroring.md)）。
