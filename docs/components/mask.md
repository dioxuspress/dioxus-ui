# Mask

Mask crops images, avatars, or decorative containers into geometric shapes using CSS `clip-path`.

## Source Copy

```bash
dxui add mask
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["mask"] }
```

## API Surface

- `Mask`
- `MaskShape` (`Squircle`, `Circle`, `Square`, `Diamond`, `Triangle`, `Triangle2`, `Triangle3`, `Triangle4`, `Pentagon`, `Hexagon`, `Hexagon2`, `Decagon`, `Star`, `Star2`, `Heart`)
- `MaskHalf` (`None`, `First`, `Second`)
- `mask_class`
- `mask_style`

```rust
rsx! {
  Mask { shape: MaskShape::Heart, class: "size-16 bg-primary/15",
    img { src: "/avatar.png", alt: "Profile", class: "size-full object-cover" }
  }
}
```

## Accessibility Notes

Mask is purely visual and adds no ARIA role. Provide meaningful `alt` text on images inside the mask or `aria-hidden="true"` when decorative.
