## API 概览

- `ChartRoot`
- `ChartSvg`
- `ChartTitle`
- `ChartDescription`
- `ChartLegend`
- `ChartFallbackTable`
- `ChartTooltipSlot`
- `ChartLineSeries`
- `ChartBarSeries`
- `ChartAreaSeries`
- `ChartPoint`
- `ChartSeries`
- `ChartDomain`
- `ChartScale`
- `ChartColorToken`
- `ChartFallbackRow`（用于 `ChartFallbackTable` 的 `rows` 属性）
- `chart_fallback_rows`
- `chart_summary`
- `chart_domain`, `chart_scale_value`, `chart_view_box`
- `ChartPieSeries`, `ChartSlice`
- `CHART_COLOR_CLASSES`

### 饼图与环形图

`ChartPieSeries` 接收一组 `ChartSlice`（每项包含 ID、标签、数值与颜色令牌）作为 `slices`，按各分片占总和的比例从正上方起顺时针绘制扇区：

```rust
ChartSvg { view_box: chart_view_box(200.0, 200.0), title_id: "traffic-title", description_id: "traffic-description",
  ChartPieSeries {
    inner_radius: 60.0,
    slices: vec![
      ChartSlice::new("direct", "Direct", 4200.0, ChartColorToken::Chart1),
      ChartSlice::new("search", "Search", 3100.0, ChartColorToken::Chart2),
    ],
  }
}
```

`center`（默认为 `(100.0, 100.0)`）与 `radius`（默认为 `96`）均采用 `viewBox` 坐标单位；将 `inner_radius` 设为大于零即可绘制环形图，`gap`（默认为 `2`）会使用背景色为各扇区描边留缝。负数、零或非有限数值不会绘制任何图形。`ChartColorToken::Chart1` 至 `Chart5` 分别读取所有主题预设均会定义的 `--chart-1` 至 `--chart-5` 颜色令牌（参见 [RFC 0065](../../../../docs/rfcs/0065-pie-and-donut-charts.md)）。

`chart_color_class` 定义在 primitives crate 中，在 crate 依赖模式下不会被 Tailwind 的 `@source` 扫描到；因此当前 crate 通过 `CHART_COLOR_CLASSES` 显式列出这些类名，确保样式能够正常生成。

## 无障碍说明

请为 `ChartSvg` 指定稳定的 `title_id` 与 `description_id`。对于非简单示意的数据，请提供 `ChartFallbackTable` 或相邻的数据表格。数据系列必须通过文本加以区分，而不能仅依赖颜色：请为饼图各扇区提供带标签的图例，并在 `ChartFallbackTable` 中列出具体数值。悬浮提示（tooltip）仅作为补充信息；指针跟踪、键盘游标探索、数值格式化、本地化以及外部图表后端集成均由应用自行负责。
