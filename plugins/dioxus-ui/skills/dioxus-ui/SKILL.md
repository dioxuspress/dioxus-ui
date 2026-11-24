---
name: dioxus-ui
description: Use when building, editing, or theming Dioxus 0.7 Rust UIs with dioxus-shadcn (dioxus-ui), choosing components or blocks, configuring Cargo component features, running dxui init/add/diff/theme workflows, or troubleshooting Tailwind CSS v4 tokens and CSP compatibility.
---

# Dioxus UI (`dioxus-shadcn`) Implementation Workflow

Follow this 5-step workflow whenever working with `dioxus-shadcn` (`dioxus-ui`) in a Rust / Dioxus 0.7 project. The installed library and the app's existing source files are always the source of truth.

Reference files bundled inside this plugin (`${CLAUDE_PLUGIN_ROOT}/skills/dioxus-ui/references/`):

- [catalog.md](references/catalog.md) — All 100 public components, categories, Cargo features, Rust module names, and `dxui add` commands.
- [blocks.md](references/blocks.md) — All 11 full-screen blocks, commands, and required component dependencies.
- [themes.md](references/themes.md) — All 33 WCAG-AA theme presets (`light`/`dark`), semantic tokens, and dark-mode patterns.
- [api-patterns.md](references/api-patterns.md) — Dioxus 0.7 RSX conventions, controlled/uncontrolled state, `merge_classes`, `DensityProvider`, and no-`'unsafe-eval'` CSP guidance.

---

## Step 1: Inspect the Target Project First

Before recommending any component, import, feature, or CLI command, inspect the workspace:

1. Read `Cargo.toml` (and `Cargo.lock` if present) to detect:
   - `dioxus` version (this plugin targets Dioxus `0.7.x` and Rust `1.88+`).
   - Whether the app uses **Crate Mode** (`dioxus-shadcn` listed in `[dependencies]` with `default-features = false, features = [...]`), **Source-Copy Mode** (`src/components/ui/mod.rs` and/or `src/blocks/mod.rs` exist), or both.
   - Exact installed `dioxus-shadcn` version from `Cargo.lock` or `Cargo.toml`.
