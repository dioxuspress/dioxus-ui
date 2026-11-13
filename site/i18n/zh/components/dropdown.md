## API 概览

- `Dropdown`
- `DropdownTrigger`
- `DropdownContent`
- `DropdownGroup`
- `DropdownLabel`
- `DropdownItem`
- `DropdownCheckboxItem`
- `DropdownRadioGroup`
- `DropdownRadioItem`
- `DropdownSeparator`
- `DropdownShortcut`
- `DropdownSub`, `DropdownSubTrigger`, `DropdownSubContent`
- `DropdownPrimitiveConfig`
- `DropdownDismissBehavior`, `DropdownSide`, `DropdownAlign`
- `dropdown_content_class`
- `dropdown_item_class`
- `dropdown_inset_item_class`
- `dropdown_checkbox_item_class`
- `dropdown_radio_item_class`
- `dropdown_shortcut_class`
- `dropdown_sub_trigger_class`

## 行为与交互

`Dropdown` 持有下拉菜单是否打开的状态，内部各子部件均读取该状态，因此必须放置在 `Dropdown` 内部（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。`DropdownTrigger` 用于切换菜单展开状态并作为菜单锚点；它渲染一个无默认样式的 `button`，请通过 `class` 添加样式。可传入 `open` 与 `on_open_change` 进行受控管理，或传入 `default_open` 默认打开。`DropdownRadioGroup` 以同样的方式通过 `value`、`default_value` 与 `on_value_change` 管理单选值，其子项接收 `value`。复选菜单项持有 `checked` 属性：其 `onclick` 负责上报激活事件，由应用翻转状态：

```rust
let mut status_bar = use_signal(|| true);

rsx! {
  Dropdown {
    DropdownTrigger {
      class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
      "View"
    }
    DropdownContent {
      DropdownCheckboxItem {
        checked: status_bar(),
        onclick: move |_| status_bar.toggle(),
        "Status bar"
        DropdownShortcut { "Ctrl+/" }
      }
      DropdownRadioGroup { default_value: "bottom",
        DropdownRadioItem { value: "bottom", "Panel bottom" }
        DropdownRadioItem { value: "right", "Panel right" }
      }
    }
  }
}
```

复选菜单项在选中时显示勾选图标，单选菜单项显示圆点，两者均位于标签文本前的内缩槽位中；普通菜单项可传入 `inset: true` 以保持文本左边缘对齐。

- 菜单采用固定定位（fixed positioning）放置在触发器旁，首选方向空间不足时自动翻转至对侧，偏移以保持在视口之内，并跟随窗口缩放与滚动更新位置。`side_offset` 默认为 `4` 像素。
- `side` 默认为 `Bottom`，`align` 默认为 `End`，与 `DropdownPrimitiveConfig` 保持一致。
- `Escape`、在内容与触发器之外按下指针，以及焦点移出外部时，会根据 `dismiss`（默认 `DismissBehavior::popover_default()`）关闭菜单。
- 打开时会自动聚焦第一个可用项。`ArrowDown` 与 `ArrowUp` 移动焦点并在首尾循环（跳过禁用项）；`Home` 与 `End` 跳转到首项与末项；输入字符前缀会聚焦下一个匹配项。
- 在可用菜单项（包括复选和单选菜单项）上按下 `Enter`、`Space` 或点击时，会调用该项的 `onclick` 并随后关闭菜单。禁用项不会调用 `onclick`。
- 关闭时焦点会恢复到触发器，除非焦点已经移至其他控件。按下 `Tab` 会将焦点移出菜单并随之关闭菜单。

### 子菜单

`DropdownSub` 承载嵌套子菜单（[RFC 0067](../../../../docs/rfcs/0067-menu-submenus.md)）并持有其展开状态；可传入 `open` 与 `on_open_change` 进行受控管理：

```rust
rsx! {
  DropdownSub {
    DropdownSubTrigger { "Share" }
    DropdownSubContent {
      DropdownItem { onclick: move |_| copy_link(), "Copy link" }
    }
  }
}
```

- 在子菜单触发项上按下 `ArrowRight`（RTL 布局下为 `ArrowLeft`）、`Enter`、`Space` 或点击，会打开子菜单并聚焦其第一项；指针悬停在触发项上也会打开子菜单，但不会移动焦点。
- 在子菜单内按下 `ArrowLeft`（RTL 布局下为 `ArrowRight`）或 `Escape` 会关闭当前层级，并将焦点返回给该子菜单的触发项。
- 选中子菜单内的任意项会关闭所有层级的菜单。高亮外层菜单的其他项或关闭外层菜单时，子菜单也会随之关闭。
- 子菜单在触发项的行内末尾弹出，空间不足时自动翻转。当指针斜向划过同级兄弟菜单项时会立即关闭子菜单（未设置指针宽限过渡区）。

Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试，Desktop 渲染器由 `npm run verify:desktop-interactions` 覆盖，iOS 模拟器由 `npm run verify:mobile-interactions` 覆盖，Android 模拟器由 `npm run verify:android-interactions` 覆盖。

## 无障碍说明

内容容器采用 `menu` 语义，菜单项采用 `menuitem`、`menuitemcheckbox` 或 `menuitemradio` 语义并附带 `aria-checked`，使选中状态既能视觉呈现又能被辅助技术播报。子菜单触发项使用 `aria-haspopup="menu"`、`aria-expanded` 与 `aria-controls`，且每个子菜单位于 `group` 中并从其触发项继承无障碍名称。菜单在各项之间直接移动 DOM 焦点，屏幕阅读器会在焦点落到某一项时立即播报。破坏性操作项应提供明确说明动作的文本，切勿仅靠颜色区分。禁用项在 `data-disabled` 之外同时设置 `aria-disabled`（参见 [RFC 0055](../../../../docs/rfcs/0055-open-state-accessibility-audit.md)）。
