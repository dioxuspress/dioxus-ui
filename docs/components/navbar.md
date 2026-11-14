# Navbar

Navbar provides a top-level horizontal navigation bar with `NavbarStart`, `NavbarCenter`, and `NavbarEnd` slots.

## Source Copy

```bash
dxui add navbar
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["navbar"] }
```

## API Surface

- `Navbar`
- `NavbarStart`
- `NavbarCenter`
- `NavbarEnd`
- `navbar_class`
- `navbar_start_class`
- `navbar_center_class`
- `navbar_end_class`

```rust
rsx! {
  Navbar {
    NavbarStart { span { class: "font-semibold", "DioxusPress" } }
    NavbarCenter { a { href: "/docs", "Docs" } }
    NavbarEnd { Button { "Sign in" } }
  }
}
```

## Accessibility Notes

`Navbar` renders a semantic `<header>` landmark. Place primary navigation links inside `<nav>` elements or use `aria-label` when rendering multiple navigation regions.
