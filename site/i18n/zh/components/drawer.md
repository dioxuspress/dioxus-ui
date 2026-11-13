## API 概览

- `Drawer`
- `DrawerTrigger`
- `DrawerOverlay`
- `DrawerContent`
- `DrawerHeader`
- `DrawerFooter`
- `DrawerTitle`
- `DrawerDescription`
- `DrawerClose`
- `DrawerPrimitiveConfig`
- `drawer_overlay_class`
- `drawer_content_class`

该模块还重导出了 `DialogPrimitiveConfig`，方便直接从 `dioxus_shadcn::drawer` 导入使用。

## 行为与交互

`open` 状态始终由应用受控管理。请将同一个 `on_open_change` 处理函数传递给 `DrawerOverlay`、`DrawerContent` 与 `DrawerClose` 以接收关闭请求。

打开期间，抽屉会锁定页面滚动，并为隐藏滚动条后的根元素补充内边距以防止布局抖动；嵌套模态层共享同一把滚动锁，当最后一个模态层关闭时恢复页面滚动（参见 [RFC 0068](../../../../docs/rfcs/0068-modal-scroll-lock.md)）。

- 当启用了 `dismiss.escape_key` 时，在内容区域按下 `Escape` 会请求关闭。
- 当启用了 `dismiss.outside_pointer` 时，点击遮罩层（overlay）会请求关闭。默认值 `DismissBehavior::dialog_default()` 将该项保持关闭。
- `DrawerClose` 始终会请求关闭。
- 打开时会优先聚焦标记了 `data-dxui-autofocus` 的元素（例如由键盘管理的 `Calendar` 日期格），否则聚焦第一个可聚焦元素，若均不存在则聚焦内容容器本身。
- 打开期间，`Tab` 与 `Shift+Tab` 会在内容区域内部循环移动焦点，并跳过 `tabindex="-1"` 的元素。
- 关闭抽屉或将内容从组件树中移除时，焦点会恢复到打开前聚焦的元素，除非焦点已经主动移到了内容区域之外的其他控件。

`Drawer` 与 `Dialog` 共享同一套焦点作用域逻辑；浏览器冒烟测试已通过 `Dialog` 与 `AlertDialog` 覆盖该机制。

## 无障碍说明

内容容器使用 `role="dialog"` 与 `aria-modal="true"`。Portal 挂载、触控手势与拖拽关闭行为暂未内置。

`DrawerContent` 会自动从内部的 `DrawerTitle` 获取无障碍名称，并从 `DrawerDescription` 获取描述：这两个子部件会自动生成唯一 ID，内容容器会将 `aria-labelledby` 和 `aria-describedby` 指向当前已挂载的对应部件。在 `DrawerContent` 上显式传入 `aria-label`、`aria-labelledby` 或 `aria-describedby` 会覆盖自动生成的值；因此当不包含标题部件时，请直接传入 `aria-label`（参见 [RFC 0039](../../../../docs/rfcs/0039-dialog-names.md)）。
