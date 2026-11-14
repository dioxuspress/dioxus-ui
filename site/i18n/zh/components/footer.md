## API 概览

- `Footer`
- `FooterLayout`（`Columns`、`Centered`）
- `FooterNav`
- `FooterTitle`
- `footer_class`
- `footer_nav_class`
- `footer_title_class`

```rust
rsx! {
  Footer {
    FooterNav {
      FooterTitle { "产品" }
      a { href: "/components", "组件库" }
      a { href: "/themes", "主题预设" }
    }
  }
}
```

## 无障碍说明

`Footer` 渲染语义化 `<footer>` 元素，`FooterNav` 与 `FooterTitle` 采用非地标容器以避免多列页脚破坏页面标题层级或引发 `<nav>` 地标重复告警。
