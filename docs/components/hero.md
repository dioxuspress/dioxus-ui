# Hero

Hero renders a full-width showcase section with centered content and an optional `HeroOverlay` backdrop.

## Source Copy

```bash
dxui add hero
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["hero"] }
```

## API Surface

- `Hero`
- `HeroOverlay`
- `HeroContent`
- `hero_class`
- `hero_overlay_class`
- `hero_content_class`

```rust
rsx! {
  Hero {
    HeroOverlay {}
    HeroContent {
      div {
        h1 { class: "text-3xl font-bold", "Build Full-Stack Rust Apps" }
        p { class: "mt-2 text-muted-foreground", "Ship faster with Dioxus 0.7 components." }
      }
    }
  }
}
```

## Accessibility Notes

`Hero` renders a `<section>` container and marks `HeroOverlay` with `aria-hidden="true"` so decorative backdrops are ignored by assistive technologies.
