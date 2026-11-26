# Dioxus UI Claude Code Plugin

Official, maintainer-owned Claude Code plugin for [`dioxus-shadcn`](https://github.com/dioxuspress/dioxus-ui) (`dioxus-ui`).

It equips Claude Code with a single component-focused skill and compact, source-generated references covering all **100 components**, **11 screen blocks**, **33 WCAG-AA theme presets**, crate-mode vs. source-copy (`dxui`) workflows, and Dioxus 0.7 API conventions.

## Supported Versions & Release Alignment

- **Plugin version**: `0.7.1` (version-locked to `Cargo.toml` `[workspace.package].version`)
- **Supported library & CLI**: `dioxus-shadcn` `0.7.x`, `dioxus-shadcn-cli` (`dxui`) `0.7.x`, Dioxus `0.7.x`, Rust `1.88+`, Tailwind CSS v4
- **How guidance stays aligned with releases**:
  - The plugin lives in the same repository as `dioxus-shadcn` and is version-bumped alongside crate releases.
  - Its component catalog, block inventory, and theme preset lists (`skills/dioxus-ui/references/*.md`) are generated directly from `crates/dioxus-shadcn-cli/{registry,blocks,themes}` and `crates/dioxus-shadcn` and verified by `npm run verify:claude-plugin` in CI.
  - At runtime, the skill instructs Claude Code to inspect the target app's `Cargo.toml`, `Cargo.lock`, `dxui --version`, and local `src/components/ui/` copies first so APIs or components introduced only on `main` are not suggested for an older installed release.

## Installation (macOS, Linux, and Windows)

Add the official `dioxuspress` marketplace once, then install the `dioxus-ui` plugin at the scope you need. The `claude plugin` CLI commands are identical across macOS, Linux, and Windows (PowerShell, Command Prompt, or Git Bash / WSL).

### 1. Register the marketplace

```bash
claude plugin marketplace add dioxuspress/dioxus-ui
```

Or interactively inside a Claude Code session:

```text
/plugin marketplace add dioxuspress/dioxus-ui
```

### 2. Install the plugin (choose a scope)

| Scope | Who receives it | Stored in | Command |
| :--- | :--- | :--- | :--- |
| **`user`** | You, across all projects on this machine | `~/.claude/settings.json` | `claude plugin install dioxus-ui@dioxuspress --scope user` |
| **`project`** | Everyone working in the repository (commit to share) | `.claude/settings.json` | `claude plugin install dioxus-ui@dioxuspress --scope project` |
| **`local`** | Only you, in the current repository | `.claude/settings.local.json` | `claude plugin install dioxus-ui@dioxuspress --scope local` |

Or interactively inside Claude Code:

```text
/plugin install dioxus-ui@dioxuspress
```

### Local development / testing from a checkout

```bash
# macOS / Linux
claude plugin marketplace add ./path/to/dioxus-ui
claude plugin install dioxus-ui@dioxuspress --scope local

# Windows (PowerShell)
claude plugin marketplace add .\path\to\dioxus-ui
claude plugin install dioxus-ui@dioxuspress --scope local
```

## Invocation

Once installed, Claude Code activates the `dioxus-ui` skill automatically whenever you ask to build screens, add components/blocks, configure `dxui` or Cargo features, or customize themes in a Dioxus project. You can also invoke it explicitly in a session:

```text
/dioxus-ui:dioxus-ui
```

Example prompts:

- "Inspect this project's `dioxus-shadcn` setup and build a settings page with a theme switcher and confirmation dialog."
- "Add the `login` block with `dxui` and wire `on_sign_in` to our auth handler without overwriting my existing button customizations."
- "Switch this app to the `nord` preset theme and keep strict CSP without `'unsafe-eval'`."

## Updating and Uninstalling

### Update

```bash
claude plugin update dioxus-ui@dioxuspress
```

Or interactively via `/plugin` → **Installed** → **Update now**.

### Uninstall

```bash
claude plugin uninstall dioxus-ui@dioxuspress
```

Optionally remove the marketplace registration as well:

```bash
claude plugin marketplace remove dioxuspress
```

## Plugin Contents

- `.claude-plugin/plugin.json` — Plugin manifest synced with the workspace crate version.
- `skills/dioxus-ui/SKILL.md` — End-to-end 5-step Dioxus UI implementation workflow.
- `skills/dioxus-ui/references/catalog.md` — Generated index of all 100 public components, categories, Cargo features, imports, and `dxui add` commands.
- `skills/dioxus-ui/references/blocks.md` — Generated index of all 11 full-screen blocks and their component dependencies.
- `skills/dioxus-ui/references/themes.md` — Generated list of all 33 WCAG-AA theme presets (`light` / `dark`), semantic tokens, and dark-mode setup.
- `skills/dioxus-ui/references/api-patterns.md` — Component API conventions, controlled/uncontrolled patterns, `merge_classes`, `DensityProvider`, and CSP rules.
