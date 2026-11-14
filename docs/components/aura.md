# Aura

Aura wraps content with a soft, semantic-token ambient glow backdrop for spotlighting hero cards, badges, or feature panels.

## Source Copy

```bash
dxui add aura
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["aura"] }
```

## API Surface

- `Aura`
- `AuraTone` (`Primary`, `Secondary`, `Accent`, `Destructive`)
- `AuraSize` (`Sm`, `Md`, `Lg`)
- `aura_class`

```rust
rsx! {
  Aura { tone: AuraTone::Primary, size: AuraSize::Lg,
    Card {
      CardBody { "Featured highlight" }
    }
  }
}
```

## Accessibility Notes

The blurred glow layers are marked `aria-hidden="true"` and `pointer-events-none` so they never interfere with assistive technology or pointer interaction.
