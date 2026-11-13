## API 概览

- `Dock`
- `DockItem`
- `DockLabel`
- `dock_class`
- `dock_item_class`
- `dock_label_class`

```rust
rsx! {
  Dock { "aria-label": "Main",
    DockItem { href: "/", active: true, HomeIcon {}, DockLabel { "Home" } }
    DockItem { href: "/search", SearchIcon {}, DockLabel { "Search" } }
    DockItem { href: "/profile", UserIcon {}, DockLabel { "Profile" } }
  }
}
```

`Dock` 默认固定在视口底部，并通过 `env(safe-area-inset-bottom)` 自动预留底部安全区内边距以避开系统手势条（home indicator）；请在页面底部内容处为其留出足够空间。设置 `fixed: false` 可使其保留在普通文档流中。`DockItem` 在传入 `href` 时渲染链接，未传入时渲染按钮；`active` 会使用主色调指示条标记当前所在页面。若与路由器搭配使用，可将 `dock_item_class(active, "")` 应用到 `Link` 组件上。

## 无障碍说明

`Dock` 是一个 `nav` 导航地标（landmark），请通过 `aria-label` 为其命名。当前激活项会设置 `aria-current="page"`，且每一项的 `DockLabel` 为其提供可见名称——即便图标含义已足够常见，也建议保留可见标签（参见 [RFC 0061](../../../../docs/rfcs/0061-mobile-navigation.md)）。
