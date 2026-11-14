# Loading

Loading renders animated status indicators across spinner, dots, ring, ball, bars, and infinity styles without JavaScript evaluation.

## Source Copy

```bash
dxui add loading
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["loading"] }
```

## API Surface

- `Loading`
- `LoadingVariant` (`Spinner`, `Dots`, `Ring`, `Ball`, `Bars`, `Infinity`)
- `LoadingSize` (`Xs`, `Sm`, `Md`, `Lg`, `Xl`)
- `loading_class`

```rust
rsx! {
  Loading { variant: LoadingVariant::Dots, size: LoadingSize::Lg, class: "text-primary" }
}
```

## Accessibility Notes

`Loading` exposes `role="status"` and an `aria-label` (default `"Loading"`), while marking internal decorative animation nodes `aria-hidden="true"`.
