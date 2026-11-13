## API 概览

- `ScrollArea`
- `ScrollAreaViewport`
- `ScrollAreaContent`
- `ScrollAreaScrollbar`
- `ScrollAreaThumb`
- `ScrollAreaCorner`
- `ScrollAreaOrientation`
- `scroll_area_orientation_attribute`

## 无障碍说明

视口（viewport）保留原生滚动行为，并带有 `tabindex="0"` 与焦点环以作为 `Tab` 停靠点，确保纯键盘用户也能滚动其内容（WebKit 自身默认不会将滚动容器设为可聚焦）。请为其设置 `role: "region"` 与 `aria-label` 进行命名；若其内部已经包含可聚焦元素，也可传入 `tabindex: "-1"`。自定义样式的滚动条部件仅作为视觉呈现钩子，对辅助技术隐藏。焦点管理、滚动位置恢复以及平台特定的滚动条行为仍由应用自行负责。
