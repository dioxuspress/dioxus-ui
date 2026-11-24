use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, ChartBarSeries, ChartColorToken, ChartDescription,
  ChartDomain, ChartFallbackTable, ChartLegend, ChartPoint, ChartRoot, ChartScale, ChartSeries,
  ChartSvg, ChartTitle, ChartTooltipSlot, chart_fallback_rows,
};

const MONTHS: [&str; 5] = ["Jan", "Feb", "Mar", "Apr", "May"];
const DESKTOP: [f64; 5] = [18.0, 26.0, 21.0, 32.0, 29.0];
const MOBILE: [f64; 5] = [11.0, 15.0, 19.0, 24.0, 22.0];

#[component]
pub fn ChartBarDemo() -> Element {
  let mut active = use_signal(|| Some(3_usize));
  let desktop = ChartSeries::new(
    "desktop",
    "Desktop",
    DESKTOP.iter().enumerate().map(|(i, v)| ChartPoint::new(i as f64 - 0.14, *v)).collect(),
  );
  let mobile = ChartSeries::new(
    "mobile",
    "Mobile",
    MOBILE.iter().enumerate().map(|(i, v)| ChartPoint::new(i as f64 + 0.14, *v)).collect(),
  );
  let x = ChartScale::new(ChartDomain::new(-0.6, 4.6), ChartDomain::new(40.0, 600.0));
  let y = ChartScale::new(ChartDomain::new(0.0, 36.0), ChartDomain::new(280.0, 32.0));
  let rows = chart_fallback_rows(&[desktop.clone(), mobile.clone()]);

  rsx! {
    div { class: "grid max-w-xl gap-4",
      div { class: "flex flex-wrap gap-2",
        for (idx, month) in MONTHS.iter().enumerate() {
          Button {
            key: "{month}",
            size: ButtonSize::Xs,
            variant: if active() == Some(idx) { ButtonVariant::Primary } else { ButtonVariant::Outline },
            onclick: move |_| active.set(Some(idx)),
            "{month}"
          }
        }
        Button {
          size: ButtonSize::Xs,
          variant: ButtonVariant::Ghost,
          onclick: move |_| active.set(None),
          "Hide tooltip"
        }
      }
      ChartRoot { class: "relative max-w-xl",
        ChartTitle { id: "chart-bar-title", "Monthly active sessions" }
        ChartDescription { id: "chart-bar-description", "Desktop versus mobile sessions in thousands, January to May." }
        ChartSvg { title_id: "chart-bar-title", description_id: "chart-bar-description",
          ChartBarSeries { series: desktop, x_scale: x, y_scale: y, color: ChartColorToken::Chart1, bar_width: 24.0 }
          ChartBarSeries { series: mobile, x_scale: x, y_scale: y, color: ChartColorToken::Chart2, bar_width: 24.0 }
        }
        ChartTooltipSlot {
          visible: active().is_some(),
          x: active().map_or(0.0, |i| 24.0 + i as f64 * 68.0),
          y: 12.0,
          if let Some(idx) = active() {
            div { class: "grid gap-0.5 text-xs",
              span { class: "font-semibold", "{MONTHS[idx]}" }
              span { class: "text-muted-foreground", "Desktop: {DESKTOP[idx]:.0}k · Mobile: {MOBILE[idx]:.0}k" }
            }
          }
        }
        ChartLegend {
          span { class: "inline-flex items-center gap-2",
            span { class: "h-2.5 w-2.5 rounded-sm bg-chart-1" }
            "Desktop"
          }
          span { class: "inline-flex items-center gap-2",
            span { class: "h-2.5 w-2.5 rounded-sm bg-chart-2" }
            "Mobile"
          }
        }
        ChartFallbackTable { rows, caption: "Sessions by platform and month" }
      }
    }
  }
}
