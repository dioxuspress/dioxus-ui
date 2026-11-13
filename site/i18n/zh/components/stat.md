## API 概览

- `StatGroup`
- `StatGroupOrientation`
- `Stat`
- `StatTitle`
- `StatValue`
- `StatDescription`
- `StatFigure`
- `stat_group_class`
- `stat_class`
- `stat_title_class`
- `stat_value_class`
- `stat_description_class`
- `stat_figure_class`

```rust
rsx! {
  StatGroup {
    Stat {
      StatTitle { "Revenue" }
      StatValue { "$45,231" }
      StatDescription { "+20% from last month" }
    }
  }
}
```

默认的 `StatGroupOrientation::Horizontal` 会在统计项之间渲染分隔线，并在窄宽度下支持横向滚动；`Vertical` 则将其垂直堆叠。无论 `StatFigure` 在源码中处于什么位置，都会占据跨行的第二列。

## 无障碍说明

`StatGroup` 渲染为 `dl`，每个 `Stat` 为其中的 `div`，`StatTitle` 为 `dt`，而数值、描述与配图均为 `dd`，从而让辅助技术将每个数值与其标题关联朗读。请确保将 `Stat` 放置在 `StatGroup` 内，并将标题放在首位。装饰性的配图图标请标记为 `aria-hidden`（参见 [RFC 0059](../../../../docs/rfcs/0059-display-components.md)）。
