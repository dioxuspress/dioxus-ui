## API 概览

- `ContextMenu`
- `ContextMenuTrigger`
- `ContextMenuContent`
- `ContextMenuGroup`
- `ContextMenuLabel`
- `ContextMenuItem`
- `ContextMenuCheckboxItem`
- `ContextMenuRadioGroup`
- `ContextMenuRadioItem`
- `ContextMenuSeparator`
- `ContextMenuShortcut`
- `ContextMenuSub`, `ContextMenuSubTrigger`, `ContextMenuSubContent`
- `ContextMenuPrimitiveConfig`
- `ContextMenuDismissBehavior`, `ContextMenuSide`, `ContextMenuAlign`
- `context_menu_content_class`
- `context_menu_item_class`
- `context_menu_checkbox_item_class`
- `context_menu_radio_item_class`
- `context_menu_sub_trigger_class`

该模块还重导出了 `DropdownPrimitiveConfig`，方便直接从 `dioxus_shadcn::context_menu` 导入使用。

## 行为与交互

`ContextMenu` 持有上下文菜单是否打开的状态，内部各子部件均读取该状态，因此必须放置在 `ContextMenu` 内部（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。`ContextMenuTrigger` 是触发打开菜单的区域：在区域内单击右键会在指针位置打开菜单；当区域内的可聚焦元素获得焦点时，按下 `Shift+F10` 或 `ContextMenu` 键会在该元素位置打开菜单。可传入 `open` 与 `on_open_change` 进行受控管理，或传入 `default_open` 默认打开。`ContextMenuRadioGroup` 通过 `value`、`default_value` 与 `on_value_change` 管理单选值，其子项接收 `value`；复选菜单项接收 `checked`，由应用在 `onclick` 中翻转状态。选中的复选菜单项会在左侧内缩区显示勾选图标，选中的单选菜单项则显示圆点：

```rust
rsx! {
  ContextMenu {
    ContextMenuTrigger { class: "h-32 border border-dashed", "Right-click here" }
    ContextMenuContent {
      ContextMenuItem { onclick: move |_| copy(), "Copy" }
    }
  }
}
```

- 菜单左上角对齐在指针位置弹出（`side` 为 `Bottom`、`align` 为 `Start`、`side_offset` 为 `0`），并自动翻转与偏移以保持距离视口边缘至少 8 像素。`Shift+F10` 与 `ContextMenu` 键会在触发区域内当前聚焦元素的中心位置打开菜单：页面脚本会向该元素派发与 `ContextMenu` 键一致的 `contextmenu` 事件，因此后代元素上的监听器能统一接收到两者。Mac 键盘没有 `ContextMenu` 键，且浏览器默认不会为 `Shift+F10` 派发事件。触发区域本身不接收焦点，请在其内部放置可聚焦元素。
- 打开菜单时会自动聚焦第一个可用项。`ArrowDown` 与 `ArrowUp` 移动焦点并在首尾循环（跳过禁用项）；`Home` 与 `End` 跳转到首项与末项；输入字符前缀会聚焦下一个匹配项。
- 在可用菜单项（包括复选和单选菜单项）上按下 `Enter`、`Space` 或点击时，会调用该项的 `onclick` 并随后关闭菜单。禁用项不会调用 `onclick`。
- `Escape`、外部指针按下及焦点移出外部会根据 `dismiss`（默认 `DismissBehavior::popover_default()`）关闭菜单。关闭时焦点会恢复到打开前聚焦的元素，除非焦点已移至其他控件。

目前仅 Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试。

### 子菜单

`ContextMenuSub` 承载嵌套子菜单（[RFC 0067](../../../../docs/rfcs/0067-menu-submenus.md)）并持有其展开状态；可传入 `open` 与 `on_open_change` 进行受控管理：

```rust
rsx! {
  ContextMenuSub {
    ContextMenuSubTrigger { "Share" }
    ContextMenuSubContent {
      ContextMenuItem { onclick: move |_| copy_link(), "Copy link" }
    }
  }
}
```

- 在子菜单触发项上按下 `ArrowRight`（RTL 布局下为 `ArrowLeft`）、`Enter`、`Space` 或点击，会打开子菜单并聚焦其第一项；指针悬停在触发项上也会打开子菜单，但不会移动焦点。
- 在子菜单内按下 `ArrowLeft`（RTL 布局下为 `ArrowRight`）或 `Escape` 会关闭当前层级，并将焦点返回给该子菜单的触发项。
- 选中子菜单内的任意项会关闭所有层级的菜单。高亮外层菜单的其他项或关闭外层菜单时，子菜单也会随之关闭。
- 子菜单在触发项的行内末尾弹出，空间不足时自动翻转。当指针斜向划过同级兄弟菜单项时会立即关闭子菜单（未设置指针宽限过渡区）。

## 无障碍说明

内容容器使用 `role="menu"`。菜单项分别使用 `menuitem`、`menuitemcheckbox` 或 `menuitemradio` 角色，且菜单会在各项之间直接移动 DOM 焦点。子菜单触发项使用 `aria-haspopup="menu"`、`aria-expanded` 与 `aria-controls`，且每个子菜单位于 `group` 中并从其触发项继承无障碍名称。
