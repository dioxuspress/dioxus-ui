# Stack

Stack layers child cards or elements on top of one another with subtle depth offsets and opacity steps.

## Source Copy

```bash
dxui add stack
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["stack"] }
```

## API Surface

- `Stack`
- `StackAlign` (`Bottom`, `Top`, `Start`, `End`)
- `stack_class`

```rust
rsx! {
  Stack { align: StackAlign::Bottom, class: "w-64",
    Card { CardBody { "Notification 1" } }
    Card { CardBody { "Notification 2" } }
    Card { CardBody { "Notification 3" } }
  }
}
```

## Accessibility Notes

All stacked children remain in the DOM in source order. Mark decorative background layers with `aria-hidden="true"` if they duplicate content or should not be announced to screen readers.
