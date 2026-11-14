## API 概览

- `Validator`
- `ValidatorHint`
- `ValidatorState`（`Neutral`、`Valid`、`Invalid`）
- `validator_class`
- `validator_hint_class`

```rust
rsx! {
  Validator { state: ValidatorState::Invalid,
    Input { r#type: "email".to_string(), placeholder: "you@example.com".to_string() }
    ValidatorHint { state: ValidatorState::Invalid, "请输入有效的邮箱地址。" }
  }
}
```

## 无障碍说明

当处于 `ValidatorState::Invalid` 状态时，`ValidatorHint` 会自动附加 `role="alert"`，确保错误提示能被屏幕阅读器及时播报。
