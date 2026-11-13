## API 概览

- `Textarea`
- `textarea_class`

## 变更事件

```rust
let mut notes = use_signal(String::new);

rsx! {
  Label { r#for: "notes", "Notes" }
  Textarea {
    id: "notes",
    value: notes(),
    on_value_change: move |value| notes.set(value),
  }
}
```

`Textarea` 是受控组件。每次触发 `input` 事件时，都会以最新文本调用 `on_value_change`，由应用将其回传给 `value`。其他属性（如 `id`、`name`、`rows` 和 `aria-describedby`）会透传至底层 `textarea` 元素。

## 无障碍说明

请将 `Textarea` 与 `Label` 搭配使用，并在字段校验不通过时设置 `invalid`；该属性会同时渲染 `aria-invalid` 与警示色边框。
