## API 概览

- `InputGroup`
- `InputGroupAddon`
- `InputGroupControl`
- `InputGroupAction`
- `InputGroupAddonPosition`
- `input_group_class`
- `input_group_addon_class`
- `input_group_control_class`
- `input_group_action_class`

## 无障碍说明

`InputGroup` 将实际的输入控件交由应用自行传入，从而完整保留原生输入框语义。请为组合输入框配对 `Label`；对于装饰性前后缀插件（addon），请视情况使用纯文本或添加 `aria-hidden`。`InputGroupAction` 渲染接收 `onclick` 并透传其他属性的原生按钮；仅包含图标的操作按钮需由应用提供 `aria-label`。
