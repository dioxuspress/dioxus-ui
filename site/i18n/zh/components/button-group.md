## API 概览

- `ButtonGroup`
- `ButtonGroupItem`
- `ButtonGroupOrientation`
- `button_group_class`
- `button_group_item_class`

## 无障碍说明

`ButtonGroup` 渲染带有 `role="group"` 的外层容器以及原生按钮子项。当分组需要无障碍名称时，请传入 `aria_label`。仅图标按钮的无障碍名称、命令行为、按下状态，以及任何工具栏或漫游焦点（roving-focus）语义均由应用自行管理。`ButtonGroupItem` 接收 `onclick`，并透传 `aria-pressed`、`aria-label` 等其他按钮属性。由于 `role="group"` 不支持 `aria-orientation`，分组仅通过 `data-orientation` 暴露其排列方向。
