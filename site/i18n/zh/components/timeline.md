## API 概览

- `Timeline`
- `TimelineOrientation`
- `TimelineItem`
- `TimelineTime`
- `TimelineMarker`
- `TimelineContent`
- `timeline_class`
- `timeline_item_class`
- `timeline_time_class`
- `timeline_marker_class`
- `timeline_content_class`

```rust
rsx! {
  Timeline {
    TimelineItem {
      TimelineTime { datetime: "2026-10-05", "Oct 5" }
      TimelineMarker {}
      TimelineContent { "Published 0.1.0" }
    }
  }
}
```

默认的 `TimelineOrientation::Vertical` 将时间放置在节点标记旁边；`Horizontal` 则将时间放在上方、内容放在下方。连接线仅在相邻标记之间延伸。`TimelineMarker` 在无子元素时绘制主色圆点；可传入图标将其替换，并通过 `text-success` 等样式类调整颜色。

## 无障碍说明

`Timeline` 渲染为 `ol`，每个条目渲染为 `li`，使辅助技术能够按顺序朗读事件及其总数。`TimelineTime` 渲染为 `time` 元素，请为其传入 `datetime`。节点标记带有 `aria-hidden`，因此若标记表达了特定状态，请同时在内容文本中体现（参见 [RFC 0059](../../../../docs/rfcs/0059-display-components.md)）。
