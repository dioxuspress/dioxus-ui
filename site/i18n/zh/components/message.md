## API 概览

- `Message`
- `MessageGroup`
- `MessageAvatar`
- `MessageContent`
- `MessageHeader`
- `MessageFooter`
- `MessageAlign`
- 各组成部分的样式类辅助函数

## 无障碍说明

Message 不会自动添加列表（list）、信息流（feed）、文章（article）、日志（log）或实时播报区域（live-region）语义。当对话记录需要这些角色时，由应用自行在外层指定。底部栏中的纯图标操作按钮需要由应用提供无障碍名称。

## 职责边界

Message 负责行级布局、对齐方式以及头像、头部、正文内容与底部的摆放位置。发送者身份、助手/用户/工具角色区分、时间戳、送达状态、Markdown 解析、代码语法高亮、流式输出以及模型服务商集成仍由应用自行负责。
