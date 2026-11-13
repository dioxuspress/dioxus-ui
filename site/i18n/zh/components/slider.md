## API 概览

- `Slider`
- `SliderOrientation`
- `SliderState`
- `SliderKeyMove`
- `slider_key_move`
- `RangeSlider`

## 键盘与指针输入

```rust
let mut volume = use_signal(|| 40.0);

rsx! {
  Label { id: "volume-label", "Volume" }
  Slider {
    "aria-labelledby": "volume-label",
    value: volume(),
    on_value_change: move |value| volume.set(value),
  }
}
```

`Slider` 是受控组件。当步进对齐后的新值与当前 `value` 不同时，它会以新值调用 `on_value_change`，由应用将其回传给 `value`。

| 输入方式 | 数值变化 |
| --- | --- |
| `ArrowRight`, `ArrowUp` | 增加一个步长 |
| `ArrowLeft`, `ArrowDown` | 减少一个步长 |
| `PageUp`, `PageDown` | 增加或减少十个步长 |
| `Home`, `End` | 跳至最小值、最大值 |
| 按下或拖拽 | 设为指针所在位置对应的值 |

已被组件处理的按键不会触发页面滚动。页面脚本会在按下时捕获指针（pointer capture），因此拖拽移出滑块区域时仍可继续生效。禁用的滑块会忽略键盘和指针输入。`slider_key_move` 导出了上述按键映射逻辑。

其他属性（如 `aria-label`、`aria-labelledby` 和 `aria-valuetext`）会透传至根元素。水平滑块从左向右填充，不支持从右到左（RTL）镜像；自下而上的填充方向请参见下方的“垂直滑块”章节。

## 垂直滑块

```rust
rsx! {
  div { class: "h-44",
    Slider {
      "aria-label": "Volume",
      orientation: SliderOrientation::Vertical,
      value: volume(),
      on_value_change: move |value| volume.set(value),
    }
  }
}
```

`orientation: SliderOrientation::Vertical` 会从底部向上填充滑块，沿高度方向读取指针位置，并渲染 `aria-orientation="vertical"`。请为其外层容器指定明确的高度。`ArrowUp` 与 `ArrowRight` 仍然用于增大数值。无论哪种方向，滑块拖拽柄（thumb）均通过行内绝对定位居中对齐在当前值位置上（参见 [RFC 0042](../../../../docs/rfcs/0042-slider-thumb-position-and-vertical-orientation.md)）。

## 范围滑块

`RangeSlider` 同时维护下界和上界两个数值（参见 [RFC 0070](../../../../docs/rfcs/0070-range-slider.md)）：

```rust
let mut price = use_signal(|| (20.0, 80.0));

rsx! {
  RangeSlider {
    "aria-label": "Price",
    value: price(),
    step: 5.0,
    min_steps_between: 2,
    on_value_change: move |next| price.set(next),
  }
}
```

- 每个拖拽柄都是拥有独立焦点的 `slider`，由 `start_label` 与 `end_label`（默认为 `"Minimum"` 与 `"Maximum"`）命名，并响应与 `Slider` 相同的键盘按键。每个拖拽柄的 `aria-valuemin` 或 `aria-valuemax` 受另一个拖拽柄的值以及 `min_steps_between` 最小步数间距的约束。
- 按下指针时会移动距离更近的拖拽柄并使其获得焦点；持续拖拽会继续移动该拖拽柄，并在触及另一个拖拽柄的边界时停止。
- 根元素具有 `group` 角色；请使用 `aria-label` 或 `aria-labelledby` 为其命名。
- 暂不支持从右到左（RTL）的水平范围滑块。

## 无障碍说明

`Slider` 渲染 `role="slider"`，并根据限制范围且步进对齐后的数值生成方向与 ARIA 数值属性。请通过 `aria-labelledby` 或 `aria-label` 为其提供无障碍名称；带有 `for` 的 `Label` 无法为 `div` 命名。当纯数字不足以表达含义（例如 `"40 percent"`）时，请传入 `aria-valuetext`。
