## API 概览

- `Dialog`
- `DialogTrigger`
- `DialogOverlay`
- `DialogContent`
- `DialogTitle`
- `DialogDescription`
- `DialogClose`
- `DialogPrimitiveConfig`
- `DismissBehavior`（对应 `dismiss` 属性）
- `dialog_overlay_class`
- `dialog_content_class`

## 行为与交互

`Dialog` 持有对话框是否打开的状态，内部各子部件均读取该状态，因此必须放置在 `Dialog` 内部（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。`DialogTrigger` 用于打开对话框；它渲染一个无默认样式的 `button`，请通过 `class` 添加样式：

```rust
rsx! {
  Dialog {
    DialogTrigger {
      class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
      "Rename project"
    }
    DialogOverlay {}
    DialogContent {
      DialogTitle { "Rename project" }
      DialogClose { "Cancel" }
    }
  }
}
```

`default_open` 可使对话框默认处于打开状态。若需受控管理，请传入 `open` 与 `on_open_change`；此时应用中的任意按钮都可以打开或关闭对话框，触发器变为可选项：

```rust
let mut open = use_signal(|| false);

rsx! {
  Dialog { open: open(), on_open_change: move |next| open.set(next),
    DialogOverlay {}
    DialogContent {
      DialogTitle { "Rename project" }
      Button { onclick: move |_| open.set(false), "Save" }
    }
  }
}
```

无论哪种模式，`on_open_change` 都会接收用户触发的每一次状态变更。触发器会将 `aria-controls` 指向内容容器并设置 `aria-expanded`。

打开期间，对话框会锁定页面滚动，并为隐藏滚动条后的根元素补充内边距以防止布局抖动；嵌套模态层共享同一把滚动锁，当最后一个模态层关闭时恢复页面滚动（参见 [RFC 0068](../../../../docs/rfcs/0068-modal-scroll-lock.md)）。

- 当启用了 `dismiss.escape_key` 时，在内容区域按下 `Escape` 会关闭对话框。
- 当启用了 `dismiss.outside_pointer` 时，点击遮罩层（overlay）会关闭对话框。默认值 `DismissBehavior::dialog_default()` 将该项保持关闭。
- `DialogClose` 始终会关闭对话框。
- 打开时会优先聚焦标记了 `data-dxui-autofocus` 的元素（例如由键盘管理的 `Calendar` 日期格），否则聚焦第一个可聚焦元素，若均不存在则聚焦内容容器本身。
- 打开期间，`Tab` 与 `Shift+Tab` 会在内容区域内部循环移动焦点，并跳过 `tabindex="-1"` 的元素。
- 关闭对话框或将内容从组件树中移除时，焦点会恢复到打开前聚焦的元素，除非焦点已经主动移到了内容区域之外的其他控件。

焦点管理通过 `document::eval` 执行，因此在 Web、Desktop 与 Mobile 渲染器中均可正常工作。Web 渲染器由 `npm run verify:runtime-interactions` 覆盖测试，Desktop 渲染器由 `npm run verify:desktop-interactions` 覆盖，iOS 模拟器由 `npm run verify:mobile-interactions` 覆盖，Android 模拟器由 `npm run verify:android-interactions` 覆盖。

## 无障碍说明

对话框会捕获焦点（focus trap）、在关闭时恢复焦点，并按照配置的 `Escape` 键与外部交互策略执行关闭。请始终渲染 `DialogTitle`。

`DialogContent` 会自动从内部的 `DialogTitle` 获取无障碍名称，并从 `DialogDescription` 获取描述：这两个子部件会自动生成唯一 ID，内容容器会将 `aria-labelledby` 和 `aria-describedby` 指向当前已挂载的对应部件。在 `DialogContent` 上显式传入 `aria-label`、`aria-labelledby` 或 `aria-describedby` 会覆盖自动生成的值；因此当不包含标题部件时，请直接传入 `aria-label`（参见 [RFC 0039](../../../../docs/rfcs/0039-dialog-names.md)）。
