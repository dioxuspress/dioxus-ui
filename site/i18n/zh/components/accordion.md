## API 概览

- `Accordion`
- `AccordionItem`
- `AccordionTrigger`
- `AccordionContent`
- `accordion_item_class`
- `accordion_trigger_class`
- `accordion_content_class`

## 行为与交互

`Accordion` 默认每次仅展开一个面板项。可通过 `default_value` 指定初始展开项，或通过 `value` 进行受控管理；空字符串表示所有面板项均处于收起状态。`on_value_change` 会监听用户的每次展开或收起操作，包括当前展开项关闭时传入的 `""`：

```rust
rsx! {
  Accordion { default_value: "shipping",
    AccordionItem { value: "shipping",
      AccordionTrigger { "Shipping" }
      AccordionContent { "Ships in two days." }
    }
    AccordionItem { value: "returns",
      AccordionTrigger { "Returns" }
      AccordionContent { "Returns within 30 days." }
    }
  }
}
```

- 设置 `multiple: true` 后可同时展开任意数量的面板项；此时根组件改为接收 `values`、`default_values` 与 `on_values_change`。
- 在触发器上点击或按 `Enter`、`Space` 会展开对应面板项，若已展开则将其收起。
- `ArrowUp` 与 `ArrowDown` 在未禁用的触发器之间循环移动焦点；`Home` 与 `End` 分别跳至第一个与最后一个触发器。仅移动焦点不会切换展开状态。
- 每个未禁用的触发器均保留在 `Tab` 焦点顺序中。
- `AccordionItem` 必须传入 `value`。每个触发器都会渲染 `aria-controls`，每个内容区域都会渲染 `aria-labelledby`，对应的关联 ID 由根组件自动生成。
- `AccordionItem` 必须位于 `Accordion` 内部，触发器与内容区则必须位于 `AccordionItem` 内部；否则对应部件不会渲染任何内容，并会在日志中提示缺少的根组件。

Web 渲染器的交互验证由 `npm run verify:runtime-interactions` 覆盖。

## 无障碍说明

每个触发器都是包裹在 `h3` 内的 `button`，并带有 `aria-expanded` 属性。内容区使用 `role="region"`，并以对应触发器作为可访问名称来源。不可收起的常驻展开项、自定义标题层级、水平手风琴以及展开过渡动画暂未实现（参见 [RFC 0021](../../../../docs/rfcs/0021-accordion-interaction.md)）。
