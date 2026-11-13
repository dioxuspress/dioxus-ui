## API 概览

- `Fab`
- `FabAction`
- `fab_class`
- `fab_action_class`

```rust
rsx! {
  // A plain button.
  Fab { "aria-label": "Compose", icon: rsx! { PlusIcon {} }, onclick: move |_| compose() }

  // A speed dial.
  Fab {
    "aria-label": "Create",
    icon: rsx! { PlusIcon {} },
    FabAction { label: "Photo", onclick: move |_| take_photo(), CameraIcon {} }
    FabAction { label: "Note", onclick: move |_| new_note(), NoteIcon {} }
  }
}
```

按钮默认固定在屏幕底部的内联末端（inline-end）角落、主屏幕指示条上方；传入 `fixed: false` 可使其保留在常规文档流中。不包含子元素时，它表现为普通按钮并直接执行 `onclick`。包含 `FabAction` 子元素时，它会作为快捷拨号菜单（speed dial）自行管理展开状态（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）：按下主按钮会切换展开与收起，展开时操作项垂直堆叠在主按钮上方，按下某个操作项会执行其 `onclick` 并收起菜单。可传入 `open` 和 `on_open_change` 进行受控管理，或传入 `default_open` 使其初始展开；每次状态变化都会通知 `on_open_change`。`icon` 用于指定主按钮的内容。

## 无障碍说明

请使用 `aria-label` 为主按钮命名。作为快捷拨号菜单时，按钮会自动设置 `aria-expanded` 和 `aria-controls`；操作项分组仅在展开时渲染，因此收起时不会进入 `Tab` 焦点顺序；按 `Escape` 会关闭菜单并将焦点移回主按钮。每个 `FabAction` 都会显示其 `label`，并以此作为自身的无障碍名称（参见 [RFC 0061](../../../../docs/rfcs/0061-mobile-navigation.md)）。
