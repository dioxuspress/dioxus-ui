## API 概览

- `Navbar`
- `NavbarStart`
- `NavbarCenter`
- `NavbarEnd`
- `navbar_class`
- `navbar_start_class`
- `navbar_center_class`
- `navbar_end_class`

```rust
rsx! {
  Navbar {
    NavbarStart { span { class: "font-semibold", "DioxusPress" } }
    NavbarCenter { a { href: "/docs", "文档" } }
    NavbarEnd { Button { "登录" } }
  }
}
```

## 无障碍说明

`Navbar` 渲染语义化 `<header>` 地标容器。若页面存在多个导航区域，建议在内层 `<nav>` 上补充 `aria-label`。
