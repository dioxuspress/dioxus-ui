## API 概览

- `Mask`
- `MaskShape`（`Squircle`、`Circle`、`Square`、`Diamond`、`Triangle`、`Triangle2`、`Triangle3`、`Triangle4`、`Pentagon`、`Hexagon`、`Hexagon2`、`Decagon`、`Star`、`Star2`、`Heart`）
- `MaskHalf`（`None`、`First`、`Second`）
- `mask_class`
- `mask_style`

```rust
rsx! {
  Mask { shape: MaskShape::Heart, class: "size-16 bg-primary/15",
    img { src: "/avatar.png", alt: "Profile", class: "size-full object-cover" }
  }
}
```

## 无障碍说明

`Mask` 仅提供纯视觉裁剪，不会添加额外 ARIA 角色。请为内部图片提供准确的 `alt` 文本，若仅作装饰用途则标记 `aria-hidden="true"`。
