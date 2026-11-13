## API 概览

- `Collapsible`
- `CollapsibleTrigger`
- `CollapsibleContent`
- `collapsible_class`
- `collapsible_trigger_class`
- `collapsible_content_class`

## 展开与收起事件

```rust
rsx! {
  Collapsible {
    CollapsibleTrigger { "Details" }
    CollapsibleContent { "More information" }
  }
}
```

`Collapsible` 默认处于收起状态，也可通过 `default_open: true` 默认展开。传入 `open` 可进入受控模式（例如在触发器文本中同步展示当前状态）；无论受控与否，`on_open_change` 都会监听用户的每次状态切换。在 `CollapsibleTrigger` 上点击或按 `Enter`、`Space` 会切换根组件的展开状态。在根组件或触发器上设置 `disabled` 均可禁用触发器。

处于收起状态时，`CollapsibleContent` 默认不渲染任何节点。设置 `force_mount: true`（默认为 `false`）可使其保留在 DOM 中并处于隐藏状态。三个部件均会将 `aria-label`、`data-*` 等其他属性透传给底层元素。触发器与内容区必须位于 `Collapsible` 内部；否则不会渲染任何内容，并会在日志中提示缺少的根组件。

## 无障碍说明

`CollapsibleTrigger` 渲染为带有 `aria-expanded` 的原生按钮，并在内容区显示时通过 `aria-controls` 指向内容区；关联所需的 ID 由根组件自动生成。动画时长与高度测量过渡效果由应用自行控制。
