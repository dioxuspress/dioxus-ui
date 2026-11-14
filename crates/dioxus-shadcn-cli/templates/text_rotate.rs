//! TextRotate: cycles vertically through a list of phrases with smooth slide-and-fade transitions.

use super::script::{Script, component_script};
use dioxus::prelude::*;
use super::utils::{classes, merge_classes};
use serde::Serialize;
use std::rc::Rc;

component_script!(
  text_rotate_script = r#"
export async function run(dioxus) {
  let timer = null;
  let intervalMs = 2500;
  let active = false;
  let paused = false;

  function schedule() {
    if (timer !== null) {
      clearInterval(timer);
      timer = null;
    }
    if (!active || paused || intervalMs <= 0) {
      return;
    }
    timer = setInterval(() => {
      dioxus.send("tick");
    }, intervalMs);
  }

  try {
    while (true) {
      const msg = await dioxus.recv();
      if (!msg) continue;
      active = Boolean(msg.active);
      paused = Boolean(msg.paused);
      intervalMs = Number(msg.interval_ms) || 2500;
      schedule();
    }
  } finally {
    if (timer !== null) {
      clearInterval(timer);
    }
  }
}
"#
);

#[derive(Serialize)]
struct TextRotateScriptMessage {
  active: bool,
  paused: bool,
  interval_ms: u64,
}

const TEXT_ROTATE_BASE_CLASS: &str = "inline-grid overflow-hidden align-bottom";

/// Classes for the `TextRotate` container.
pub fn text_rotate_class(class: &str) -> String {
  merge_classes(classes([Some(TEXT_ROTATE_BASE_CLASS)]), class)
}

/// Vertically rotates through `items` at `interval_ms`, pausing on hover when `pause_on_hover` is set.
#[component]
pub fn TextRotate(
  items: Vec<String>,
  #[props(default = 2500)] interval_ms: u64,
  #[props(default = true)] pause_on_hover: bool,
  #[props(default)] index: Option<usize>,
  #[props(default)] class: String,
  #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
  let total = items.len();
  let mut internal_index = use_signal(|| 0usize);
  let mut hovered = use_signal(|| false);
  let is_controlled = index.is_some();

  let current = if total == 0 { 0 } else { index.unwrap_or(internal_index()) % total };

  let mut script_signal = use_signal(|| None::<Rc<Script>>);

  use_effect(move || {
    if is_controlled || total <= 1 {
      script_signal.with(|s| {
        if let Some(script) = s {
          let _ =
            script.send(TextRotateScriptMessage { active: false, paused: false, interval_ms });
        }
      });
      return;
    }
    if script_signal.with(|s| s.is_none()) {
      let script = Rc::new(text_rotate_script::start());
      let reader = Rc::clone(&script);
      spawn(async move {
        while let Ok(_msg) = reader.recv::<String>().await {
          let len = total.max(1);
          internal_index.set((internal_index() + 1) % len);
        }
      });
      script_signal.set(Some(script));
    }
    script_signal.with(|s| {
      if let Some(script) = s {
        let _ = script.send(TextRotateScriptMessage {
          active: true,
          paused: pause_on_hover && hovered(),
          interval_ms,
        });
      }
    });
  });

  let class = text_rotate_class(&class);

  rsx! {
    span {
      class,
      "data-active-index": "{current}",
      onmouseenter: move |_| hovered.set(true),
      onmouseleave: move |_| hovered.set(false),
      ..attributes,
      for (idx, item) in items.iter().enumerate() {
        span {
          key: "{idx}-{item}",
          class: if idx == current {
            "col-start-1 row-start-1 inline-block translate-y-0 opacity-100 transition-all duration-300 ease-out"
          } else {
            "col-start-1 row-start-1 inline-block -translate-y-full opacity-0 transition-all duration-300 ease-out pointer-events-none select-none"
          },
          "aria-hidden": (idx != current).to_string(),
          "{item}"
        }
      }
    }
  }
}
