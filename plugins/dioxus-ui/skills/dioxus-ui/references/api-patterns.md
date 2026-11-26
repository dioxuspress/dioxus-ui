# Dioxus UI (`dioxus-shadcn`) API Patterns & Conventions

Concise reference distilled from `docs/component-api.md` and shipped components for `dioxus-shadcn` `0.8.0` (Dioxus `0.7.x`).

## Imports: Crate Mode vs. Source-Copy Mode

```rust
// Crate mode (`Cargo.toml`: dioxus-shadcn = { version = "0.8", default-features = false, features = ["button", "card", "dialog", "input"] })
use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, Card, CardContent, CardHeader, CardTitle,
  Dialog, DialogClose, DialogContent, DialogOverlay, DialogTitle, DialogTrigger,
  Input,
};

// Source-copy mode (`dxui add button card dialog input`)
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::ui::dialog::{
  Dialog, DialogClose, DialogContent, DialogOverlay, DialogTitle, DialogTrigger,
};
use crate::components::ui::input::Input;
```

## Common Props & Naming

- `class: String` (default empty): merged last via `merge_classes`, so user utilities override conflicting component utilities of the same property/variant (e.g. `bg-blue-100` replaces `bg-muted`, while `hover:*` stays). For physical vs. logical conflicts (e.g. `ms-2` over `ml-4`), append Tailwind's important modifier (`ms-2!`).
- `children: Element`: standard on container/content components (`Button`, `Card`, `Alert`, `Dialog`, `Tabs`). Void inputs (`Input`, `Separator`, `Progress`) omit `children`.
- Semantic `variant` enums (never color names): e.g. `ButtonVariant::{Primary, Secondary, Destructive, Outline, Ghost, Link, Soft, Dash}`.
- Component-local `size` enums: e.g. `ButtonSize::{Xs, Sm, Md, Lg, IconSm, Icon, IconLg}`.
- Boolean state props without `is_` prefix: `disabled`, `invalid`, `loading`.
- Events follow Dioxus `on_*` naming (`onclick`, `oninput`, `on_open_change`, `on_value_change`, `on_checked_change`).

## Uncontrolled vs. Controlled State

Interactive composite components (`Dialog`, `AlertDialog`, `Sheet`, `Drawer`, `Popover`, `Dropdown`, `Select`, `Tabs`, `Accordion`, `Collapsible`) own their state by default and accept controlled props when the app needs external control:

```rust
// Uncontrolled Dialog (component owns open state)
rsx! {
  Dialog {
    DialogTrigger {
      class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
      "Rename project"
    }
    DialogOverlay {}
    DialogContent {
      DialogTitle { "Rename project" }
      DialogClose { "Cancel" }
    }
  }
}

// Controlled Dialog (app signal owns open state)
let mut open = use_signal(|| false);
rsx! {
  Button { onclick: move |_| open.set(true), "Open dialog" }
  Dialog {
    open: open(),
    on_open_change: move |next| open.set(next),
    DialogOverlay {}
    DialogContent {
      DialogTitle { "Confirm action" }
      Button { onclick: move |_| open.set(false), "Confirm" }
    }
  }
}
```

## Touch & Compact Density (`DensityProvider`)

Never pass a `density` prop to individual components. Wrap a subtree (or app root) once:

```rust
use dioxus_shadcn::{DensityProvider, UiDensity}; // or crate::components::ui::density::{...}

rsx! {
  DensityProvider { density: UiDensity::Touch,
    // Interactive controls automatically expand to 44x44 CSS px targets
    AppShell {}
  }
}
```

Available values: `UiDensity::Compact`, `UiDensity::Comfortable` (default), `UiDensity::Touch`.

## Theme Controller & Avoiding Dark-Mode Flash

```rust
use dioxus_shadcn::{Theme, ThemeController, ToggleGroup, ToggleGroupItem};

let mut theme = use_signal(|| Theme::System);

rsx! {
  ThemeController {
    theme: theme(),
    on_theme_change: move |stored| theme.set(stored),
  }
  ToggleGroup {
    "aria-label": "Theme",
    value: theme().as_str().to_string(),
    on_value_change: move |value: String| {
      if !value.is_empty() {
        theme.set(Theme::parse(&value));
      }
    },
    for (option, label) in [(Theme::System, "System"), (Theme::Light, "Light"), (Theme::Dark, "Dark")] {
      ToggleGroupItem { key: "{label}", value: option.as_str().to_string(), "{label}" }
    }
  }
}
```

To prevent a light-mode flash before hydration, inject `theme_init_script(THEME_STORAGE_KEY)` in `<head>` and allow that inline script via its CSP hash or nonce.

## Styling, Security, and CSP Guardrails

- **No dynamic Tailwind class interpolation**: always write complete static Tailwind utility tokens in string literals (`if active { "bg-primary text-primary-foreground" } else { "bg-muted text-muted-foreground" }`), never `format!("bg-{token}")`.
- **Strict CSP**: library page scripts work under `script-src 'self' 'wasm-unsafe-eval'`. Avoid `document::eval` and `document::Title` in strict-CSP web apps; configure `[web.app] title` in `Dioxus.toml`.
- **Safe URLs & escaping**: no component uses `dangerous_inner_html`; link props allow relative paths, fragments, `http`, `https`, `mailto`, and `tel` only.
- **No `unwrap()`/`expect()` in app handlers**: handle `Result`/`Option` cleanly in callbacks and form validation.
