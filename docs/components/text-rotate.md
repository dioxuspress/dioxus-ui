# Text Rotate

Text Rotate cycles vertically through a list of phrases with slide-and-fade transitions, pausing on hover by default.

## Source Copy

```bash
dxui add text-rotate
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["text-rotate"] }
```

## API Surface

- `TextRotate`
- `text_rotate_class`

```rust
rsx! {
  h2 { class: "text-2xl font-bold",
    "Build "
    TextRotate {
      class: "text-primary",
      items: vec!["faster".to_string(), "safer".to_string(), "accessible".to_string()],
    }
    " apps"
  }
}
```

## Accessibility Notes

Inactive phrases are marked `aria-hidden="true"` so screen readers announce only the currently visible phrase. Set `index: Some(...)` for controlled playback or reduced-motion scenarios.
