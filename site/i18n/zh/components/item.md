## API 概览

- `Item`
- `ItemMedia`
- `ItemContent`
- `ItemTitle`
- `ItemDescription`
- `ItemActions`
- 各子部件对应的样式类名辅助函数

## 无障碍说明

- `Item` 通过数据属性（data attributes）暴露选中态与禁用态。
- `Item` 自身不添加列表框（listbox）、菜单（menu）、表格（table）或链接（link）语义。
- 点击事件处理、路由跳转、集合容器角色、键盘导航行为，以及特定复合控件模式所需的 `aria-selected` 属性，均由消费方应用自行负责。
