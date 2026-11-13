## API 概览

- `CodeTabs`
- `CodeTabsList`
- `CodeTabsTrigger`
- `CodeTabsContent`
- `CodeTabsActivation`
- `code_tabs_class`
- `code_tabs_list_class`
- `code_tabs_trigger_class`
- `code_tabs_content_class`

## 行为与交互

`CodeTabs` 持有当前选中的代码标签页，并将触发器与面板互相关联（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。可通过 `default_value` 指定初始标签页，或通过 `value` 进行受控管理；无论哪种模式，`on_value_change` 都会接收每一次切换变更：

```rust
rsx! {
  CodeTabs { default_value: "typst",
    CodeTabsList { "aria-label": "Markup syntax comparison",
      CodeTabsTrigger { value: "markdown", "Markdown" }
      CodeTabsTrigger { value: "latex", "LaTeX" }
      CodeTabsTrigger { value: "typst", "Typst" }
    }
    CodeTabsContent { value: "markdown", "# Heading\n\nUse **bold** and `code`." }
    CodeTabsContent { value: "latex", "\\section{Heading}\nUse \\textbf{bold} and \\texttt{code}." }
    CodeTabsContent { value: "typst", "= Heading\nUse *bold* and `code`." }
  }
}
```

- 所有触发器共同组成单一的 `Tab` 停靠点（落在当前选中的触发器上）。在此按下 `Tab` 会将焦点移入带有 `tabindex="0"` 的当前可见代码面板。
- 左右方向键（`ArrowLeft` / `ArrowRight`）在可用触发器之间循环移动焦点；`Home` 与 `End` 跳转到首项与末项。移动焦点的同时会自动选中获得焦点的触发器（自动激活模式）。
- 当设置 `activation: CodeTabsActivation::Manual` 时，方向键仅移动焦点，需按下 `Enter`、`Space` 或点击才会触发选中。
- 激活状态的触发器会渲染下划线指示条（`border-primary text-primary`），非激活触发器则显示弱化文本色。
- `CodeTabs` 通过 `aria-controls` 与 `aria-labelledby` 将每个触发器与其对应面板关联起来。位于 `CodeTabs` 外部的子部件不会渲染任何内容，且 Dioxus 会输出日志提示其必须放置在 `CodeTabs` 内部。

Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试。

## 无障碍说明

标签列表使用 `role="tablist"` 与 `aria-orientation="horizontal"`，触发器使用 `role="tab"` 与 `aria-selected`，面板使用 `role="tabpanel"`。`CodeTabsList` 支持属性透传，请使用 `aria-label` 或 `aria-labelledby` 为标签列表命名（参见 [RFC 0040](../../../../docs/rfcs/0040-composite-widget-names.md)）。
