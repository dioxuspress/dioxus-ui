## API 概览

- `NavigationMenu`
- `NavigationMenuList`
- `NavigationMenuItem`
- `NavigationMenuTrigger`
- `NavigationMenuContent`
- `NavigationMenuLink`
- `NavigationMenuViewport`
- `NavigationMenuIndicator`
- `NavigationMenuOrientation`
- `NavigationMenuPrimitiveConfig`
- `navigation_menu_class`
- `navigation_menu_trigger_class`
- `navigation_menu_link_class`

该模块还重新导出了 `PopoverPrimitiveConfig`，便于从 `dioxus_shadcn::navigation_menu` 直接导入的用户使用。

## 行为与交互

与 Radix 一致，`NavigationMenu` 通过当前展开的 `NavigationMenuItem` 的 `value` 来管理展开项状态（全部收起时为空字符串）；每个菜单项的触发器与内容区都会将该状态与自身所属菜单项的 `value` 进行比对（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。传入 `value` 与 `on_value_change` 可进入受控模式，传入 `default_value` 可在初始时展开指定项；`on_value_change` 会在每次状态变更时触发，关闭时传入空字符串。未显式指定 `value` 的菜单项会自动生成一个唯一值：

```rust
rsx! {
  NavigationMenu { "aria-label": "Product",
    NavigationMenuList {
      NavigationMenuItem { value: "docs",
        NavigationMenuTrigger { "Docs" }
        NavigationMenuContent {
          NavigationMenuLink { href: "/docs/install", "Install" }
        }
      }
      NavigationMenuItem {
        NavigationMenuLink { href: "/blog", "Blog" }
      }
    }
  }
}
```

- 在触发器上点击或按 `Enter`、`Space` 会切换对应内容区的展开与收起。
- 使用鼠标时，指针悬停在触发器上 200 ms 后会展开内容区；若此时已有其他项处于展开状态，则会立即切换展开。指针移出已展开项的触发器与内容区 300 ms 后会自动关闭。通过点击手动关闭的触发器在指针移出前不会因悬停再次展开。触屏设备仅响应点击操作。
- 所有触发器与链接均保留在 `Tab` 焦点顺序中，且内容区紧跟在其触发器之后，因此在已展开的触发器上按 `Tab` 会直接进入其内部的链接列表。
- `ArrowLeft` 与 `ArrowRight` 在顶层触发器和链接之间循环移动焦点；`Home` 与 `End` 分别跳至首项与末项。禁用的触发器及带有 `disabled` 的链接会被跳过。
- 在从右到左（RTL）布局中，`ArrowLeft` 移至下一个顶层菜单项，`ArrowRight` 移至上一个顶层菜单项。
- 在触发器上按 `ArrowDown` 会将焦点移入其内容区的第一个链接（若尚未展开则会先将其展开）。在内容区内部，`ArrowDown` 与 `ArrowUp` 在各链接间循环移动焦点；`Home` 与 `End` 分别跳至第一个与最后一个链接。
- 按 `Escape` 会关闭当前展开的内容区，且当焦点原本位于菜单内时会将焦点移回对应触发器。在菜单外部按下指针、焦点移出菜单，或点击内容区内的链接，都会关闭当前内容区。

内容区通过 CSS 布局，而非浮层锚点定位。只要有任意菜单项处于展开状态，`NavigationMenuViewport` 与 `NavigationMenuIndicator` 就会显示。

内容区的展开由应用状态驱动，因此在服务端渲染（SSR）页面中仅在水合（hydration）完成后才可展开；在此之前，顶层 `NavigationMenuLink` 可作为普通链接正常跳转，而各内容区保持 `hidden`。若在水合前通过 CSS `:hover` 展示内容区，水合后就会出现双重状态源（例如悬停展开的内容区无法响应 `Escape`，或在触发器标示 `aria-expanded="false"` 时仍然可见）；因此若菜单必须在无 Wasm 环境下工作，建议直接使用原生链接配合应用自身的 CSS 实现。

### 子菜单

在内容区内嵌套一个设置 `orientation: NavigationMenuOrientation::Vertical` 的 `NavigationMenu` 即可构建大型多级菜单（mega menu）：其触发器呈纵向排列，各内容面板并排显示在列表一侧，并通过各自的 `value` 与对应菜单项关联：

```rust
NavigationMenuContent {
  NavigationMenu {
    orientation: NavigationMenuOrientation::Vertical,
    "aria-label": "Solution areas",
    // Keep a panel open: ignore the empty value a close sends.
    value: area(),
    on_value_change: move |value: String| if !value.is_empty() { area.set(value) },
    NavigationMenuList {
      NavigationMenuItem { value: "web", NavigationMenuTrigger { "Web" } }
      NavigationMenuItem { value: "mobile", NavigationMenuTrigger { "Mobile" } }
    }
    NavigationMenuContent { value: "web", /* links */ }
    NavigationMenuContent { value: "mobile", /* links */ }
  }
}
```

每个菜单的交互脚本仅作用于其自身的菜单项。在纵向子菜单中，点击或悬停会立即打开对应面板，`ArrowDown` 与 `ArrowUp` 在各触发器间移动焦点，`ArrowRight` 进入面板而 `ArrowLeft` 返回触发器，且指针移出时面板保持展开；因此建议始终保持一个面板处于打开状态（例如在外层菜单项展开时默认选中第一个子面板）。按 `Escape` 会同时关闭内外两层菜单，并将焦点移回外层触发器（参见 [RFC 0063](../../../../docs/rfcs/0063-navigation-menu-submenus.md)）。

Web 渲染器的交互验证由 `npm run verify:runtime-interactions` 覆盖，Desktop 渲染器由 `npm run verify:desktop-interactions` 覆盖，iOS 模拟器由 `npm run verify:mobile-interactions` 覆盖，Android 模拟器由 `npm run verify:android-interactions` 覆盖。

## 无障碍说明

请将 `NavigationMenu` 用于站点页面导航，应用级命令菜单请改用 `Menubar` 或 `ContextMenu`。`NavigationMenu` 遵循披露式导航（disclosure navigation）模式：触发器为带有 `aria-expanded` 的按钮，内容区承载链接，不使用 `menu` 角色。视口尺寸动态测量与过渡动画暂未实现（参见 [RFC 0016](../../../../docs/rfcs/0016-navigation-menu-interaction.md)）。

`NavigationMenu` 支持透传属性；当页面存在多个导航地标时，请为该 `nav` 地标设置 `aria-label`（参见 [RFC 0040](../../../../docs/rfcs/0040-composite-widget-names.md)）。
