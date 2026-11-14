# Footer

Footer provides a semantic `<footer>` layout with `FooterLayout::Columns` and `FooterLayout::Centered` modes, `FooterNav` columns, and `FooterTitle` headings.

## Source Copy

```bash
dxui add footer
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["footer"] }
```

## API Surface

- `Footer`
- `FooterLayout` (`Columns`, `Centered`)
- `FooterNav`
- `FooterTitle`
- `footer_class`
- `footer_nav_class`
- `footer_title_class`

```rust
rsx! {
  Footer {
    FooterNav {
      FooterTitle { "Product" }
      a { href: "/components", "Components" }
      a { href: "/themes", "Themes" }
    }
    FooterNav {
      FooterTitle { "Resources" }
      a { href: "/docs", "Docs" }
    }
  }
}
```

## Accessibility Notes

`Footer` uses a semantic `<footer>` element, while `FooterNav` and `FooterTitle` use non-landmark containers so multiple footer columns never clash with page heading hierarchy or `<nav>` landmark uniqueness.
