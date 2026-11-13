## API 概览

- `Button`
- `ButtonVariant`
- `ButtonSize`
- `button_class`
- `DensityProvider`, `use_density`, `UiDensity`

`ButtonVariant::Link` 使用前景色渲染文本，并在悬停时显示主色下划线，使链接按钮在使用浅色主色的主题预设下依然保持清晰可读（参见 [RFC 0057](../../../../docs/rfcs/0057-theme-presets.md)）。

## 事件处理

```rust
rsx! {
  Button { onclick: move |_| save(), "Save" }
  Button { r#type: "button", "aria-label": "Close", onclick: move |_| close(), "×" }
}
```

点击、`Enter` 或 `Space` 会携带鼠标事件调用 `onclick`。其他属性（例如 `type`、`name` 与 `aria-label`）会透传至底层按钮元素。在表单内部，按钮默认保留原生的 `submit` 类型；若不希望触发表单提交，请传入 `r#type: "button"`。

## 触控密度

`Button` 从最近的 `DensityProvider` 获取密度配置（参见 [RFC 0078](../../../../docs/rfcs/0078-touch-density.md)），未包裹时默认为 `Comfortable`。在 `Touch` 密度下，按钮高度至少为 48 CSS 像素、宽度至少为 44 CSS 像素，以满足触控目标尺寸要求；在 `Comfortable` 密度下，高度仅由尺寸决定（例如 `Sm` 为 32 像素）：

```rust
rsx! {
  DensityProvider { density: UiDensity::Touch,
    Button { "Save" }
  }
}
```

以按钮样式呈现的其他元素（例如浮层触发器）可以将相同的密度传入 `button_class(variant, size, use_density(), "")`。所有交互式组件均以同样方式遵循该 `DensityProvider`。

## 无障碍说明

对于不可用的操作请使用 `disabled`。仅包含图标的按钮应由业务应用提供无障碍标签（例如 `aria-label`）。
