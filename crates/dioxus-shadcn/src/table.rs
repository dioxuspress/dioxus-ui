//! Table: styled semantic table parts for tabular data.

use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

const TABLE_CONTAINER_BASE_CLASS: &str = "relative w-full overflow-auto";
const TABLE_BASE_CLASS: &str = "w-full caption-bottom text-sm";
const TABLE_HEADER_BASE_CLASS: &str = "[&_tr]:border-b";
const TABLE_BODY_BASE_CLASS: &str = "[&_tr:last-child]:border-0";
const TABLE_FOOTER_BASE_CLASS: &str =
  "border-t border-border bg-muted font-medium [&>tr]:last:border-b-0";
const TABLE_ROW_BASE_CLASS: &str = "border-b border-border transition-colors hover:bg-muted";
const TABLE_HEAD_CORE_CLASS: &str = "text-left align-middle font-medium text-muted-foreground";
const TABLE_CELL_CORE_CLASS: &str = "align-middle";
const TABLE_CAPTION_BASE_CLASS: &str = "mt-4 text-sm text-muted-foreground";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct TableContext {
  striped: bool,
  compact: bool,
  sticky_header: bool,
}

fn use_table_context() -> TableContext {
  try_use_context::<TableContext>().unwrap_or_default()
}

fn table_container_class(class: &str) -> String {
  merge_classes(classes([Some(TABLE_CONTAINER_BASE_CLASS)]), class)
}

/// Classes for the `table` element, with `class` merged over them.
pub fn table_class(class: &str) -> String {
  merge_classes(classes([Some(TABLE_BASE_CLASS)]), class)
}

fn table_header_class(sticky_header: bool, class: &str) -> String {
  let sticky = if sticky_header { Some("sticky top-0 z-10 bg-background") } else { None };
  merge_classes(classes([Some(TABLE_HEADER_BASE_CLASS), sticky]), class)
}

fn table_body_class(striped: bool, class: &str) -> String {
  let zebra = if striped { Some("[&_tr:nth-child(even)]:bg-muted/40") } else { None };
  merge_classes(classes([Some(TABLE_BODY_BASE_CLASS), zebra]), class)
}

fn table_footer_class(class: &str) -> String {
  merge_classes(classes([Some(TABLE_FOOTER_BASE_CLASS)]), class)
}

fn table_row_class(class: &str) -> String {
  merge_classes(classes([Some(TABLE_ROW_BASE_CLASS)]), class)
}

fn table_head_class(compact: bool, class: &str) -> String {
  let density = if compact { "h-9 px-3 text-xs" } else { "h-12 px-4" };
  merge_classes(classes([Some(TABLE_HEAD_CORE_CLASS), Some(density)]), class)
}

fn table_cell_class(compact: bool, class: &str) -> String {
  let density = if compact { "px-3 py-2 text-xs" } else { "p-4" };
  merge_classes(classes([Some(TABLE_CELL_CORE_CLASS), Some(density)]), class)
}

fn table_caption_class(class: &str) -> String {
  merge_classes(classes([Some(TABLE_CAPTION_BASE_CLASS)]), class)
}

/// Scrollable semantic table wrapper with optional zebra striping, compact cell
/// density, and a sticky header row.
#[component]
pub fn Table(
  #[props(default)] striped: bool,
  #[props(default)] compact: bool,
  #[props(default)] sticky_header: bool,
  #[props(default)] class: String,
  children: Element,
) -> Element {
  use_context_provider(|| TableContext { striped, compact, sticky_header });
  let container_class = table_container_class("");
  let class = table_class(&class);

  rsx! {
    div {
      class: container_class,
      table {
        class,
        "data-striped": striped.then_some("true"),
        "data-compact": compact.then_some("true"),
        "data-sticky-header": sticky_header.then_some("true"),
        {children}
      }
    }
  }
}

/// Header section (`<thead>`) of a [`Table`].
#[component]
pub fn TableHeader(#[props(default)] class: String, children: Element) -> Element {
  let ctx = use_table_context();
  let class = table_header_class(ctx.sticky_header, &class);

  rsx! {
    thead {
      class,
      {children}
    }
  }
}

/// Body section (`<tbody>`) of a [`Table`].
#[component]
pub fn TableBody(#[props(default)] class: String, children: Element) -> Element {
  let ctx = use_table_context();
  let class = table_body_class(ctx.striped, &class);

  rsx! {
    tbody {
      class,
      {children}
    }
  }
}

/// Footer section (`<tfoot>`) of a [`Table`].
#[component]
pub fn TableFooter(#[props(default)] class: String, children: Element) -> Element {
  let class = table_footer_class(&class);

  rsx! {
    tfoot {
      class,
      {children}
    }
  }
}

/// Row (`<tr>`) inside a [`Table`].
#[component]
pub fn TableRow(#[props(default)] class: String, children: Element) -> Element {
  let class = table_row_class(&class);

  rsx! {
    tr {
      class,
      {children}
    }
  }
}

/// Header cell (`<th>`) inside a [`TableHeader`] row.
#[component]
pub fn TableHead(#[props(default)] class: String, children: Element) -> Element {
  let ctx = use_table_context();
  let class = table_head_class(ctx.compact, &class);

  rsx! {
    th {
      class,
      {children}
    }
  }
}

/// Data cell (`<td>`) inside a [`TableRow`].
#[component]
pub fn TableCell(#[props(default)] class: String, children: Element) -> Element {
  let ctx = use_table_context();
  let class = table_cell_class(ctx.compact, &class);

  rsx! {
    td {
      class,
      {children}
    }
  }
}

/// Caption (`<caption>`) describing a [`Table`].
#[component]
pub fn TableCaption(#[props(default)] class: String, children: Element) -> Element {
  let class = table_caption_class(&class);

  rsx! {
    caption {
      class,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn table_class_appends_user_class() {
    let actual = table_class("min-w-lg");

    assert!(actual.contains(TABLE_BASE_CLASS));
    assert!(actual.ends_with("min-w-lg"));
  }

  #[test]
  fn table_row_class_appends_user_class() {
    let actual = table_row_class("data-[selected=true]:bg-muted");

    assert!(actual.contains(TABLE_ROW_BASE_CLASS));
    assert!(actual.ends_with("data-[selected=true]:bg-muted"));
  }

  #[test]
  fn table_flags_propagate_striped_compact_and_sticky_header() {
    fn app() -> Element {
      rsx! {
        Table { striped: true, compact: true, sticky_header: true,
          TableHeader {
            TableRow { TableHead { "Region" } }
          }
          TableBody {
            TableRow { TableCell { "us-east-1" } }
          }
        }
      }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains(r#"data-striped="true""#), "{html}");
    assert!(html.contains(r#"data-compact="true""#), "{html}");
    assert!(html.contains(r#"data-sticky-header="true""#), "{html}");
    assert!(html.contains("sticky top-0 z-10 bg-background"), "{html}");
    assert!(table_body_class(true, "").contains("[&_tr:nth-child(even)]:bg-muted/40"));
    assert!(html.contains("h-9 px-3 text-xs"), "{html}");
    assert!(html.contains("px-3 py-2 text-xs"), "{html}");
  }
}
