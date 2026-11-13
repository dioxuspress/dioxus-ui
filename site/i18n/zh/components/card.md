## API 概览

- `Card`
- `CardHeader`
- `CardTitle`
- `CardDescription`
- `CardContent`
- `CardFooter`
- `card_class`

## 无障碍说明

卡片组件自身不添加地标（landmark）或交互语义。与 shadcn/ui v4 一致，`CardTitle` 渲染为 `div`；当卡片标题需要作为文档标题时，请根据页面大纲所需的层级在内部自行包裹对应的标题元素（如 `h2`、`h3`）。
