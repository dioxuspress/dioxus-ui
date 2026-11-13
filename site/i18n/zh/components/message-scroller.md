## API 概览

- `MessageScroller`
- `MessageScrollerViewport`
- `MessageScrollerContent`
- `MessageScrollerBottomAnchor`
- `MessageScrollerUnreadMarker`
- `MessageScrollerJumpButton`
- `MessageScrollerMetrics`
- `MessageScrollerEvent`
- `MessageScrollerIntent`
- 纯滚动意图计算辅助函数
- 各组成部分的样式类辅助函数

## 无障碍说明

Message Scroller 默认不添加 `feed`、`log`、`list` 或实时播报区域（live-region）语义。`MessageScrollerViewport` 是一个带有焦点环的 `Tab` 键停靠点（`tabindex="0"`），方便键盘用户滚动浏览，并支持透传 `aria-label` 等属性；当其内部已包含可聚焦元素时，请传入 `tabindex: "-1"`。未读消息标记应包含可见文本。`MessageScrollerJumpButton` 是一个 `type="button"` 按钮，接收 `onclick`（供应用在回调中执行滚动并清除未读状态）并透传其余属性；纯图标的跳转控件需要由应用提供 `aria-label`。追加新消息时不得自动转移焦点。

## 运行时说明

该组件不直接测量 DOM，也不直接执行滚动命令。目前 Web 运行时验证通过以下命令检查测试夹具中的可见前置条件：

```bash
node scripts/runtime-web-verify.mjs
```

Desktop 与 Mobile 端的滚动行为验证暂缓支持。Desktop WebView 的滚动、窗口缩放、设备像素比缩放以及焦点保持需先建立可重复执行的冒烟测试路径，之后才会声明正式支持。Mobile 端的视觉视口（visual viewport）、安全区域（safe area）、软键盘视口、原生滚动以及减弱动态效果（reduced-motion）行为在具备真机或模拟器自动化验证命令之前仅作为文档说明保留。

## 职责边界

Message Scroller 负责静态视口结构组合、可见的未读/跳转状态展示、`data-*` 属性、Tailwind 样式类映射以及纯滚动意图计算逻辑。滚动尺寸测量、滚动命令执行、异步流接入、虚拟滚动、消息 ID 管理、历史记录加载、减弱动态效果策略以及运行时适配器均由应用自行负责。
