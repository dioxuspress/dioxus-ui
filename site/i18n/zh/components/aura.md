## API 概览

- `Aura`
- `AuraTone`（`Primary`、`Secondary`、`Accent`、`Destructive`）
- `AuraSize`（`Sm`、`Md`、`Lg`）
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

## 无障碍说明

背景模糊光晕层已标记 `aria-hidden="true"` 与 `pointer-events-none`，不会干扰屏幕阅读器或指针事件交互。
