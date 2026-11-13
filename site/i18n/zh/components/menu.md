## API 概览

- `Menu`
- `MenuTitle`
- `MenuItem`
- `MenuGroup`
- `menu_class`
- `menu_item_class`

```rust
rsx! {
  nav { "aria-label": "Documentation",
    Menu {
      MenuTitle { "Guides" }
      MenuItem { href: "/docs/start", active: true, "Getting started" }
      MenuItem { href: "/docs/theming", "Theming" }
      MenuGroup {
        label: rsx! { "Components" },
        default_open: true,
        MenuItem { href: "/components/button", "Button" }
        MenuItem { href: "/components/dialog", "Dialog" }
      }
    }
  }
}
```

## 行为与交互

参见 [RFC 0072](../../../../docs/rfcs/0072-menu-and-mockup.md)。

- `Menu` 渲染为 `ul`，`MenuTitle` 与 `MenuItem` 渲染为 `li` 元素，而 `MenuGroup` 渲染为包含一个按钮和嵌套 `ul` 的 `li`。
- 带有 `href` 的菜单项渲染为链接，带有 `onclick` 的渲染为按钮；若两者均未提供，则直接包裹应用自行传入的链接或按钮。禁用的链接会移除 `href`，禁用的按钮会设置原生 `disabled`，因此两者都不会接收焦点或触发 `onclick`。
- `MenuGroup` 持有自身的展开状态：可通过 `default_open` 指定初始状态，或通过 `open` 由应用受控管理，且 `on_open_change` 会接收用户触发的每一次状态变更。点击分组按钮会切换展开状态，收起时嵌套列表处于隐藏状态；折叠箭头在收起时朝下、展开时朝上。
- 支持多层嵌套：一个 `MenuGroup` 内部可以包含另一个 `MenuGroup`。

与 `Dropdown` 或 `Menubar` 不同，`Menu` 不使用 `role="menu"`；它本质上是普通的链接列表，用户可像浏览其他常规列表一样通过 `Tab` 键在其中阅读和切换焦点。

## 无障碍说明

请将 `Menu` 放置在带有 `aria-label` 的 `nav` 元素内。当前激活的链接或按钮带有 `aria-current="page"`。分组按钮使用 `aria-expanded` 与 `aria-controls` 关联嵌套列表。`MenuTitle` 仅作为纯文本展示；当一组链接需要支持折叠并使用标题为其命名时，请改用 `MenuGroup` 包裹。
