## API 概览

- `Stack`
- `StackAlign`（`Bottom`、`Top`、`Start`、`End`）
- `stack_class`

```rust
rsx! {
  Stack { align: StackAlign::Bottom, class: "w-64",
    Card { CardBody { "Notification 1" } }
    Card { CardBody { "Notification 2" } }
    Card { CardBody { "Notification 3" } }
  }
}
```

## 无障碍说明

所有堆叠的子元素仍按源码顺序保留在 DOM 中。若后层卡片仅用于视觉衬托或包含重复内容，建议为其添加 `aria-hidden="true"` 以避免屏幕阅读器重复播报。
