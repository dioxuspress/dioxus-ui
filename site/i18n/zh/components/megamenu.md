## API 概览

- `Megamenu`
- `MegamenuTrigger`
- `MegamenuContent`
- `MegamenuColumn`
- `MegamenuHeading`
- `MegamenuLink`
- `megamenu_class`、`megamenu_trigger_class`、`megamenu_content_class`
- `megamenu_column_class`、`megamenu_heading_class`、`megamenu_link_class`

```rust
rsx! {
  Megamenu {
    MegamenuTrigger { "组件分类" }
    MegamenuContent {
      MegamenuColumn {
        MegamenuHeading { "布局" }
        MegamenuLink { href: "/components/navbar".to_string(), "Navbar" }
      }
    }
  }
}
```

## 无障碍说明

`MegamenuTrigger` 同步暴露 `aria-expanded` 与 `aria-haspopup="true"`，支持按 `Escape` 键关闭面板，`MegamenuLink` 会通过 `safe_href` 自动过滤危险 URL 协议。
