# Link

Link renders styled anchor elements with semantic color variants, underline modes, and automatic unsafe URL filtering.

## Source Copy

```bash
dxui add link
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["link"] }
```

## API Surface

- `Link`
- `LinkVariant` (`Default`, `Primary`, `Secondary`, `Accent`, `Neutral`, `Muted`, `Success`, `Warning`, `Destructive`)
- `LinkUnderline` (`Always`, `Hover`, `None`)
- `link_class`

```rust
rsx! {
  Link {
    href: "https://dioxuspress.dev".to_string(),
    variant: LinkVariant::Primary,
    underline: LinkUnderline::Hover,
    external: true,
    "Read the documentation"
  }
}
```

## Accessibility Notes

`Link` renders a native `<a>` element with `focus-visible` ring styles, filters out scriptable URL schemes such as `javascript:` or `data:`, and adds `rel="noopener noreferrer"` when `external: true`.
