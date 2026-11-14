# Hover 3d

Hover 3D wraps card or showcase content in a 3D perspective container that tilts smoothly toward whichever of the 9 pointer zones is hovered.

## Source Copy

```bash
dxui add hover-3d
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["hover-3d"] }
```

## API Surface

- `Hover3D`
- `Hover3dIntensity` (`Subtle`, `Default`, `Strong`)
- `hover_3d_class`
- `hover_3d_surface_class`
- `hover_3d_transform`

```rust
rsx! {
  Hover3D { intensity: Hover3dIntensity::Default, class: "w-72",
    Card {
      CardBody {
        CardTitle { "3D Tilt Card" }
        p { "Move your pointer across the surface." }
      }
    }
  }
}
```

## Accessibility Notes

The 3×3 pointer zone grid is marked `aria-hidden="true"` and does not add keyboard traps. Use `Hover3D` for non-interactive showcase cards or keep primary actions outside the tilt overlay.