2. If source-copy files exist or the user asks to run `dxui`, check `dxui --version` (when a shell is available) and inspect `src/components/ui/mod.rs`, `src/blocks/mod.rs`, and `assets/dioxus-shadcn.css` (or the app's Tailwind v4 input stylesheet).

Do not guess the mode or assume the latest `main` branch APIs if the project pins an older release.

---

## Step 2: Enforce Release & Version Alignment

This plugin snapshot is synchronized with **`dioxus-shadcn` `0.7.0`** (`dxui` `0.7.0`).

- **If the project uses Source-Copy Mode**: read the existing file under `src/components/ui/<component>.rs` or `src/blocks/<block>.rs` before writing call sites. Local copies may have custom props, variants, or older signatures—match what is actually on disk.
- **If the project pins an older Crate Mode release** (e.g., `< 0.7.0`): verify available features and exported items in the installed crate (`cargo metadata` / `~/.cargo/registry/src/.../dioxus-shadcn-<version>/src/lib.rs`) before introducing newer components, blocks, or props. Never suggest APIs that exist only on `main` or in a newer minor release without flagging the required upgrade.
- **If a requested component does not exist in the installed release**: tell the user clearly, show the closest existing component in `references/catalog.md`, or explain the exact crate/`dxui` upgrade needed.

---

## Step 3: Select Components & Blocks and Compose Dioxus 0.7 RSX

1. Check [blocks.md](references/blocks.md) first when the user wants a full screen (`login`, `signup`, `dashboard`, `settings`, `chat`, `inbox`, `files`, `checkout`, `landing`, `pricing`, `schedule`). Blocks copy starter screens into `src/blocks/` and pull in their component dependencies automatically.
2. Check [catalog.md](references/catalog.md) to pick exact component slugs, Cargo features, and module names.
3. Follow [api-patterns.md](references/api-patterns.md) for idiomatic Dioxus 0.7 RSX:
   - **Crate Mode imports**: `use dioxus_shadcn::{Button, ButtonVariant, Dialog, DialogContent, DialogOverlay, DialogTitle, DialogTrigger};` (or module-scoped `use dioxus_shadcn::button::{Button, ButtonVariant};`).
   - **Source-Copy Mode imports**: `use crate::components::ui::button::{Button, ButtonVariant};` and for blocks `mod blocks; blocks::login::LoginBlock { ... }`.
   - Use Dioxus 0.7 APIs only (`#[component]`, `use_signal`, `use_memo`, `EventHandler`, `rsx!`). Never use removed Dioxus 0.5/0.6 APIs (`cx`, `Scope`, `use_state`).
   - Respect component-owned state (`Dialog`, `Tabs`, `Accordion`, `Select`, `Popover`, etc. own their open/value state by default via `default_*` and support controlled mode via `open` + `on_open_change` or `value` + `on_value_change`).
   - Always render an accessible name for overlays (e.g., `DialogTitle` inside `DialogContent`, or `aria-label` when visually omitted) and for icon-only buttons.
   - Use `DensityProvider { density: UiDensity::Touch, ... }` for touch/mobile roots instead of passing a `density` prop to individual components.

---

## Step 4: Configure Dependencies or Run `dxui` Safely

### Crate Mode

Enable only the component features used by the app (`default-features = false` avoids compiling unused components):

```toml
[dependencies]
dioxus-shadcn = { version = "0.6", default-features = false, features = ["button", "card", "dialog", "input"] }
```

Run `dxui init` once (or after upgrading `dioxus-shadcn`) so `assets/dioxus-shadcn.css` contains the semantic tokens and the Tailwind v4 `@source` path pointing to the installed crate's source files.

### Source-Copy Mode

```bash
dxui init
dxui list
dxui list blocks
dxui add button card dialog input
dxui add login
```

Critical rules when modifying or updating source-copy projects:

- **Default `dxui add` is non-destructive**: existing modified files are kept untouched and reported as unchanged or kept.
- **Never run `--overwrite` blindly**: inspect local customizations first with `dxui diff <component>` (or `dxui diff` for all installed components and blocks). Explain the unified diff to the user and only use `dxui add <name> --overwrite` when they explicitly want to replace local edits.
- **Page-script helper dependencies**: interactive components that ship `script.rs` (`dialog`, `tabs`, `slider`, menus, overlays, `theme-controller`, etc.) require `serde = "1"` and wasm32 `serde_json = "1"` + `wasm-bindgen = "0.2"` in `Cargo.toml` (`dxui add` reports any missing entries).

---

## Step 5: Apply Themes, Tokens, and CSP Rules

Consult [themes.md](references/themes.md) for the full token set and 33 presets:

1. **Tailwind CSS v4 input stylesheet**: keep `assets/dioxus-shadcn.css` as a Tailwind v4 input (`@import "tailwindcss";`, `@custom-variant dark (&:is(.dark *));`, `:root`, `.dark`, `@theme inline`). Do not replace it with Tailwind v3 `@tailwind base/components/utilities` directives or precompiled CSS.
2. **Semantic tokens over raw palette classes**: prefer `bg-background`, `text-foreground`, `bg-card`, `text-muted-foreground`, `bg-primary`, `text-primary-foreground`, `border-border`, `bg-destructive`, `text-success`, `text-warning`, `text-info` so surfaces automatically adapt across light, dark, and preset themes.
3. **Rebranding**: override token variables in `:root` and `.dark` (e.g., `--primary`, `--primary-foreground`, `--ring`, `--radius`) instead of editing every component.
4. **Dark mode & presets**:
   - Opt-in dark mode: add the `dark` class on the root container.
   - Presets: run `dxui theme list`, then `dxui theme add nord dracula`, and set `"data-theme": "nord"` on a container (or use `ThemeController` + `Theme::Preset("nord".into())`). Dark presets set `color-scheme: dark` automatically without needing the `dark` class.
5. **Strict CSP (no `'unsafe-eval'`)**:
   - `dioxus-shadcn` page scripts execute as wasm-bindgen snippets and work under `script-src 'self' 'wasm-unsafe-eval'`.
   - Avoid adding `document::eval` or `document::Title` in strict-CSP web apps (set `[web.app] title` in `Dioxus.toml` instead).
   - If using `theme_init_script(THEME_STORAGE_KEY)` in `<head>` to prevent FOUC, allow that inline script via its SHA-256 hash or nonce.
   - Never construct Tailwind utility class names dynamically via string interpolation (`format!("bg-{color}-500")`)—Tailwind v4 static scanning will omit them.
