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
      Table { class: "text-xs",
        TableHeader { class: "bg-muted/50",
          TableRow {
            TableHead { class: "h-9 px-3", "Region" }
            TableHead { class: "h-9 px-3", "Uptime" }
            TableHead { class: "h-9 px-3 text-right", "P95 Latency" }
            TableHead { class: "h-9 px-3 text-right", "Throughput" }
          }
        }
        TableBody {
          for (region, uptime, latency, rps) in METRICS {
            TableRow { key: "{region}", class: "even:bg-muted/30",
              TableCell { class: "px-3 py-2 font-mono font-medium", "{region}" }
              TableCell { class: "px-3 py-2", "{uptime}" }
              TableCell { class: "px-3 py-2 text-right tabular-nums", "{latency}" }
              TableCell { class: "px-3 py-2 text-right tabular-nums", "{rps}" }
            }
          }
        }
      }
    }
  }
}
