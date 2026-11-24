use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeSize, BadgeStyle, BadgeVariant, Button, ButtonSize, ButtonVariant, Input, InputSize,
  Label, Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetOverlay,
  SheetSide, SheetTitle, SheetTrigger, Switch, button_class, use_density,
};

#[component]
pub fn SheetFilterDemo() -> Element {
  let mut open = use_signal(|| false);
  let mut keyword = use_signal(String::new);
  let mut in_stock = use_signal(|| true);
  let mut featured = use_signal(|| false);
  let mut on_sale = use_signal(|| false);

  let active_count =
    usize::from(!keyword().trim().is_empty()) + usize::from(in_stock()) + usize::from(featured()) + usize::from(on_sale());

  rsx! {
    div { class: "flex flex-wrap items-center gap-3",
      Sheet { open: open(), on_open_change: move |next| open.set(next),
        SheetTrigger {
          class: button_class(ButtonVariant::Outline, ButtonSize::Sm, use_density(), "gap-2"),
          span { "Filters" }
          Badge {
            variant: BadgeVariant::Secondary,
            style: BadgeStyle::Soft,
            size: BadgeSize::Xs,
            "{active_count}"
          }
        }
        SheetOverlay {}
        SheetContent { side: SheetSide::Right, class: "flex flex-col justify-between",
          div { class: "grid gap-5",
            SheetHeader {
              SheetTitle { "Filter catalog" }
              SheetDescription { "Narrow down items before exporting or sharing." }
            }
            div { class: "grid gap-2",
              Label { r#for: "sheet-filter-keyword", "Keyword" }
              Input {
                id: "sheet-filter-keyword",
                size: InputSize::Sm,
                placeholder: "Search by name or SKU...",
                value: keyword(),
                on_value_change: move |value| keyword.set(value),
              }
            }
            div { class: "grid gap-3 border-t border-border pt-4",
              div { class: "flex items-center justify-between gap-3",
                Label { r#for: "sheet-filter-stock", "In stock only" }
                Switch {
                  id: "sheet-filter-stock",
                  checked: in_stock(),
                  on_checked_change: move |next| in_stock.set(next),
                }
              }
              div { class: "flex items-center justify-between gap-3",
                Label { r#for: "sheet-filter-featured", "Featured releases" }
                Switch {
                  id: "sheet-filter-featured",
                  checked: featured(),
                  on_checked_change: move |next| featured.set(next),
                }
              }
              div { class: "flex items-center justify-between gap-3",
                Label { r#for: "sheet-filter-sale", "On sale" }
                Switch {
                  id: "sheet-filter-sale",
                  checked: on_sale(),
                  on_checked_change: move |next| on_sale.set(next),
                }
              }
            }
          }
          SheetFooter {
            Button {
              variant: ButtonVariant::Ghost,
              onclick: move |_| {
                keyword.set(String::new());
                in_stock.set(false);
                featured.set(false);
                on_sale.set(false);
              },
              "Reset"
            }
            Button { onclick: move |_| open.set(false), "Apply filters" }
          }
          SheetClose { "×" }
        }
      }
      p { class: "text-sm text-muted-foreground", "{active_count} active filters" }
    }
  }
}
