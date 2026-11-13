## API 概览

- `Command`
- `CommandInput`
- `CommandList`
- `CommandEmpty`
- `CommandStatus`
- `CommandGroup`
- `CommandLabel`
- `CommandItem`
- `CommandSeparator`
- `CommandShortcut`
- `ActiveDescendantState`（仅限 crate 模式）
- `command_class`
- `command_input_class`
- `command_item_class`
- `command_matches`

## 行为与交互

`Command` 内部维护当前高亮的选项，并将输入框与列表关联起来（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。查询字符串由应用持有，应用负责过滤并渲染选项列表，而 `Command` 通过 `on_select` 上报用户选中的项：

```rust
let mut query = use_signal(String::new);
let items = [("calendar", "Calendar"), ("settings", "Settings")];

rsx! {
  Command {
    on_select: move |value: String| run(&value),
    CommandInput {
      value: query(),
      placeholder: "Type a command...",
      oninput: move |event: FormEvent| query.set(event.value()),
    }
    CommandList {
      for (value, label) in items.into_iter().filter(|(_, label)| command_matches(label, &query())) {
        CommandItem { key: "{value}", id: "command-{value}", value, "{label}" }
      }
    }
  }
}
```

- 默认高亮第一个可用选项。输入框的 `aria-activedescendant` 会指向该选项，且该选项带有 `data-highlighted` 属性。
- `ArrowDown` 和 `ArrowUp` 分别移动到下一个或上一个可用选项，并在首尾边界处停止。`Home` 和 `End` 分别高亮第一个和最后一个可用选项。
- 键盘输入（包括 `Space`）会直接进入输入框。当查询词变化时，高亮状态会重置为重新渲染后列表中的第一个可用选项。
- 在选项上按 `Enter` 或点击会以该项的 `value`（若未设置则为 `id`）调用 `on_select`。焦点始终停留在输入框内。禁用项会被跳过且无法选中。
- 指针移入某个选项时会将其高亮。
- 当没有匹配结果时，请渲染 `CommandEmpty`；此时输入框不包含 `aria-activedescendant`，按 `Enter` 也不会触发任何操作。
- 当去除首尾空白后的查询词为空，或标签忽略大小写包含该查询词时，`command_matches(label, query)` 返回 `true`。
- 输入框拥有自动生成的 `id` 以及指向列表的 `aria-controls`。
- `CommandInput` 和 `CommandList` 必须位于 `Command` 内部；否则它们不会渲染任何内容，并会在日志中提示缺失的根组件。

若要向屏幕阅读器播报匹配项数量，请始终渲染 `CommandStatus` 并仅更新其文本内容；空文本不会触发播报：

```rust
let matches = items.iter().filter(|(_, label)| command_matches(label, &query())).count();

rsx! {
  CommandStatus {
    if query().is_empty() { "" }
    else if matches == 0 { "No results" }
    else if matches == 1 { "1 result" }
    else { "{matches} results" }
  }
}
```

`CommandStatus` 会渲染一个视觉隐藏的 `role="status"` 区域，带有 `aria-live="polite"` 和 `aria-atomic="true"`。由于 `CommandEmpty` 通常仅在无匹配项时才挂载渲染，因此它自身不会触发实时播报。

Web 渲染器的交互行为已由 `npm run verify:runtime-interactions` 覆盖验证。

## 无障碍说明

输入框采用 `combobox` 语义，列表采用 `listbox` 语义，子项采用 `option` 语义。`CommandStatus` 会播报应用传入的状态文本（参见 [RFC 0027](../../../../docs/rfcs/0027-combobox-and-command-result-announcements.md)）。目前未实现模糊排序、循环跳转以及 `Ctrl` 组合键绑定（参见 [RFC 0024](../../../../docs/rfcs/0024-command-keyboard-and-filtering.md)）。
