# Code Tabs

Code Tabs provides an underlined tab bar and switching code panels for
side-by-side comparison of commands, languages, or code examples. The
`CodeTabs` root owns the selected tab, or follows `value` when the app
controls it.

## Source Copy

```bash
dxui add code-tabs
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["code-tabs"] }
```

## API Surface

- `CodeTabs`
- `CodeTabsList`
- `CodeTabsTrigger`
- `CodeTabsContent`
- `CodeTabsActivation`
- `code_tabs_class`
- `code_tabs_list_class`
- `code_tabs_trigger_class`
- `code_tabs_content_class`

## Behavior

`CodeTabs` owns which code tab is selected and links its triggers and panels
(see [RFC 0077](../rfcs/0077-component-owned-state.md)). Start it with
`default_value`, or control it with `value`; `on_value_change` hears every
change either way:

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

- The triggers form one Tab stop on the selected trigger. Tab from it moves
  into the visible code panel, which has `tabindex="0"`.
- Left and Right move focus between enabled triggers and wrap; Home and End
  jump to the first and last. Moving focus selects the focused trigger
  (automatic activation).
- With `activation: CodeTabsActivation::Manual`, keys only move focus, and
  Enter, Space, or a click selects.
- The active trigger renders an underline indicator (`border-primary text-primary`),
  while inactive triggers render muted text.
- `CodeTabs` links each trigger to its panel with `aria-controls` and
  `aria-labelledby`. A part outside a `CodeTabs` renders nothing, and Dioxus
  logs that it must be inside one.

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

The list uses `role="tablist"` with `aria-orientation="horizontal"`, triggers
use `role="tab"` with `aria-selected`, and panels use `role="tabpanel"`.
`CodeTabsList` passes through attributes, so name the tab list with
`aria-label` or `aria-labelledby` (see
[RFC 0040](../rfcs/0040-composite-widget-names.md)).
