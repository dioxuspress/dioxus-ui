# Hover Gallery

Hover Gallery displays a primary image and switches across frames as the pointer moves across horizontal slices.

## Source Copy

```bash
dxui add hover-gallery
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["hover-gallery"] }
```

## API Surface

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

## Accessibility Notes

Each horizontal slice renders a keyboard-focusable `<button>` with `aria-label="Show image X of Y"` and `aria-pressed`, so keyboard and touch users can switch frames without a mouse.
