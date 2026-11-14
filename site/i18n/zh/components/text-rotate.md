## API 概览

- `TextRotate`
- `text_rotate_class`

```rust
rsx! {
  h2 { class: "text-2xl font-bold",
    "Build "
    TextRotate {
      class: "text-primary",
      items: vec!["faster".to_string(), "safer".to_string(), "accessible".to_string()],
    }
    " apps"
  }
}
```

## 无障碍说明

非当前激活词条均标记为 `aria-hidden="true"`，确保屏幕阅读器只朗读当前显示的词条。可通过 `index: Some(...)` 接管受控播放或适配减弱动画偏好。
