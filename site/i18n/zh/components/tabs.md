## API 概览

- `Tabs`
- `TabsList`
- `TabsTrigger`
- `TabsContent`
- `TabsActivation`
- `TabsOrientation`
- `tabs_class`
- `tabs_list_class`
- `tabs_trigger_class`
- `tabs_content_class`

## 行为与交互

`Tabs` 负责管理当前选中的标签页并将各子部件关联起来（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。可通过 `default_value` 指定初始选中项，或通过 `value` 进行受控管理；两种方式下 `on_value_change` 都会在每次切换时收到通知：

```rust
rsx! {
  Tabs { default_value: "account",
    TabsList {
      TabsTrigger { value: "account", "Account" }
      TabsTrigger { value: "billing", "Billing" }
    }
    TabsContent { value: "account", "Account settings" }
    TabsContent { value: "billing", "Billing settings" }
  }
}
```

当应用需要主动切换标签页（例如保存失败后跳转到对应面板）时，可使用受控模式：

```rust
let mut tab = use_signal(|| "account".to_string());

rsx! {
  Tabs { value: tab(), on_value_change: move |next| tab.set(next),
    // the same parts
  }
}
```

- 所有触发器共同构成单一 `Tab` 停靠点，落在当前选中的触发器上。在此按 `Tab` 会将焦点移入带有 `tabindex="0"` 的当前可见面板。
- `ArrowLeft` 与 `ArrowRight` 在未禁用的触发器之间循环移动焦点；`Home` 与 `End` 分别跳至首项与末项。焦点移动时会立即选中所聚焦的触发器（自动激活模式）。
- 设置 `activation: TabsActivation::Manual` 后，方向键仅移动焦点，按 `Enter`、`Space` 或点击才会真正选中标签页。
- 设置 `orientation: TabsOrientation::Vertical` 后，改为使用 `ArrowUp` 与 `ArrowDown` 移动焦点，列表会渲染 `aria-orientation="vertical"`，同时根容器、列表与各内容面板都会渲染 `data-orientation="vertical"`，使标签列表与内容面板左右并排排列。
- 当焦点离开标签列表时，当前选中的触发器会重新成为 `Tab` 停靠点，因此按 `Shift+Tab` 退回列表时会准确落回该触发器。
- 在从右到左（RTL）的水平标签列表中，`ArrowLeft` 移至下一个触发器，`ArrowRight` 移至上一个触发器。
- 在触发器上点击或按 `Enter`、`Space` 会选中该标签页。
- `Tabs` 会通过 `aria-controls` 与 `aria-labelledby` 将每个触发器与其对应的内容面板关联起来。放置在 `Tabs` 外部的子部件不会渲染任何内容，且 Dioxus 会在日志中提示其必须位于 `Tabs` 内部。

Web 渲染器的交互验证由 `npm run verify:runtime-interactions` 覆盖。

## 无障碍说明

列表使用 `role="tablist"` 并设置与 `orientation` 一致的 `aria-orientation`，触发器使用带有 `aria-selected` 的 `role="tab"`，内容面板使用 `role="tabpanel"`。激活模式与方向语义遵循 [RFC 0026](../../../../docs/rfcs/0026-tabs-manual-activation-and-vertical-orientation.md)。

`TabsList` 支持属性透传，请通过 `aria-label` 或 `aria-labelledby` 为标签列表提供可访问名称（参见 [RFC 0040](../../../../docs/rfcs/0040-composite-widget-names.md)）。
