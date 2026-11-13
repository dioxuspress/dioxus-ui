## API 概览

- `Menubar`
- `MenubarMenu`
- `MenubarTrigger`
- `MenubarContent`
- `MenubarItem`
- `MenubarCheckboxItem`
- `MenubarRadioGroup`
- `MenubarRadioItem`
- `MenubarLabel`
- `MenubarSeparator`
- `MenubarShortcut`
- `MenubarSub`, `MenubarSubTrigger`, `MenubarSubContent`
- `MenubarPrimitiveConfig`
- `MenubarDismissBehavior`, `MenubarAlign`, `MenubarSide`
- `menubar_class`
- `menubar_trigger_class`
- `menubar_item_class`
- `menubar_checkbox_item_class`
- `menubar_radio_item_class`
- `menubar_sub_trigger_class`

该模块还重导出了 `DropdownPrimitiveConfig`、`DismissBehavior`、`OverlayAlign` 与 `OverlaySide`，方便直接从 `dioxus_shadcn::menubar` 导入使用。

## 行为与交互

与 Radix 一样，`Menubar` 通过其子级 `MenubarMenu` 的 `value` 记录当前处于打开状态的菜单（无任何菜单打开时为空字符串，参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。每个菜单的触发器与内容容器会自动读取该状态并互相关联 ID。可传入 `value` 与 `on_value_change` 进行受控管理，或传入 `default_value` 默认打开指定菜单。未显式传入 `value` 的 `MenubarMenu` 会自动生成一个唯一值。

`MenubarRadioGroup` 通过 `value`、`default_value` 与 `on_value_change` 管理单选值，其子项接收 `value`；`MenubarCheckboxItem` 接收 `checked`，由应用在 `onclick` 中翻转状态。选中的复选菜单项会在左侧内缩区显示勾选图标，选中的单选菜单项则显示圆点。

```rust
rsx! {
  Menubar { "aria-label": "Editor",
    MenubarMenu { value: "file",
      MenubarTrigger { "File" }
      MenubarContent {
        MenubarItem { onclick: move |_| new_file(), "New" }
      }
    }
  }
}
```

- 所有菜单触发器共同组成单一的 `Tab` 停靠点。左右方向键（`ArrowLeft` / `ArrowRight`）在可用触发器之间循环移动焦点；`Home` 与 `End` 跳转到首个与末个触发器。
- 点击触发器会切换对应菜单的展开状态。在收起状态的触发器上按下 `ArrowDown`、`Enter` 或 `Space` 会打开该菜单。
- 内容容器放置在触发器下方（`side` 为 `Bottom`、`align` 为 `Start`、`side_offset` 为 `4`），空间不足时自动翻转与偏移以留在视口内，并根据 `dismiss`（默认 `DismissBehavior::popover_default()`）在按下 `Escape`、外部指针按下或焦点移出外部时关闭。
- 打开后的菜单行为与 `Dropdown` 一致：首个可用项获得焦点，`ArrowDown` 与 `ArrowUp` 循环移动，`Home`、`End` 与字符前缀输入（typeahead）可快速跳转，在可用项上按下 `Enter`、`Space` 或点击会调用其 `onclick` 并关闭菜单。
- 当已有菜单处于打开状态时，按下左右方向键会直接切换到相邻菜单，将指针移入另一个可用触发器也会立即切换打开该触发器对应的菜单。
- 在从右到左（RTL）布局中，`ArrowLeft` 移向下一个触发器或菜单，`ArrowRight` 移向上一个。
- 关闭菜单时会将焦点返还给刚才打开菜单的触发器，除非焦点已经移至其他控件。按下 `Tab` 会将焦点移出菜单并关闭菜单。

Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试，Desktop 渲染器由 `npm run verify:desktop-interactions` 覆盖，iOS 模拟器由 `npm run verify:mobile-interactions` 覆盖，Android 模拟器由 `npm run verify:android-interactions` 覆盖。

### 子菜单

`MenubarSub` 承载嵌套子菜单（[RFC 0067](../../../../docs/rfcs/0067-menu-submenus.md)）并持有其展开状态；可传入 `open` 与 `on_open_change` 进行受控管理：

```rust
rsx! {
  MenubarSub {
    MenubarSubTrigger { "Share" }
    MenubarSubContent {
      MenubarItem { onclick: move |_| copy_link(), "Copy link" }
    }
  }
}
```

- 在子菜单触发项上按下 `ArrowRight`（RTL 布局下为 `ArrowLeft`）、`Enter`、`Space` 或点击，会打开子菜单并聚焦其第一项；指针悬停在触发项上也会打开子菜单，但不会移动焦点。
- 在子菜单内按下 `ArrowLeft`（RTL 布局下为 `ArrowRight`）或 `Escape` 会关闭当前层级，并将焦点返回给该子菜单的触发项。
- 选中子菜单内的任意项会关闭所有层级的菜单。高亮外层菜单的其他项或关闭外层菜单时，子菜单也会随之关闭。
- 子菜单在触发项的行内末尾弹出，空间不足时自动翻转。当指针斜向划过同级兄弟菜单项时会立即关闭子菜单（未设置指针宽限过渡区）。

## 无障碍说明

根元素使用 `role="menubar"`，触发器使用 `role="menuitem"` 并附带 `aria-haspopup="menu"`、`aria-expanded` 以及打开时的 `aria-controls`，内容容器使用 `role="menu"`。打开的菜单在各项之间直接移动 DOM 焦点，使屏幕阅读器在每项获得焦点时予以播报。子菜单触发项使用 `aria-haspopup="menu"`、`aria-expanded` 与 `aria-controls`，且每个子菜单位于 `group` 中并从其触发项继承无障碍名称（参见 [RFC 0067](../../../../docs/rfcs/0067-menu-submenus.md)）。按下 `ArrowUp` 打开菜单并聚焦末项的行为暂未实现（参见 [RFC 0015](../../../../docs/rfcs/0015-menubar-keyboard-behavior.md)）。

`Menubar` 支持属性透传，请使用 `aria-label` 为菜单栏命名（参见 [RFC 0040](../../../../docs/rfcs/0040-composite-widget-names.md)）。
