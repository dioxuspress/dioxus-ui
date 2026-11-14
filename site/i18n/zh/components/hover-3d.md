## API 概览

- `Hover3D`
- `Hover3dIntensity`（`Subtle`、`Default`、`Strong`）
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

## 无障碍说明

3×3 指针感应网格已标记 `aria-hidden="true"`，不会引入键盘焦点陷阱。建议将 `Hover3D` 用于展示型卡片，或将主要交互按钮放在倾斜遮罩层之外。
