## API 概览

- `Loading`
- `LoadingVariant`（`Spinner`、`Dots`、`Ring`、`Ball`、`Bars`、`Infinity`）
- `LoadingSize`（`Xs`、`Sm`、`Md`、`Lg`、`Xl`）
- `loading_class`

```rust
rsx! {
  Loading { variant: LoadingVariant::Dots, size: LoadingSize::Lg, class: "text-primary" }
}
```

## 无障碍说明

`Loading` 容器带有 `role="status"` 与 `aria-label`（默认为 `"Loading"`），内部动画装饰节点均标记为 `aria-hidden="true"`。
