## API 概览

- `Progress`
- `progress_class`
- `progress_indicator_class`

## 无障碍说明

Progress 使用带有进度值属性的 `role="progressbar"`。当用户需要了解具体是哪项任务正在推进时，请在周围提供说明文本。

ARIA 规范要求进度条必须具有无障碍名称。`Progress` 支持透传 `aria-label`、`aria-labelledby` 和 `aria-valuetext` 等属性（参见 [RFC 0038](../../../../docs/rfcs/0038-form-control-naming.md)）。
