use dioxus::prelude::*;
use dioxus_shadcn::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};

const METRICS: [(&str, &str, &str, &str); 4] = [
  ("us-east-1", "99.98%", "24 ms", "412 req/s"),
  ("eu-central-1", "99.95%", "31 ms", "289 req/s"),
  ("ap-northeast-1", "99.99%", "19 ms", "356 req/s"),
  ("ap-southeast-1", "99.92%", "42 ms", "198 req/s"),
];

#[component]
pub fn TableStripedDemo() -> Element {
  rsx! {
    div { class: "w-full min-w-0 overflow-hidden rounded-md border border-border",
      Table { striped: true, compact: true, sticky_header: true,
        TableHeader { class: "bg-muted/50",
          TableRow {
            TableHead { "Region" }
            TableHead { "Uptime" }
            TableHead { class: "text-right", "P95 Latency" }
            TableHead { class: "text-right", "Throughput" }
          }
        }
        TableBody {
          for (region, uptime, latency, rps) in METRICS {
            TableRow { key: "{region}",
              TableCell { class: "font-mono font-medium", "{region}" }
              TableCell { "{uptime}" }
              TableCell { class: "text-right tabular-nums", "{latency}" }
              TableCell { class: "text-right tabular-nums", "{rps}" }
            }
          }
        }
      }
    }
  }
}
