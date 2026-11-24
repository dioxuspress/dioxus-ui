## API 概览

- `Breadcrumb`
- `BreadcrumbList`
- `BreadcrumbItem`
- `BreadcrumbLink`
- `BreadcrumbPage`
- `BreadcrumbSeparator`
- `BreadcrumbEllipsis`
- 各子部件对应的样式辅助函数

## 无障碍说明

- `Breadcrumb` 渲染默认 `aria-label="breadcrumb"` 的导航区域（`nav`）；当同页存在多个面包屑时可通过透传 `aria-label` 属性覆盖。
- `BreadcrumbList` 渲染有序列表（`ol`）。
- `BreadcrumbLink { current: true }` 与 `BreadcrumbPage` 均会暴露 `aria-current="page"`。
- `BreadcrumbLink` 会透传 `title` 和 `target` 等链接属性。
- 路由集成以及自定义图标的视觉隐藏标签仍由应用自行管理。
