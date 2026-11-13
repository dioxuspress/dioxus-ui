## API 概览

- `SidebarProvider`
- `Sidebar`
- `SidebarRail`
- `SidebarHeader`
- `SidebarContent`
- `SidebarFooter`
- `SidebarGroup`
- `SidebarGroupLabel`
- `SidebarItem`
- `SidebarTrigger`
- `SidebarSide`
- `SidebarState`
- `sidebar_toggle`
- `SIDEBAR_MOBILE_QUERY`
- `sidebar_mobile_panel_class`, `sidebar_mobile_class`, `sidebar_overlay_class`

## 折叠切换与菜单项

`SidebarProvider` 负责管理侧边栏的折叠状态（[RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)），并将其共享给内部的 `Sidebar`、`SidebarRail` 与 `SidebarTrigger`；这些子部件可以放置在 `SidebarProvider` 内的任意层级，例如将侧边栏置于页面一侧、而将触发器放在页面顶栏中。`SidebarProvider` 自身不渲染任何 DOM 元素。`SidebarItem` 渲染为可标记当前页面的链接或按钮：

```rust
let mut section = use_signal(|| "inbox");

rsx! {
  SidebarProvider {
    SidebarTrigger { "Toggle sidebar" }
    Sidebar { "aria-label": "Main",
      SidebarContent {
        SidebarItem { active: section() == "inbox", onclick: move |_| section.set("inbox"), "Inbox" }
        SidebarItem { href: "/settings", "Settings" }
        SidebarItem { disabled: true, onclick: move |_| {}, "Archive" }
      }
    }
  }
}
```

- `SidebarProvider` 默认处于展开状态，也可通过 `default_collapsed: true` 默认折叠；传入 `collapsed` 可进入受控模式（例如在折叠时隐藏文字标签）。`on_collapsed_change` 会监听用户的每次折叠状态切换。
- `SidebarTrigger` 用于切换侧边栏状态，并通过 `aria-controls` 指向由 `SidebarProvider` 自动生成 ID 的侧边栏。若你为 `Sidebar` 显式指定了 `id`，请同时为 `SidebarTrigger` 传入一致的 `aria-controls`。禁用的触发器不会执行任何操作。
- `Sidebar`、`SidebarRail` 与 `SidebarTrigger` 必须位于 `SidebarProvider` 内部；否则它们不会渲染任何内容，并会在日志中提示缺少的根组件。
- 带有 `href` 的 `SidebarItem` 会渲染为锚点元素，适用于路由页面或服务端页面；带有 `onclick` 且未传 `href` 的 `SidebarItem` 会渲染为按钮，适用于应用内分区切换（因为 Desktop 渲染器会在系统浏览器中打开锚点链接）；两者都未传入时则仅作为包装容器，供你在内部自行渲染链接或按钮。
- 当 `active` 为真时，锚点与按钮都会渲染 `aria-current="page"`。禁用按钮使用原生 `disabled`，禁用锚点则会移除 `href`，因此两者都不会获得焦点或触发 `onclick`。
- 除 `SidebarRail` 外，其余所有部件均透传全局属性与元素属性，因此可直接为 `Sidebar` 设置 `aria-label`。

## 抽屉模式与快捷键

在 `SidebarProvider` 上设置 `off_canvas` 可使侧边栏在视口宽度小于 `SIDEBAR_MOBILE_QUERY`（768px）时切换为抽屉（off-canvas）面板；向 `Sidebar` 传入 `shortcut` 可绑定 `Ctrl` 或 `Command` 加指定按键的全局快捷键（参见 [RFC 0069](../../../../docs/rfcs/0069-off-canvas-sidebar.md)）：

```rust
let mut mobile_open = use_signal(|| false);

rsx! {
  SidebarProvider {
    off_canvas: true,
    mobile_open: mobile_open(),
    on_mobile_open_change: move |next| mobile_open.set(next),
    SidebarTrigger { "Toggle sidebar" }
    Sidebar { "aria-label": "Main", shortcut: 'b',
      SidebarContent {
        SidebarItem { onclick: move |_| mobile_open.set(false), "Inbox" }
      }
    }
  }
}
```

- 在断点宽度以下，侧边栏会变为贴靠一侧的模态 `dialog` 面板，默认隐藏直至打开，并复用侧边栏的 `aria-label` 作为可访问名称。它会锁定焦点（focus trap）、禁止页面滚动，并在按下 `Escape` 或点击遮罩层时请求关闭；关闭后焦点会移回触发器。由于选择导航项后通常也应关闭抽屉，可按上方示例受控管理 `mobile_open` 以便在菜单项点击时关闭面板。
- 触发器在断点以下切换抽屉面板的开关，在断点以上切换折叠状态；`aria-expanded` 会随当前生效的模式同步更新。快捷键 `shortcut` 的行为与之完全一致。
- 视口宽度在首次渲染之后读取，因此手机端首帧会先按宽屏布局渲染，与服务端渲染行为保持一致。
- 未开启 `off_canvas` 时，侧边栏在任意视口宽度下均渲染为同一个 `aside` 元素。

## 无障碍说明

折叠状态、停靠侧、菜单项激活态与禁用态均通过数据属性暴露，链接型与按钮型菜单项都会通过 `aria-current` 标记当前页面。请使用 `aria-label` 为 `aside` 地标命名，抽屉模式下的 `dialog` 也会沿用该名称。状态持久化由应用自行负责。
