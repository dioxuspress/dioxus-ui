# Filter

Filter renders a pill-based radio group with an automatic reset button (`×`) whenever a non-empty option is selected.

## Source Copy

```bash
dxui add filter
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["filter"] }
```

## API Surface

- `Filter`
- `FilterOption`
- `filter_class`
- `filter_item_class`
- `filter_reset_class`

```rust
let mut selected = use_signal(|| "rust".to_string());

rsx! {
  Filter {
    options: vec![
      FilterOption::new("rust", "Rust"),
      FilterOption::new("wasm", "WASM"),
      FilterOption::new("ssr", "SSR"),
    ],
    value: Some(selected()),
    on_change: move |next| selected.set(next),
  }
}
```

## Accessibility Notes

`Filter` uses `role="radiogroup"` with `role="radio"` and `aria-checked` on each option button. The reset button includes a descriptive `aria-label` (`"Clear filter"` by default).
