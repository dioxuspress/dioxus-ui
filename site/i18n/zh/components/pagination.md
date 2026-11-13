## API 概览

- `Pagination`
- `PaginationContent`
- `PaginationItem`
- `PaginationLink`
- `PaginationPrevious`
- `PaginationNext`
- `PaginationEllipsis`
- `PaginationRangeItem`
- `pagination_range`
- `pagination_link_class`

## 页码切换

`PaginationLink`、`PaginationPrevious` 与 `PaginationNext` 均接收 `onclick` 回调。未传入 `href` 时，它们会渲染为按钮，便于应用在信号（signal）中维护当前页码：

```rust
let mut page = use_signal(|| 1);

rsx! {
  Pagination {
    PaginationContent {
      PaginationItem {
        PaginationPrevious { disabled: page() == 1, onclick: move |_| page -= 1 }
      }
      for number in 1..=5 {
        PaginationItem { key: "{number}",
          PaginationLink { active: page() == number, onclick: move |_| page.set(number), "{number}" }
        }
      }
      PaginationItem {
        PaginationNext { disabled: page() == 5, onclick: move |_| page += 1 }
      }
    }
  }
}
```

- `href` 为空时会渲染为 `button type="button"`，按 `Enter` 与 `Space` 均可激活。应用内分页请使用此形式，因为 Desktop 渲染器会在系统浏览器中打开锚点链接。
- `href` 非空时会渲染为锚点元素（`a`），适用于服务端渲染页面或路由页面。
- 禁用状态的控件不会触发 `onclick`。禁用按钮使用原生 `disabled` 属性，禁用锚点则会移除 `href`，因此两者都不会获得焦点或响应 `Enter`。
- `id`、`title`、`target` 等全局属性与锚点属性均会透传到底层元素。传入 `aria-label` 可在浏览器端及服务端渲染的 HTML 中替换 `PaginationPrevious` 与 `PaginationNext` 的默认英文无障碍标签；按钮上可见的 `"Previous"` 与 `"Next"` 文本仍保持英文。

## 页码范围计算

`pagination_range(current, total, siblings)` 返回总页数为 `total` 时第 `current` 页应渲染的页码条目列表（两者均从 1 开始计数）：当总页数不超过 `2 * siblings + 5` 时返回全部页码；否则返回首页与末页、`current` 左右两侧各 `siblings` 个相邻页码，并将跨度不少于两页的间隙折叠为 `PaginationRangeItem::Ellipsis`。长页码列表始终保持 `2 * siblings + 5` 个条目，从而在翻页过程中保持分页栏宽度稳定：

```rust
for (index, entry) in pagination_range(page(), 10, 1).into_iter().enumerate() {
  PaginationItem { key: "{index}",
    match entry {
      PaginationRangeItem::Page(number) => rsx! {
        PaginationLink { active: page() == number, onclick: move |_| page.set(number), "{number}" }
      },
      PaginationRangeItem::Ellipsis => rsx! { PaginationEllipsis {} },
    }
  }
}
```

## 窄容器自适应

当单行空间不足时，`PaginationContent` 会将条目居中折行排列，确保在手机屏幕宽度下每个分页链接依然可点击。若希望始终保持单行显示，可配合 `PaginationEllipsis` 减少显示的页码数量。

## 无障碍说明

`Pagination` 会渲染一个带有分页标签的导航区域（`nav`）。请使用 `active` 属性标记当前页，无论是按钮形式还是锚点形式都会自动设置 `aria-current="page"`。
