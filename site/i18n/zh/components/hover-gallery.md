## API 概览

- `HoverGallery`
- `HoverGalleryImage`
- `hover_gallery_class`

```rust
rsx! {
  HoverGallery {
    class: "aspect-square w-64",
    images: vec![
      HoverGalleryImage::new("/product-1.jpg", "Front view"),
      HoverGalleryImage::new("/product-2.jpg", "Side view"),
      HoverGalleryImage::new("/product-3.jpg", "Detail view"),
    ],
  }
}
```

## 无障碍说明

每个横向触发区域都会渲染为一个可键盘聚焦的 `<button>`，并带有 `aria-label="Show image X of Y"` 与 `aria-pressed` 状态，确保键盘和触屏用户也能顺畅切换预览图。
