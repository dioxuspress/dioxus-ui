## API 概览

- `Select`
- `SelectTrigger`
- `SelectValue`
- `SelectContent`
- `SelectGroup`
- `SelectLabel`
- `SelectItem`
- `SelectSeparator`
- `SelectPrimitiveConfig`
- `SelectDismissBehavior`, `SelectSide`, `SelectAlign`
- `select_trigger_class`
- `select_item_class`

## 行为与交互

`Select` 持有当前选中的值以及列表是否展开的状态，并将内部各子部件关联起来（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。可通过 `default_value` 指定初始值，或通过 `value` 进行受控管理；无论哪种模式，`on_value_change` 都会接收每一次选择变更：

```rust
rsx! {
  Select { default_value: "system",
    SelectTrigger { SelectValue { placeholder: "Theme" } }
    SelectContent {
      for theme in ["light", "dark", "system"] {
        SelectItem { key: "{theme}", value: theme, "{theme}" }
      }
    }
  }
}
```

若要显示不同于原始值的展示文本，可受控管理 `value`，并在触发器内自行渲染标签内容：

```rust
let mut fruit = use_signal(|| "banana".to_string());

rsx! {
  Select { id: "fruit", value: fruit(), on_value_change: move |next| fruit.set(next),
    Label { r#for: "fruit", "Fruit" }
    SelectTrigger { span { class: "truncate", "{label_of(&fruit())}" } }
    SelectContent {
      SelectItem { value: "apple", "Apple" }
      SelectItem { value: "banana", "Banana" }
    }
  }
}
```

- 列表的展开状态同样支持通过 `open`、`default_open` 与 `on_open_change` 进行非受控或受控管理。
- `id` 用于为触发器指定标识以便配合 `Label` 使用；未指定时会自动生成 ID。
- 点击触发器会切换列表展开状态。在收起的触发器上按下 `ArrowDown` 或 `ArrowUp` 会展开列表。
- 列表默认放置在 `side`（默认 `Bottom`）方向，采用 `align`（默认 `Start`）对齐与 `side_offset`（默认 `4`）偏移，并像 `Popover` 一样自动翻转与偏移以留在视口内。列表宽度至少与触发器等宽，且不小于 `8rem`（参见 [RFC 0046](../../../../docs/rfcs/0046-listbox-width-follows-trigger.md)）。
- 焦点始终保留在触发器上。当前选中项（或第一个可用选项）会添加 `data-highlighted` 高亮属性，并由触发器的 `aria-activedescendant` 引用。`ArrowDown` 与 `ArrowUp` 在可用选项间移动且不循环，`Home` 与 `End` 跳转到首尾选项，输入字符前缀会高亮下一个匹配项。
- 按下 `Enter`、`Space` 或点击选项会选中该项并关闭列表。禁用选项会被跳过且无法选中。
- `Escape` 与外部交互会根据 `dismiss`（默认 `DismissBehavior::popover_default()`）关闭列表。
- `SelectValue` 会显示当前选中的值、以逗号连接的多个选中值或 `placeholder`，为空时附带 `data-placeholder="true"`。
- 选中项会在行内末尾显示勾选图标；高亮项会应用强调背景色。
- 位于 `Select` 外部的子部件不会渲染任何内容，且 Dioxus 会输出日志提示其必须放置在 `Select` 内部。

### 多选模式

启用 `multiple` 后，选择操作会切换对应值的选中状态并保持列表展开，同时设置 `aria-multiselectable="true"`。此时通过 `values`、`default_values` 与 `on_values_change` 管理选中的值列表：

```rust
rsx! {
  Select { multiple: true, default_values: vec!["rust".to_string()],
    SelectTrigger { SelectValue { placeholder: "Languages" } }
    SelectContent {
      SelectItem { value: "rust", "Rust" }
      SelectItem { value: "go", "Go" }
    }
  }
}
```

按下 `Escape` 或发生外部交互时仍会关闭列表（参见 [RFC 0062](../../../../docs/rfcs/0062-multi-select.md)）。

Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试，Desktop 渲染器由 `npm run verify:desktop-interactions` 覆盖，iOS 模拟器由 `npm run verify:mobile-interactions` 覆盖，Android 模拟器由 `npm run verify:android-interactions` 覆盖。

## 无障碍说明

触发器采用仅选择模式的组合框（select-only combobox）语义，带有 `aria-haspopup="listbox"` 与 `aria-activedescendant`；弹出内容采用 `listbox` 与 `option` 语义。使用方应用应为触发器提供可见或编程式的无障碍标签。

列表框渲染 `id="{trigger id}-content"` 并通过 `aria-labelledby` 继承触发器的名称，同时触发器将 `aria-controls` 指向该列表框；显式传入的 `aria-controls` 会覆盖派生值。禁用项会设置 `aria-disabled`（参见 [RFC 0055](../../../../docs/rfcs/0055-open-state-accessibility-audit.md)）。

`SelectTrigger` 会透传 `aria-labelledby` 和 `aria-describedby` 等属性，便于关联字段说明或错误提示（参见 [RFC 0038](../../../../docs/rfcs/0038-form-control-naming.md)）。
