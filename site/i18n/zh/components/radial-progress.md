## API 概览

- `RadialProgress`
- `RadialProgressSize`
- `radial_progress_class`

```rust
rsx! {
  RadialProgress { value: 72.0, "aria-label": "Storage used" }
  RadialProgress { value: 3.0 / 5.0 * 100.0, "aria-label": "Tasks done", "3/5" }
}
```

`value` 会被限制在 `0` 到 `100` 之间。传入子元素会替换居中的百分比文本。`RadialProgressSize` 支持 `Sm`、`Md`（默认）和 `Lg`。可通过样式类（如 `[&_circle:last-of-type]:stroke-success`）自定义圆环颜色。

## 无障碍说明

组件渲染为 `role="progressbar"`，并带有 `aria-valuemin="0"`、`aria-valuemax="100"` 以及四舍五入后的 `aria-valuenow`；圆环图形本身标记为 `aria-hidden`。请通过 `aria-label` 或 `aria-labelledby` 为其提供无障碍名称（参见 [RFC 0059](../../../../docs/rfcs/0059-display-components.md)）。
