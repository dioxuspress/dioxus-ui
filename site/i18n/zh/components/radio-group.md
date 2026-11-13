## API 概览

- `RadioGroup`
- `RadioGroupItem`
- `radio_group_class`
- `radio_group_item_class`
- `radio_group_move_value`
- `NavigationOrientation`
- `RovingFocusItem`
- `FocusMove`

## 行为与交互

`RadioGroup` 内部维护当前选中的值（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。可通过 `default_value` 设置初始选中值，或通过 `value` 进行受控管理；无论哪种模式，用户触发的每次变更都会通知 `on_value_change`。各个单选项会将自身的 `value` 与分组当前值进行比对：

```rust
rsx! {
  RadioGroup { default_value: "small",
    RadioGroupItem { value: "small", "aria-label": "Small" }
    RadioGroupItem { value: "large", "aria-label": "Large" }
  }
}
```

- 所有子项共同构成一个 `Tab` 焦点停靠点：优先聚焦当前选中的项，若均未选中则聚焦第一个可用项。
- 与 `orientation` 对应的方向键会在可用项之间移动焦点，开启 `looping` 时支持首尾循环；默认值 `Both` 同时响应四个方向键。`Home` 和 `End` 分别跳转到第一项和最后一项。移动焦点时会同时选中新获得焦点的项。
- 在从右到左（RTL）布局下，`ArrowLeft` 移动到下一项，`ArrowRight` 移动到上一项；`Up` 和 `Down` 行为不变。
- 点击某个子项或在子项上按 `Space` 会将其选中。
- 位于 `RadioGroup` 外部的 `RadioGroupItem` 不会渲染任何内容，并会在日志中提示缺失的根组件。

Web 渲染器的交互行为已由 `npm run verify:runtime-interactions` 覆盖验证。`radio_group_move_value` 和 `radio_group_item_tabindex` 仍保留导出，供自行渲染单选项的应用使用。

## 无障碍说明

`RadioGroup` 渲染 `role="radiogroup"`，子项渲染带有 `aria-checked` 的 `role="radio"`。方向键会自动遵循文本书写方向（参见 [RFC 0025](../../../../docs/rfcs/0025-right-to-left-arrow-mirroring.md)）。

单选项自身不包含可见文本，因此必须为每一项提供名称。`RadioGroup` 和 `RadioGroupItem` 均支持透传 `id`、`aria-label` 与 `aria-labelledby` 等属性（参见 [RFC 0038](../../../../docs/rfcs/0038-form-control-naming.md)）：

```rust
rsx! {
  Label { id: "size-label", "Size" }
  RadioGroup { "aria-labelledby": "size-label", default_value: "small",
    RadioGroupItem { id: "size-small", value: "small" }
    Label { r#for: "size-small", "Small" }
  }
}
```
