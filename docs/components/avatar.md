# Avatar

Avatar displays a user or entity image with a fallback part for initials or
short labels.

## Source Copy

```bash
dxui add avatar
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["avatar"] }
```

## API Surface

- `Avatar` (`size`, `shape`, `class`)
- `AvatarImage`
- `AvatarFallback`
- `AvatarGroup`
- `AvatarSize` (`Xs`, `Sm`, `Md`, `Lg`, `Xl`)
- `AvatarShape` (`Circle`, `Square`)
- `avatar_class`
- `avatar_styled_class`
- `avatar_image_class`
- `avatar_fallback_class`
- `avatar_group_class`

## Accessibility Notes

Use meaningful `alt` text when the avatar communicates identity. Use an empty
`alt` value when the surrounding text already names the person or object.
