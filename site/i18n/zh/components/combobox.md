## API 概览

- `Combobox`
- `ComboboxTrigger`
- `ComboboxInput`
- `ComboboxContent`
- `ComboboxList`
- `ComboboxEmpty`
- `ComboboxStatus`
- `ComboboxGroup`
- `ComboboxValue`
- `ComboboxItem`
- `ComboboxPrimitiveConfig`
- `ComboboxDismissBehavior`, `ComboboxSide`, `ComboboxAlign`
- `combobox_trigger_class`
- `combobox_input_class`
- `combobox_item_class`

## 行为与交互

与 `Select` 一样，`Combobox` 持有当前选中的值（或在 `multiple` 下持有多个值）以及列表是否展开的状态，内部各子部件均读取该状态，因此必须放置在 `Combobox` 内部（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。用户输入的搜索文本由应用自行维护，并由应用对所渲染的选项进行过滤：

```rust
let fruits = ["Apple", "Banana", "Blueberry", "Cherry"];
let mut query = use_signal(String::new);

rsx! {
  Combobox {
    id: "fruit-input",
    on_value_change: move |value| query.set(value),
    ComboboxInput {
      value: query(),
      placeholder: "Search fruit",
      oninput: move |event: FormEvent| query.set(event.value()),
    }
    ComboboxContent {
      ComboboxList {
        for fruit in fruits.iter().filter(|fruit| fruit.to_lowercase().contains(&query().to_lowercase())) {
          ComboboxItem { key: "{fruit}", value: *fruit, "{fruit}" }
        }
      }
    }
  }
}
```

可传入 `value`（`values`）或 `open` 及其变更回调进行受控管理，或传入 `default_value`（`default_values`）与 `default_open` 指定初始状态；两种模式下 `on_value_change` 都会接收每一次选择变更。

- 每个组合框只包含一个组合框宿主元素：优先是 `ComboboxInput`，若未使用输入框则是 `ComboboxTrigger`。根组件的 `id` 用于为该元素命名（以便配合 `Label`），且下拉列表会锚定到该元素；未指定 `id` 时会自动生成。请二选一使用：若在输入框旁同时放置触发器，会为同一个列表产生第二个组合框元素。
- 输入字符会展开列表，并通过 `oninput` 将事件传递给应用。在收起状态的输入框上按下 `ArrowDown` 会展开列表；点击触发器会切换列表展开状态。
- 弹出内容默认放置在 `side`（默认 `Bottom`）方向，采用 `align`（默认 `Start`）对齐与 `side_offset`（默认 `4`）偏移，并像 `Popover` 一样自动翻转与偏移。列表宽度至少与输入框等宽，且不小于 `8rem`（参见 [RFC 0046](../../../../docs/rfcs/0046-listbox-width-follows-trigger.md)）。
- 焦点始终保留在输入框中。在按下 `ArrowDown` 或 `ArrowUp` 之前不会高亮任何选项；高亮项会获得 `data-highlighted` 属性并由输入框的 `aria-activedescendant` 引用。上下方向键会跳过禁用项且不循环，而 `Home`、`End`、`Space` 与可打印字符键仍由输入框自身处理。
- 当过滤操作移除了当前高亮项时，高亮状态会自动清除。
- 按下 `Enter` 或点击选项会选中该项（使其成为当前值），并关闭列表。
- `Escape` 与外部交互会根据 `dismiss`（默认 `DismissBehavior::popover_default()`）关闭列表。

若要向屏幕阅读器播报匹配选项的数量，请在 `ComboboxContent` 外部（因为内容区域在收起时处于隐藏状态）、紧邻输入框的位置渲染 `ComboboxStatus`。请保持该元素始终挂载并仅更新其内部文本；空字符串不会触发播报：

```rust
let matches = fruits.iter().filter(|fruit| fruit.to_lowercase().contains(&query().to_lowercase())).count();

rsx! {
  ComboboxInput { /* ... */ }
  ComboboxStatus {
    // `open` is the app's signal, passed to the root as `open`.
    if !open() { "" }
    else if matches == 0 { "No results" }
    else if matches == 1 { "1 result" }
    else { "{matches} results" }
  }
  ComboboxContent { /* ... */ }
}
```

`ComboboxStatus` 渲染一个视觉隐藏的 `role="status"` 实时区域，带有 `aria-live="polite"` 与 `aria-atomic="true"`。若按条件挂载渲染，会重新创建 DOM 元素，导致首次写入的文本无法被辅助技术播报。

目前仅 Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试。

### 多选模式

与 `Select` 类似，在 `Combobox` 上启用 `multiple` 后，选择操作会切换 `values` 中的对应值、保持列表展开，并在 `ComboboxList` 上设置 `aria-multiselectable="true"`；是否清空搜索词由应用自行决定。已选中的选项会显示勾选图标（参见 [RFC 0062](../../../../docs/rfcs/0062-multi-select.md)）。

## 无障碍说明

触发器与输入框采用 `combobox` 语义，列表采用 `listbox` 语义，选项采用 `option` 语义。输入框保持焦点，并通过 `aria-activedescendant` 引用当前高亮项。`ComboboxStatus` 负责播报应用传入的文本；具体文案与播报时机由应用控制（参见 [RFC 0027](../../../../docs/rfcs/0027-combobox-and-command-result-announcements.md)）。异步加载状态由使用方应用自行管理。

`ComboboxList` 渲染 `id="{id}-list"` 并通过 `aria-labelledby` 继承组合框元素的名称；同时 `ComboboxInput`（以及展开状态下的 `ComboboxTrigger`）会将 `aria-controls` 指向该列表；显式传入的 `aria-controls` 会覆盖派生值（参见 [RFC 0055](../../../../docs/rfcs/0055-open-state-accessibility-audit.md)）。

`ComboboxInput` 会透传 `name`、`aria-labelledby` 和 `aria-describedby` 等属性（参见 [RFC 0038](../../../../docs/rfcs/0038-form-control-naming.md)）。`ComboboxTrigger` 负责切换列表展开状态、调用自身的 `onclick`，并透传 `aria-label` 和 `aria-controls` 等属性；其 `role`、`aria-expanded` 与 `aria-invalid` 则由组件属性派生（参见 [RFC 0053](../../../../docs/rfcs/0053-interactive-part-callbacks.md)）。
