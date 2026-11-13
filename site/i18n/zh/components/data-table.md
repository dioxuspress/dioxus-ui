## API 概览

- `DataTable { class, children }`
- `DataTableToolbar { class, children }`
- `DataTableContainer { class, children }`
- `DataTableHeaderCell { sorted, direction, class, children }`
- `DataTableRow { selected, disabled, class, children }`
- `DataTableCell { hidden, class, children }`
- `DataTablePagination { class, children }`
- `DataTableSelectedCount { count, total, class }`
- `DataTableEmpty { class, children }`
- `DataTableLoading { class, children }`

状态辅助函数：

- `data_table_toggle_sort(current, column_id)`
- `data_table_page_count(total_items, page_size)`
- `data_table_page_window(page, page_size, total_items)`
- `data_table_toggle_row(selected_ids, row_id)`
- `data_table_toggle_all_rows(selected_ids, visible_ids)`
- `data_table_toggle_column(hidden_ids, column_id)`
- `data_table_is_column_visible(hidden_ids, column_id)`

## 无障碍说明

请在 `DataTableContainer` 内使用语义化的 `Table` 部件或原生表格标记。排序表头映射为 `aria-sort`，选中行映射为 `data-selected`，隐藏单元格使用 `hidden` 属性，加载状态内容使用 `role="status"`。筛选、键盘快捷键、异步数据加载与虚拟滚动仍由应用自行管理。
