## API 概览

- `ToggleGroup`
- `ToggleGroupItem`
- `ToggleGroupType`
- `toggle_group_class`
- `toggle_group_item_class`
- `NavigationOrientation`
- `RovingFocusItem`
- `FocusMove`

## 行为与交互

`ToggleGroup` 内部维护当前处于按下状态的子项（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。在默认的单选模式下，最多只有一个子项处于按下状态，可通过 `default_value` 指定初始值，或通过 `value` 进行受控管理；空字符串表示未选中任何项。每次变更都会通知 `on_value_change`，包括已按下项被释放时传入 `""`：

```rust
rsx! {
  ToggleGroup { "aria-label": "Alignment", default_value: "left",
    ToggleGroupItem { value: "left", "Left" }
    ToggleGroupItem { value: "right", "Right" }
  }
}
```

当设置 `selection_type: ToggleGroupType::Multiple` 时，允许任意数量的子项同时处于按下状态，此时分组改用 `values`、`default_values` 和 `on_values_change`。

- 所有子项共同构成一个 `Tab` 焦点停靠点：优先聚焦上次拥有焦点的项，其次为第一个已按下的项，否则为第一个可用项。
- 与 `orientation` 对应的方向键会在可用项之间移动焦点而不会直接按下它们，开启 `looping` 时支持首尾循环；默认值 `Both` 同时响应四个方向键。`Home` 和 `End` 分别跳转到第一项和最后一项。
- 在从右到左（RTL）布局下，`ArrowLeft` 移动到下一项，`ArrowRight` 移动到上一项；`Up` 和 `Down` 行为不变。
- 在子项上点击、按 `Enter` 或 `Space` 会将其按下，若已处于按下状态则将其释放。
- 位于 `ToggleGroup` 外部的 `ToggleGroupItem` 不会渲染任何内容，并会在日志中提示缺失的根组件。

Web 渲染器的交互行为已由 `npm run verify:runtime-interactions` 覆盖验证。

## 无障碍说明

`ToggleGroup` 渲染 `role="group"`，子项渲染带有 `aria-pressed` 的按钮。当同一时刻只能激活一项时使用单选模式，当每一项都可独立开关时使用多选模式。方向键会自动遵循文本书写方向（参见 [RFC 0025](../../../../docs/rfcs/0025-right-to-left-arrow-mirroring.md)）。

`ToggleGroup` 支持属性透传，请使用 `aria-label` 或 `aria-labelledby` 为分组命名（参见 [RFC 0040](../../../../docs/rfcs/0040-composite-widget-names.md)）。由于 `role="group"` 不支持 `aria-orientation`，分组仅通过 `data-orientation` 暴露其排列方向。
