## API 概览

- `Link`
- `LinkVariant`（`Default`、`Primary`、`Secondary`、`Accent`、`Neutral`、`Muted`、`Success`、`Warning`、`Destructive`）
- `LinkUnderline`（`Always`、`Hover`、`None`）
- `link_class`

```rust
rsx! {
  Link {
    href: "https://dioxuspress.dev".to_string(),
    variant: LinkVariant::Primary,
    underline: LinkUnderline::Hover,
    external: true,
    "阅读文档"
  }
}
```

## 无障碍说明

`Link` 渲染原生 `<a>` 元素并提供 `focus-visible` 焦点环样式，会自动过滤 `javascript:` / `data:` 等不安全协议，并在 `external: true` 时附加 `rel="noopener noreferrer"`。
