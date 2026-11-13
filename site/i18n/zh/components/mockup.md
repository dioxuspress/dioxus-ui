## API 概览

- `MockupBrowser`
- `MockupWindow`
- `MockupCode`
- `MockupCodeLine`
- `MockupPhone`
- `mockup_frame_class`
- `mockup_code_line_class`

```rust
rsx! {
  MockupBrowser { url: "https://example.com", content_class: "p-8", "Hello" }
  MockupCode {
    MockupCodeLine { prefix: "$", "cargo install dioxus-shadcn-cli" }
    MockupCodeLine { prefix: "$", "dxui add mockup" }
    MockupCodeLine { highlight: true, "Added mockup" }
  }
  MockupPhone { div { class: "grid h-full place-items-center", "App" } }
}
```

## 行为与交互

参见 [RFC 0072](../../../../docs/rfcs/0072-menu-and-mockup.md)。

- `MockupBrowser` 在顶部显示三个窗口圆点，并在传入 `url` 时展示地址栏；`MockupWindow` 仅显示窗口圆点。主体内容放置在下方带边框的区域内，可通过 `content_class` 自定义样式。
- `MockupCode` 将多行 `MockupCodeLine` 纵向排列在终端风格的代码块中，单行过长时支持横向滚动。每行的 `prefix` 显示在行首且不会被划选复制；`highlight` 用于高亮标记当前行。
- `MockupPhone` 提供带有摄像头刘海的圆角边框，内部包裹按手机屏幕比例呈现的显示区域，可通过 `display_class` 自定义显示区样式。

## 无障碍说明

窗口圆点、摄像头刘海以及代码行前缀均设置了 `aria-hidden`，因此屏幕阅读器只会朗读实际内容。地址栏仅为纯文本而非可点击链接。当样机框中展示的是应用截图而非真实可交互内容时，请为图片本身提供合适的替代文本（alt text）。
