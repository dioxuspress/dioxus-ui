# Megamenu

Megamenu renders a toggleable multi-column navigation panel with keyboard Escape dismissal and safe URL filtering.

## Source Copy

```bash
dxui add megamenu
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["megamenu"] }
```

## API Surface

- `Megamenu`
- `MegamenuTrigger`
- `MegamenuContent`
- `MegamenuColumn`
- `MegamenuHeading`
- `MegamenuLink`
- `megamenu_class`
- `megamenu_trigger_class`
- `megamenu_content_class`
- `megamenu_column_class`
- `megamenu_heading_class`
- `megamenu_link_class`

```rust
rsx! {
  Megamenu {
    MegamenuTrigger { "Components" }
    MegamenuContent {
      MegamenuColumn {
        MegamenuHeading { "Layout" }
        MegamenuLink { href: "/components/navbar".to_string(), "Navbar" }
        MegamenuLink { href: "/components/hero".to_string(), "Hero" }
      }
    }
  }
}
```

## Accessibility Notes

`MegamenuTrigger` exposes `aria-expanded` and `aria-haspopup="true"`. Pressing `Escape` inside the menu closes `MegamenuContent`, and `MegamenuLink` filters unsafe URL schemes via `safe_href`.
