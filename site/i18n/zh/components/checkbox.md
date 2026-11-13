## API 概览

- `Checkbox`
- `checkbox_class`

## 变更事件

```rust
let mut terms = use_signal(|| false);

rsx! {
  Checkbox {
    id: "terms",
    name: "terms",
    checked: terms(),
    on_checked_change: move |checked| terms.set(checked),
  }
  Label { r#for: "terms", "Accept terms" }
}
```

`Checkbox` 为受控组件。点击复选框、按 `Space` 或点击关联的 `Label` 都会以目标状态 `!checked` 调用 `on_checked_change`，再由应用将其回传给 `checked`。禁用状态下的复选框不会触发该回调。

其他属性（例如 `id`、`name`、`value` 与 `aria-describedby`）会透传至底层 `input` 元素。

## 半选状态

```rust
let mut items = use_signal(|| [true, false]);
let all = items().iter().all(|item| *item);
let some = items().iter().any(|item| *item);

rsx! {
  Checkbox {
    "aria-label": "Select all",
    checked: all,
    indeterminate: some && !all,
    on_checked_change: move |checked| items.set([checked, checked]),
  }
}
```

`indeterminate` 会设置底层输入框的原生 `indeterminate` DOM 属性（浏览器会将其播报为混合/半选复选框），并渲染 `data-state="indeterminate"`。处于半选状态时触发变更会请求切换为 `true`。浏览器在用户点击时会自动清除原生 `indeterminate` 属性；若应用状态仍保持半选，组件会在下一次渲染后重新设置该属性。由于该 DOM 属性需在浏览器端设置，服务端渲染（SSR）生成的初始 HTML 在水合（hydration）完成前会将该复选框暴露为未选中状态（参见 [RFC 0041](../../../../docs/rfcs/0041-checkbox-indeterminate-state.md)）。

输入框通过 `appearance-none` 自行绘制选框外观：选中状态在主色填充背景上显示对勾，半选状态显示短横线。由于视觉标记基于 `data-state` 驱动，服务端渲染输出的 HTML 即可直接呈现正确外观（参见 [RFC 0045](../../../../docs/rfcs/0045-drawn-checkbox.md)）。每个标记均通过 `::before` 伪元素配合形状遮罩并使用 `--primary-foreground` 填充，因此能自动适配深色主题与所有主题预设（参见 [RFC 0057](../../../../docs/rfcs/0057-theme-presets.md)）。

## 无障碍说明

请为复选框配对可见标签或无障碍名称。将选中状态保存在应用状态中，以确保表单提交与键盘交互行为始终可预测。
