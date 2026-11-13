## API 概览

- `Field`
- `FieldLabel`
- `FieldDescription`
- `FieldError`
- `FieldGroup`
- 各子组件的样式类名辅助函数

## 无障碍说明

- `Field` 通过 data 属性暴露校验失败（invalid）与禁用（disabled）状态。
- `FieldLabel` 渲染原生 `label` 元素。请传入指向控件 `id` 的 `r#for`，或直接包裹控件；空的 `for` 属性会被自动省略。`id` 等其他属性会透传。
- `FieldDescription` 和 `FieldError` 会透传其他属性，因此可为它们设置 `id` 并将控件的 `aria-describedby` 指向该 `id`。实际表单控件上的 `aria-describedby`、`aria-invalid` 以及实时播报区域（live-region）行为均由应用自行管理。
