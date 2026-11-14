## API 概览

- `Join`
- `JoinOrientation`（`Horizontal`、`Vertical`）
- `join_class`
- `join_item_class`

```rust
rsx! {
  Join { orientation: JoinOrientation::Horizontal,
    Input { placeholder: "搜索组件..." }
    Button { "搜索" }
  }
}
```

## 无障碍说明

`Join` 仅负责相邻控件的布局与圆角拼接，并通过 `focus-visible:z-10` 确保键盘聚焦时的外轮廓不会被相邻边框遮挡。
