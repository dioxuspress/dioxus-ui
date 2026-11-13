## API 概览

- `ThemeController`
- `Theme`
- `theme_init_script`
- `THEME_STORAGE_KEY`

```rust
let mut theme = use_signal(|| Theme::System);

rsx! {
  ThemeController {
    theme: theme(),
    on_theme_change: move |stored| theme.set(stored),
  }
  ToggleGroup {
    "aria-label": "Theme",
    value: theme().as_str().to_string(),
    // Pressing the current theme again asks to clear it; a theme stays set.
    on_value_change: move |value: String| {
      if !value.is_empty() {
        theme.set(Theme::parse(&value));
      }
    },
    for (option, label) in [(Theme::System, "System"), (Theme::Light, "Light"), (Theme::Dark, "Dark")] {
      ToggleGroupItem {
        key: "{label}",
        value: option.as_str().to_string(),
        "{label}"
      }
    }
  }
}
```

## 行为与交互

参见 [RFC 0071](../../../../docs/rfcs/0071-theme-controller.md)。

- `Theme::Light` 和 `Theme::Dark` 会在根元素上移除或添加显式的 `dark` 类名（参见 [RFC 0047](../../../../docs/rfcs/0047-opt-in-dark-theme.md)）。`Theme::System` 会跟随系统的 `prefers-color-scheme` 媒体查询并随之动态切换。
- `Theme::Preset(name)` 会将根元素的 `data-theme` 设置为通过 `dxui theme add` 添加的主题预设（参见 [RFC 0057](../../../../docs/rfcs/0057-theme-presets.md)）；由于主题预设自带配色方案，此时会移除 `dark` 类名。
- 应用传入的每次主题变更都会以 `storage_key`（默认为 `THEME_STORAGE_KEY`，即 `dxui-theme`；传空字符串则不持久化）保存在 `localStorage` 中。当控制器挂载时，若已保存的主题与当前 `theme` 不同，则会立即应用已保存的主题并通过 `on_theme_change` 回调通知应用同步状态。若无已保存的主题，则默认为 `System`。
- `theme` 保持受控：控制器仅应用外部传入的值，除挂载时上报已存储的主题外不会主动修改它。
- 控制器会渲染一个隐藏的 `span`；请保持其处于挂载状态（通常放置在应用的根组件中）。

### 首屏渲染前防闪烁

由于控制器在应用首次渲染之后才会执行，已保存的深色或预设主题在加载瞬间可能会先闪烁为浅色。请将 `theme_init_script(THEME_STORAGE_KEY)` 的输出放入页面 `<head>` 内的 `<script>` 标签中，以便在应用加载前提前应用主题。在 Dioxus Web 应用中，可将其加入 `index.html` 模板；Desktop 和 Mobile 应用则可将其传入 WebView 的自定义 head 配置中。若启用了内容安全策略，请通过哈希值或 nonce 放行该内联脚本（参见[内容安全策略（CSP）](../../../../README.md#content-security-policy)）。

## 无障碍说明

控制器自身不渲染可见内容。切换主题的交互控件由应用自行构建；请为其提供无障碍名称（例如标记为 `"Theme"` 的 `ToggleGroup`），并明确展示当前选中的模式，因为 `System` 在视觉上可能与浅色或深色任一方案相同。
