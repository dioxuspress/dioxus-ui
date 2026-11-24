# Alert

Alert presents important status or validation messages with title and
description parts.

## Source Copy

```bash
dxui add alert
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["alert"] }
```

## API Surface

- `Alert` (`variant`, `style`, `class`)
- `AlertTitle`
- `AlertDescription`
- `AlertVariant` (`Default`, `Destructive`, `Success`, `Warning`, `Info`)
- `AlertStyle` (`Default`, `Soft`, `Outline`, `Dash`, `Solid`)
- `alert_class`
- `alert_styled_class`
- `alert_title_class`
- `alert_description_class`

`AlertVariant` has `Default`, `Destructive`, and the status variants
`Success`, `Warning`, and `Info`. The status colors are too light to be text,
so status alerts tint the surface and border and keep foreground text
([RFC 0058](../rfcs/0058-status-variants.md)). `AlertStyle` selects between
the default card/tint look, `Soft`, `Outline`, `Dash`, and high-contrast
`Solid` surfaces; `AlertDescription` adapts automatically through `Alert`'s
context.

## Accessibility Notes

`Alert` always renders `role="alert"`, so use it for messages that should be
announced immediately; use a plain `div` for passive supporting copy.

`AlertTitle` renders a `div`, as in shadcn/ui v4, so it never skips a heading
level. Wrap its text in a heading of the right level when the page outline
needs one (see [RFC 0054](../rfcs/0054-automated-accessibility-audit.md)).
