## API 概览

- `Indicator`
- `IndicatorItem`
- `IndicatorPlacement`
- `indicator_class`
- `indicator_item_class`

```rust
rsx! {
  Indicator {
    IndicatorItem { Badge { "3" } }
    Button { variant: ButtonVariant::Outline, "Inbox" }
  }
}
```

`IndicatorPlacement` 可取 `TopEnd`（默认值）、`TopStart`、`BottomEnd` 或 `BottomStart`。停靠位置采用逻辑边（logical sides）语义，因此在从右到左（RTL）布局中 `TopEnd` 会位于左上角。

## 无障碍说明

`Indicator` 自身不添加额外语义。计数角标会按其在源码中的先后顺序被朗读，因此当计数值从属于目标元素时，建议将 `IndicatorItem` 放在该元素之后，或者直接将计数写入目标元素的可访问名称中（例如 `aria-label: "Inbox, 3 unread"`；参见 [RFC 0059](../../../../docs/rfcs/0059-display-components.md)）。
