# Badge

Badge is a compact status or category indicator with static visual variants.

## Source Copy

```bash
dxui add badge
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["badge"] }
```

## API Surface

- `Badge` (`variant`, `style`, `size`, `pill`, `class`)
- `BadgeVariant` (`Default`, `Secondary`, `Destructive`, `Outline`, `Success`, `Warning`, `Info`)
- `BadgeStyle` (`Solid`, `Soft`, `Outline`, `Dash`)
- `BadgeSize` (`Xs`, `Sm`, `Md`, `Lg`)
- `badge_class`
- `badge_styled_class`

`BadgeVariant` has `Default`, `Secondary`, `Destructive`, and `Outline`, and
the status variants `Success`, `Warning`, and `Info`, which draw
`text-success-foreground` and the like on solid surfaces
([RFC 0058](../rfcs/0058-status-variants.md)). `BadgeStyle::Soft`,
`BadgeStyle::Outline`, and `BadgeStyle::Dash` tint the background or border
while keeping `text-foreground` for WCAG AA contrast across all presets.

## Accessibility Notes

Use badges as supporting text, not as the only way to communicate important
state. Pair color-coded badges with clear wording.
