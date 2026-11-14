## API 概览

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

## 无障碍说明

`Filter` 使用 `role="radiogroup"` 与 `role="radio"` 标记选项按钮，并通过 `aria-checked` 同步选中状态；重置按钮默认带有 `"Clear filter"` 的 `aria-label`。
