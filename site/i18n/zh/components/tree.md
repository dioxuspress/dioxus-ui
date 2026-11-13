## API 概览

- `Tree`
- `TreeItem`
- `tree_class`
- `tree_group_class`
- `tree_item_row_class`

## 行为与交互

`Tree` 负责管理当前展开的分支列表以及当前选中的节点（[RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。可通过 `default_expanded` 与 `default_selected` 设置初始状态，或通过 `expanded` 与 `selected` 进行受控管理。每次状态变更后，`on_expanded_change` 会收到最新展开的分支列表，`on_selected_change` 会收到当前选中的节点值。

`TreeItem` 的子元素即为其显示标签；向 `group` 属性传入子节点列表即可将其变为可展开分支：

```rust
rsx! {
  Tree { "aria-label": "Project files", default_expanded: vec!["src".to_string()],
    TreeItem {
      value: "src",
      group: rsx! {
        TreeItem { value: "main", "main.rs" }
      },
      "src"
    }
    TreeItem { value: "cargo", "Cargo.toml" }
  }
}
```

- 所有可见节点共同构成单一 `Tab` 停靠点，优先落在当前选中节点上，其次是上次拥有焦点的节点，再次为第一个可见节点。
- `ArrowUp` 与 `ArrowDown` 在可见节点之间移动焦点，`Home` 与 `End` 分别跳至首个与最后一个可见节点。直接输入字符会跳至标签以所输入文本开头的下一个节点；重复按下同一字母会在以该字母开头的节点之间依次切换。
- `ArrowRight` 会展开已收起的分支，或进入已展开分支的第一个子节点；`ArrowLeft` 会收起已展开的分支，或退回至父节点。在从右到左（RTL）布局中，两者的方向互换。
- 按 `Enter` 或 `Space` 会选中当前聚焦的节点。鼠标点击会选中节点，并在节点为分支时同步切换展开或收起状态。
- 禁用节点在键盘导航时会被跳过，且无法被选中。
- 每深入一层级，行内容缩进增加 `1rem`；`class` 用于设置节点行本身的样式，`group_class` 用于设置其子节点分组容器的样式。
- 放置在 `Tree` 外部的 `TreeItem` 不会渲染任何内容，并会在日志中提示缺少的根组件。

Web 渲染器的交互验证由 `npm run verify:runtime-interactions` 覆盖。

## 无障碍说明

`Tree` 渲染 `role="tree"`，每个节点渲染 `role="treeitem"` 并附带 `aria-level`、`aria-selected`，分支节点还会额外设置 `aria-expanded`；分支的子节点包裹在 `role="group"` 容器中，并在分支收起时设置 `hidden`。焦点落在节点元素本身，焦点环与选中高亮则呈现在节点行上。请使用 `aria-label` 或 `aria-labelledby` 为树组件命名（参见 [RFC 0040](../../../../docs/rfcs/0040-composite-widget-names.md)）。多选模式与按 `*` 展开同级所有分支的功能暂未实现。
