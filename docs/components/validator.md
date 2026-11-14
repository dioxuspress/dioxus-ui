# Validator

Validator wraps form controls to highlight validation state (`Neutral`, `Valid`, `Invalid`) and pair them with an accessible `ValidatorHint` message.

## Source Copy

```bash
dxui add validator
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["validator"] }
```

## API Surface

- `Validator`
- `ValidatorHint`
- `ValidatorState` (`Neutral`, `Valid`, `Invalid`)
- `validator_class`
- `validator_hint_class`

```rust
rsx! {
  Validator { state: ValidatorState::Invalid,
    Input { r#type: "email".to_string(), placeholder: "you@example.com".to_string() }
    ValidatorHint { state: ValidatorState::Invalid, "Please enter a valid email address." }
  }
}
```

## Accessibility Notes

When `ValidatorState::Invalid` is active, `ValidatorHint` automatically sets `role="alert"` so screen readers announce validation errors immediately.
