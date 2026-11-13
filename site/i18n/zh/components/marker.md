## API 概览

- `Marker`
- `MarkerIcon`
- `MarkerContent`
- `MarkerVariant`
- `marker_class`
- `marker_icon_class`
- `marker_content_class`

## 无障碍说明

`MarkerIcon` 默认为装饰性元素。仅当图标承载了应用赋予的无障碍语义时，才传入 `decorative: false`。

对于流式输出、进度展示或异步状态场景，应用应在需要处显式设置状态角色或实时播报区域（live-region）语义。Marker 自身不会主动播报状态更新。

## 职责边界

Marker 负责内联状态、带边框行以及带标签分隔线的布局与样式。搜索索引、引用解析、工具执行状态、对话记录压缩以及气泡浮层详情仍由应用自行管理。
