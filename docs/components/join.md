# Join

Join groups adjacent controls such as inputs, buttons, and selects into a seamless strip by collapsing inner border radii.

## Source Copy

```bash
dxui add join
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["join"] }
```

## API Surface

- `Join`
- `JoinOrientation` (`Horizontal`, `Vertical`)
- `join_class`
- `join_item_class`

```rust
rsx! {
  Join { orientation: JoinOrientation::Horizontal,
    Input { placeholder: "Search crates..." }
    Button { "Search" }
  }
}
```

## Accessibility Notes

`Join` is a pure layout container that elevates focused child controls (`focus-visible:z-10`) so keyboard focus rings remain visible across shared borders.
