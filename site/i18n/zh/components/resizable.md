## API 概览

- `ResizablePanelGroup`
- `ResizablePanel`
- `ResizableHandle`
- `LayoutOrientation`
- `ResizablePanelState`
- `resizable_clamp`
- `resizable_resize_pair`

## 尺寸调节

`ResizableHandle` 通过 `value` 接收其前方相邻面板的当前尺寸及上下限，并通过 `on_resize` 以百分比增量的形式上报拖拽或按键调节量。可配合 `resizable_resize_pair` 应用尺寸变更：

```rust
let mut panels = use_signal(|| {
  (ResizablePanelState::new(30.0, 20.0, 80.0), ResizablePanelState::new(70.0, 20.0, 80.0))
});

rsx! {
  ResizablePanelGroup {
    ResizablePanel { id: "files", size: panels().0.size, "Files" }
    ResizableHandle {
      "aria-controls": "files",
      "aria-label": "Resize files",
      value: panels().0.size,
      min: 20.0,
      max: 80.0,
      on_resize: move |delta| {
        let (first, second) = panels();
        panels.set(resizable_resize_pair(first, second, delta));
      },
    }
    ResizablePanel { size: panels().1.size, "Editor" }
  }
}
```

- 未禁用时，调节手柄是一个 `Tab` 停靠点。在水平分组中按 `ArrowRight` 与 `ArrowLeft` 会按 `step`（默认为 10）调整尺寸；在垂直分组中则使用 `ArrowDown` 与 `ArrowUp`。按 `Home` 与 `End` 会直接跳至 `min` 与 `max`。
- 按住鼠标主按键拖拽时会捕获指针并持续跟踪位移，位移比例相对于手柄的父容器计算，因此 `ResizableHandle` 必须是 `ResizablePanelGroup` 的直接子元素。当面板尺寸触及上下限停止缩放后，分割边线在指针回移时仍会与指针位置保持对齐。
- `aria-valuenow`、`aria-valuemin` 与 `aria-valuemax` 会同步映射 `value`、`min` 与 `max`。请通过 `aria-controls` 传入目标面板的 `id`，并设置 `aria-label`。
- 禁用的手柄不会上报任何尺寸变化，也会移出 `Tab` 焦点顺序。
- 分组容器、各面板及手柄均支持透传全局属性与元素属性。

## 无障碍说明

调节手柄遵循 APG 窗口分割器（window splitter）模式：渲染为带有数值属性的可聚焦分隔条。`aria-orientation` 描述的是分割线本身的走向，因此在左右并排的面板之间其值为 `vertical`。从右到左（RTL）分组布局、键盘一键折叠以及布局状态持久化由应用自行负责。
