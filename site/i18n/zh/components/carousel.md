## API 概览

- `Carousel`
- `CarouselViewport`
- `CarouselContent`
- `CarouselItem`
- `CarouselPrevious`
- `CarouselNext`
- `CarouselIndicator`
- `CarouselOrientation`
- `CarouselStep`
- `carousel_next`
- `carousel_previous`
- `carousel_can_go_next`
- `carousel_can_go_previous`

## 幻灯片切换

`Carousel` 负责管理当前选中的幻灯片索引（[RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。请通过 `count` 传入幻灯片总数，并为每个 `CarouselItem` 与 `CarouselIndicator` 指定各自的 `index`。根组件初始停留在 `default_index`，或在受控模式下跟随应用传入的 `index`；`on_index_change` 会监听用户的每次切换操作：

```rust
rsx! {
  Carousel { "aria-label": "Featured", count: 3,
    CarouselViewport {
      CarouselContent {
        for slide in 0..3 {
          CarouselItem { key: "{slide}", index: slide, "aria-label": "{slide + 1} of 3", "Slide {slide + 1}" }
        }
      }
    }
    CarouselPrevious {}
    CarouselNext {}
  }
}
```

- `CarouselPrevious` 与 `CarouselNext` 用于前后步进切换索引，且在未开启 `looping` 时会在首尾两端自动禁用；传入 `disabled` 也可单独禁用任一按钮。`CarouselIndicator` 点击后会切换至对应幻灯片，并在选中期间设置 `aria-current`。
- 每个幻灯片项按 `index` 乘以自身尺寸进行平移，因此 `basis-1/2` 等多列同屏视图无需测量 DOM 尺寸即可正常工作。当系统开启减弱动态效果（reduced motion）时会自动关闭过渡动画。
- `CarouselContent` 会将当前索引写入 `--dxui-carousel-index` CSS 自定义属性，并与传入的 `style` 合并；具体的 `transform` 样式由各幻灯片项自身持有。
- 只要焦点位于根容器内部任意位置（包括某张幻灯片内的表单字段），水平轮播即可通过 `ArrowLeft` 与 `ArrowRight` 前后切换，垂直轮播则通过 `ArrowUp` 与 `ArrowDown` 切换。
- 所有子部件必须位于 `Carousel` 内部；否则不会渲染任何内容，并会在日志中提示缺少的根组件。
- 所有部件均支持透传全局属性与元素属性。传入 `aria-label` 可替换 `CarouselPrevious`、`CarouselNext` 与 `CarouselIndicator` 的默认英文标签，便于为每个指示器标明其对应的幻灯片。
- 屏幕外的幻灯片默认仍保留在 `Tab` 焦点顺序和无障碍树中。当幻灯片内部包含可聚焦内容时，请为未处于当前屏幕的幻灯片传入 `inert`。

## 无障碍说明

根容器使用带有 `aria-roledescription="carousel"` 的区域（region）语义，幻灯片项使用幻灯片分组语义，上一页和下一页控件均为带有禁用状态的原生按钮。应用应当通过 `aria-label` 为轮播区域本身以及每张幻灯片（例如 `"2 of 5"`）提供可访问名称，并为每个指示器命名。
