## API 概览

- `NativeSelect { invalid, disabled, class, on_value_change, children }`
- `NativeSelectGroup { label, class, children }`
- `NativeSelectOption { value, disabled, selected, class, children }`

样式类名辅助函数：

- `native_select_class(invalid, class)`
- `native_select_group_class(class)`
- `native_select_option_class(class)`

## 变更事件

```rust
let mut size = use_signal(|| "md".to_string());

rsx! {
  Label { r#for: "size", "Size" }
  NativeSelect {
    id: "size",
    name: "size",
    on_value_change: move |value| size.set(value),
    NativeSelectOption { value: "sm", selected: size() == "sm", "Small" }
    NativeSelectOption { value: "md", selected: size() == "md", "Medium" }
  }
}
```

选项变更时会携带所选选项的 `value` 调用 `on_value_change`。请将对应选项标记为 `selected` 以保持下拉框处于受控状态。其他属性（例如 `id`、`name`、`required` 与 `aria-describedby`）会透传至底层 `select` 元素。

## 无障碍说明

`NativeSelect` 完整保留了浏览器与操作系统的原生键盘、焦点及表单提交行为。建议尽可能为其配对可见的 `Label`，使用 `invalid` 渲染 `aria-invalid`，并在选项不可用时在下拉框或单个选项上设置 `disabled`。

对于少量选项、简单设置项以及移动端表单，建议优先选用 `NativeSelect` 而非自定义 `Select` 或 `Combobox`；当用户在选择前需要搜索或过滤选项时，则优先选用 `Combobox`。
