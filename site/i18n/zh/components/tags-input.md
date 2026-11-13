## API 概览

- `TagsInput`
- `tags_input_class`

```rust
let mut topics = use_signal(Vec::<String>::new);

rsx! {
  TagsInput {
    "aria-label": "Topics",
    placeholder: "Add a topic",
    tags: topics(),
    on_tags_change: move |tags| topics.set(tags),
  }
}
```

按下 `Enter` 或输入逗号会将去除首尾空白后的草稿文本添加为标签（若为空或已存在则忽略）；粘贴包含逗号的文本时，会将其中每一个完整片段依次添加为标签。在输入框为空时按下 `Backspace` 会移除最后一个标签，且每个标签胶囊都带有独立的移除按钮。标签列表变更时会以新列表调用 `on_tags_change`。`remove_label` 用于设置移除按钮无障碍名称的前缀，默认为 `"Remove"`。其余属性会透传至文本输入框。

## 无障碍说明

已添加的标签渲染为 `ul` 列表，以便辅助技术播报标签数量，且每个移除按钮命名为 `"Remove {tag}"`。请使用 `Label` 或 `aria-label` 为文本输入框命名。按下 `Enter` 会添加标签而不会提交外层表单（参见 [RFC 0060](../../../../docs/rfcs/0060-input-components.md)）。
