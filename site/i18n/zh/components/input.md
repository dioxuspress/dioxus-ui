## API 概览

- `Input`
- `input_class`

## 变更事件

```rust
let mut notes = use_signal(String::new);

rsx! {
  Label { r#for: "notes", "Notes" }
  Input {
    id: "notes",
    value: notes(),
    on_value_change: move |value| notes.set(value),
  }
}
```

`Input` 为受控组件。每次触发 `input` 事件时都会携带最新文本调用 `on_value_change`，再由应用将其回传给 `value`。其他属性（例如 `id`、`name`、`type`、`autocomplete` 与 `aria-describedby`）会透传至底层 `input` 元素。

## 无障碍说明

请将输入框与 `Label` 搭配使用，并在字段校验未通过时设置 `invalid`；该属性会同时渲染 `aria-invalid` 和警示色边框。
