# RFC 0083: Desktop Workstation Ergonomics and Fixes

- Status: Accepted
- Created: 2026-11-26

## Summary

Address five concrete defects and ergonomics gaps reported while migrating
`phonon-studio` (a dense Dioxus 0.7 Desktop audio workstation) onto the
published `dioxus-shadcn` 0.7.0 crate and `dxui` 0.7.0 CLI (`FB-studio-01` to
`FB-studio-05`), and record two deferred observations (`FB-studio-06` and
`FB-studio-07`). The changes ship in `0.8.0` alongside the existing unreleased
`0.8.0` improvements, keeping all existing RSX call sites source-compatible.

## Current State

Observed against published `v0.7.0` (`dioxus-shadcn` 0.7.0 + `dxui` 0.7.0)
during the `phonon-studio` Desktop migration:

- **FB-studio-01, `UiDensity::Compact` stretches `ButtonSize::Xs`.**
  `button_class` maps `UiDensity::Compact` to `Some("min-h-8")` (32 px), which
  overrides `ButtonSize::Xs`'s `h-7` (28 px). Enabling `Compact` density makes
  extra-small buttons taller than under the default `Comfortable` density.
- **FB-studio-02, oversized `Button` loading spinner at `Xs` / `IconSm`.**
  `BUTTON_SPINNER_CLASS` hardcodes `me-2 inline-block size-4` (16 px) for all
  button sizes, which crowds `ButtonSize::Xs` (`h-7 text-xs`) and
  `ButtonSize::IconSm` (`h-8 w-8`).
- **FB-studio-03, `dxui init` creates an unused `src/components/ui/mod.rs` in
  crate-mode apps.** When an app already declares `dioxus-shadcn` in
  `Cargo.toml` (`crate_sources` is non-empty), `init_project` still creates
  `src/components/ui/mod.rs`, which a crate-mode app never imports.
- **FB-studio-04, `NativeSelect` lacks orthogonal size presets.** While 0.7.0
  added `Size` enums to `Input`, `Textarea`, `Checkbox`, `Switch`, `Badge`, and
  `Progress`, `NativeSelect` remains hardcoded to `h-10 px-3 py-2 text-sm`.
  Dense workstation forms had to repeat `h-8 px-2 py-1 text-xs` overrides across
  eight call sites.
- **FB-studio-05, `Slider` / `RangeSlider` lack a commit callback and wake all
  observers on unrelated DOM mutations.** Apps with undo stacks or expensive
  recalculation need a pointer-release / keyboard-commit callback
  (`on_value_commit`) distinct from continuous `on_value_change` drag updates.
  In addition, each slider's disconnect `MutationObserver` on
  `document.documentElement` inspects `root.isConnected` on every DOM childList
  mutation, even pure node insertions when `removedNodes` is empty.
- **FB-studio-06, `Badge` attribute forwarding.** Wrapping `Badge` in a `span`
  just for `title` is another instance of the deferred "Attributes on more
  components" entry in `TODOs.md` and should be decided globally rather than
  patched on one component.
- **FB-studio-07, `TooltipTrigger` + `Button` composition.** Passing
  `button_class(...)` to `TooltipTrigger` works without nested `<button>`
  elements; adding a wrapper component waits for broader demand.

## Decision

- **FB-studio-01.** Under `UiDensity::Compact`, `button_class` omits `min-h-8`
  when `size == ButtonSize::Xs`, letting `Xs` stay 28 px high (`h-7`). All other
  sizes keep `min-h-8`, and `Comfortable` and `Touch` are unchanged.
- **FB-studio-02.** `Button` renders a compact spinner (`size-3 me-1.5`) for
  `ButtonSize::Xs` and `ButtonSize::IconSm` (with no trailing margin on
  icon-only buttons when desired, or `me-1.5` consistent with `BUTTON_SPINNER_CLASS`),
  and keeps `size-4 me-2` for `Sm`, `Md`, `Lg`, `Icon`, and `IconLg`.
- **FB-studio-03.** `init_command` resolves `crate_sources(&root)` before
  creating directories; when `dioxus-shadcn` is already a Cargo dependency
  (`!sources.is_empty()`), `dxui init` writes `assets/dioxus-shadcn.css` and its
  `@source` line without creating `src/components/ui/mod.rs`. Source-copy apps
  (no `dioxus-shadcn` dependency) keep the existing behavior, and `dxui add`
  still creates `src/components/ui/mod.rs` on demand.
- **FB-studio-04.** Add `NativeSelectSize::{Sm, Md, Lg}` (default `Md`, matching
  `InputSize`: `Sm => "h-8 px-2.5 py-1 text-xs"`, `Md => "h-10 px-3 py-2 text-sm"`,
  `Lg => "h-12 px-4 py-2.5 text-base"`), `native_select_sized_class(size,
  invalid, class)`, and `#[props(default)] size: NativeSelectSize` on
  `NativeSelect`. Existing `native_select_class` and `Md` default rendering are
  unchanged.
- **FB-studio-05.** Add `#[props(default)] on_value_commit: Option<EventHandler<f64>>`
  to `Slider` and `#[props(default)] on_value_commit: Option<EventHandler<(f64, f64)>>`
  to `RangeSlider`. The pointer scripts emit `["change", ...]` during press/move
  and `["commit", ...]` on `pointerup` / `pointercancel` while pointer capture
  is held; keyboard moves call `on_value_commit` after `on_value_change` when
  the value changes. Both scripts short-circuit their `MutationObserver`
  callback when no mutation record removed nodes (`records.some((r) =>
  r.removedNodes.length > 0)`).
- **FB-studio-06 & FB-studio-07.** Deferred and documented; no public API added
  in this RFC.

Every crate change lands in its CLI source-copy template, docs page, and tests
in the same milestone.

## Validation

- Unit tests (each failing before its fix):
  - `Button`: `button_class(Primary, Xs, Compact, "")` contains `h-7` and does
    not contain `min-h-8`; SSR `Button { size: ButtonSize::Xs, loading: true }`
    renders `size-3`.
  - `NativeSelect`: `native_select_sized_class` for `Sm`/`Md`/`Lg` and SSR
    `NativeSelect { size: NativeSelectSize::Sm }` renders `h-8`.
  - `Slider` & `RangeSlider`: `slider_pointer_script` / `range_slider_pointer_script`
    unit & SSR coverage for `on_value_commit` and `removedNodes` guard.
  - `dxui` CLI: `init_command` inside a crate-mode manifest creates
    `assets/dioxus-shadcn.css` with `@source` and does not create
    `src/components/ui/mod.rs`; source-copy init still creates both.
- `template_parity`, `cargo test --workspace --all-features`,
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
  `npm run verify:docs`, `npm run verify:tailwind-conflicts`,
  `scripts/generated-fixture-smoke.sh`, and `npm run verify:semver` pass.
