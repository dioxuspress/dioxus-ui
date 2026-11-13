## API 概览

- `Steps`
- `StepsOrientation`
- `Step`
- `StepStatus`
- `steps_class`
- `step_class`
- `step_track_class`
- `step_indicator_class`
- `step_label_class`

```rust
rsx! {
  Steps {
    Step { status: StepStatus::Complete, "Cart" }
    Step { status: StepStatus::Current, "Shipping" }
    Step { "Payment" }
  }
}
```

`StepStatus` 可取 `Complete`（主色实心圆）、`Current`（主色描边圆环）或 `Upcoming`（弱化样式，默认值）；连接线在当前步骤之前均显示为主色。步骤列表通过 CSS 计数器自动编号。默认方向为 `StepsOrientation::Horizontal`；设为 `Vertical` 时步骤呈纵向堆叠，标签显示在圆圈一侧。当前处于哪一步由应用自身控制。

## 无障碍说明

`Steps` 渲染为有序列表 `ol`，以便辅助技术朗读步骤总数与先后顺序。当前步骤设置 `aria-current="step"`，已完成步骤会附带视觉隐藏的 `"(completed)"` 文本，而编号圆圈与连接线均标记为 `aria-hidden`。`Steps` 仅用于展示流程进度；请通过应用自身的导航机制确保各步骤内容可访问（参见 [RFC 0059](../../../../docs/rfcs/0059-display-components.md)）。
