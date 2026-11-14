## API 概览

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

## 无障碍说明

`Hero` 使用 `<section>` 语义容器，并将装饰性的 `HeroOverlay` 标记为 `aria-hidden="true"`，避免干扰屏幕阅读器。
