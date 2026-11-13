## API 概览

- `Diff`
- `DiffBefore`
- `DiffAfter`
- `diff_class`
- `diff_layer_class`

```rust
let mut position = use_signal(|| 50.0);

rsx! {
  Diff {
    class: "aspect-video",
    position: position(),
    on_position_change: move |value| position.set(value),
    DiffBefore { img { src: "/before.jpg", alt: "Before retouching" } }
    DiffAfter { img { src: "/after.jpg", alt: "After retouching" } }
  }
}
```

`DiffBefore` 显示在分割线左侧，`DiffAfter` 显示在分割线右侧，分割位置为 `position` 百分比（默认为 `50`，限制在 `0` 到 `100` 之间）。该组件为受控组件：移动分割线会调用 `on_position_change`，再由应用将更新后的值传回。图层内的图片会填满容器框，请为外层容器指定尺寸（例如宽高比）。

## 无障碍说明

组件内部使用一个透明且覆盖整个容器框的原生 `range` 输入控件来移动分割线，因此原生支持拖拽、点击以及方向键、`Home` 和 `End` 键操作，辅助技术也会将其识别并播报为滑块。可通过 `label` 指定其无障碍名称（默认为 `"Comparison position"`）。拖拽手柄会显示焦点环。请为两侧图片设置明确的 `alt` 文本以标明各自对应的版本（参见 [RFC 0059](../../../../docs/rfcs/0059-display-components.md)）。
