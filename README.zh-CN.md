# dioxus-shadcn

[English](README.md) · **简体中文**

`dioxus-shadcn` 是面向 [Dioxus 0.7](https://dioxuslabs.com) 的 `shadcn/ui` 风格组件库：

- **无样式原语层（Headless Primitives）**：负责交互行为、无障碍访问（ARIA / 键盘导航）、状态管理与组合能力
- **Tailwind CSS 默认视觉层**：基于语义化设计令牌（Design Tokens）构建的开箱即用样式组件
- **源码拷贝 CLI（`dxui`）**：按需将组件源码复制到你的应用仓库中，支持自由深度定制
- **Cargo Crate 依赖模式**：偏好通过 Cargo 管理版本更新的项目也可直接按特性（feature）引入

当前共提供 **84 个组件**、`dxui` 命令行工具以及在线交互预览站 [`ui.dioxuspress.dev`](https://ui.dioxuspress.dev/)。最新版本 `0.6.7` 已发布至 crates.io：[`dioxus-shadcn`](https://crates.io/crates/dioxus-shadcn) 与 [`dioxus-shadcn-cli`](https://crates.io/crates/dioxus-shadcn-cli)。

## 产品定位与架构形态

`dioxus-shadcn` 并非单纯的 Tailwind 样式模板包，而是三层协同架构：

```text
Dioxus 无样式交互与无障碍原语层 (dioxus-shadcn-primitives)
+ Tailwind CSS v4 默认样式层 (dioxus-shadcn)
+ CLI 组件源码生成器 (dioxus-shadcn-cli / dxui)
```

开发者可任选以下两种工作流，且**两者的组件 API 完全一致**：

1. **源码拷贝模式（Source-Copy）**：将组件源码直接生成到业务项目中，完全掌控并按需修改每一行代码。
2. **Crate 依赖模式（Crate Mode）**：在 `Cargo.toml` 中按 feature 引入所需组件，跟随 Cargo 升级版本。

## 仓库目录结构

```text
dioxus-ui/
├─ crates/
│  ├─ dioxus-shadcn-core/        # 共享类型、Tailwind 类名合并、主题令牌与脚本桥接
│  ├─ dioxus-shadcn-primitives/  # 无样式交互与无障碍原语组件
│  ├─ dioxus-shadcn/             # 面向业务公开的 Tailwind 样式组件（84 个）
│  └─ dioxus-shadcn-cli/         # dxui 命令行工具（init / add / diff / list / theme）
│     ├─ registry/               # 内嵌于 CLI 二进制中的组件注册表元数据
│     └─ templates/              # dxui add 拷贝到用户项目的组件与辅助模板
├─ examples/
│  ├─ preview-states/            # Web / Desktop / Mobile 共享的组件状态预览夹具
│  ├─ web-demo/
│  ├─ desktop-demo/
│  ├─ mobile-demo/
│  ├─ runtime-web-verification/
│  ├─ runtime-desktop-verification/
│  └─ fullstack-hydration/       # 全栈 SSR + 水合一致性验证示例
├─ site/                         # 在线组件展示站源码（ui.dioxuspress.dev）
└─ docs/
   └─ rfcs/                      # 架构与交互规范 RFC（0001–0082）
```

发布在 crates.io 上的简要使用说明位于 [crates/README.md](crates/README.md)；本文件面向仓库整体架构、使用方式与研发质量门禁说明。示例运行方式见 [examples/README.md](examples/README.md)。

## 快速使用

### 方式一：源码拷贝模式（Source-Copy Mode）

```bash
cargo install dioxus-shadcn-cli

dxui init
dxui add button dialog input
```

- `dxui add` 默认保留本地已修改过的组件文件，并逐一汇报每个文件是「新写入」「未变化」还是「因本地改动而保留」；如需强制覆盖模板，可显式传入 `dxui add button --overwrite`。
- `dxui diff button` 会输出本地拷贝与当前 CLI 模板之间的 unified diff（存在差异或缺失时退出码为 `1`，可直接接入 CI 检查）；不带组件名运行 `dxui diff` 会自动扫描 `src/components/ui/mod.rs` 与 `src/blocks/mod.rs` 中声明的全部组件与区块。

在 Dioxus 项目中生成的目录结构示例：

```text
src/components/ui/mod.rs
src/components/ui/button.rs
src/components/ui/class_merge.rs
src/components/ui/class_merge_table.rs
src/components/ui/default_attribute.rs
src/components/ui/density.rs
src/components/ui/dialog.rs
src/components/ui/dialog_labels.rs
src/components/ui/element_id.rs
src/components/ui/input.rs
src/components/ui/modal_focus.rs
src/components/ui/overlay.rs
src/components/ui/overlay_root.rs
src/components/ui/root_state.rs
src/components/ui/script.rs
src/components/ui/utils.rs
assets/dioxus-shadcn.css
```

每个组件都会复用 `utils.rs`；像 `Dialog` 这类复合弹层组件还会按需拉取各自独立的辅助模块（详见 [RFC 0074](docs/rfcs/0074-helper-templates.md)）。涉及浏览器端 DOM 辅助脚本的组件（如 `Dialog`、各类菜单、`Tabs`、`Slider`）会一并拷贝 `script.rs`——它基于 `wasm-bindgen` 执行模块片段，**在 Web 端完全不依赖 `eval`**（见下方[内容安全策略 CSP](#内容安全策略-csp)），仅需在应用 `Cargo.toml` 中补充以下依赖（若缺失，`dxui add` 会自动提示）：

```toml
[dependencies]
serde = "1"

[target.'cfg(target_arch = "wasm32")'.dependencies]
serde_json = "1"
wasm-bindgen = "0.2"
```

#### Tailwind CSS v4 与主题令牌

在 Tailwind CSS v4 下，`assets/dioxus-shadcn.css` 是输入样式表（由应用在构建时扫描源码与组件后编译出最终 CSS）：

```css
@import "tailwindcss";

@custom-variant dark (&:is(.dark *));

:root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.141 0.005 285.823);
  --primary: oklch(0.21 0.006 285.885);
  --primary-foreground: oklch(0.985 0 0);
  /* ...其余语义化令牌 */
}

.dark {
  color-scheme: dark;
  --background: oklch(0.141 0.005 285.823);
  /* ... */
}

@theme inline {
  --color-background: var(--background);
  --color-primary: var(--primary);
  /* ... */
}
```

样式表完整遵循 shadcn/ui v4 语义化颜色令牌规范（[RFC 0051](docs/rfcs/0051-semantic-color-tokens.md)）：包含 `--background`、`--foreground`、`--card`、`--popover`、`--primary`、`--secondary`、`--muted`、`--accent`、`--destructive`（及其对应的 `-foreground` 前景色）、`--border`、`--input`、`--ring`、`--chart-1` 至 `--chart-5`、`--sidebar-*` 分组、`--radius`，并额外扩展了 `--destructive-foreground` 以及 `--success` / `--warning` / `--info` 状态色。定制品牌色时，只需重定义 `:root` 与 `.dark` 中的对应变量即可。

启用深色模式只需在应用顶层容器添加 `dark` 类名：

```rust
div { class: "dark min-h-screen bg-background text-foreground", App {} }
```

此外，`dxui theme list` 内置了 **33 套预设主题**（[RFC 0057](docs/rfcs/0057-theme-presets.md)），可通过 `dxui theme add <name>...` 追加到样式表中，并在任意子树上通过 `data-theme` 切换：

```rust
div { "data-theme": "nord", class: "min-h-screen bg-background text-foreground", App {} }
```

### 方式二：Crate 依赖模式（Crate Mode）

```toml
[dependencies]
dioxus-shadcn = { version = "0.6", default-features = false, features = ["button", "input", "dialog"] }
```

```rust
use dioxus_shadcn::{Button, DialogContent, DialogTitle, Input};
```

在 Crate 模式下同样运行一次 `dxui init` 即可生成 `assets/dioxus-shadcn.css`，并且它会自动通过 `cargo metadata` 写入指向 `dioxus-shadcn` 源码目录的 `@source` 指令，确保 Tailwind CSS v4 能扫描到组件内部使用的原子类（升级 crate 版本后重跑一次 `dxui init` 即可，详见 [crates/README.md](crates/README.md#depend-on-the-crate)）。

## 版本与兼容性策略

- 最低支持的 Rust 编译器版本（MSRV）：**Rust 1.88**（`rust-version`）。
- `1.0` 之前，次版本（`0.x`）允许调整公开 API，并会在 `CHANGELOG.md` 中附带迁移说明；自 `1.0` 起，`1.x` 将稳定保持 crate API、源码模板与 `dxui` 命令兼容，跨 Dioxus 大版本升级将对应新的主版本号。详见 [版本兼容性说明](docs/release.md#compatibility)。

## 内容安全策略 CSP

在 Web 平台上，所有组件的页面辅助脚本均通过 `wasm-bindgen` 同源模块片段运行，**绝不调用字符串 `eval`**（[RFC 0080](docs/rfcs/0080-page-scripts-without-eval.md)），因此可直接在严格的无 `'unsafe-eval'` 策略下运行：

```text
script-src 'self' 'wasm-unsafe-eval'
```

每次发版均通过 `npm run verify:csp` 在真实浏览器中验证该策略（像 [DioxusPress](https://github.com/dioxuspress/dioxuspress) 全栈生产环境即直接运行在无 `'unsafe-eval'` 的 CSP 基线下）。应用层额外注意事项：

- `theme_init_script` 返回首屏防闪烁内联脚本，可通过哈希（hash）或 `nonce` 放行。
- Dioxus 0.7 内置的 `document::eval` 与 `document::Title` 底层走 `eval`，若启用严格 CSP，建议通过 `Dioxus.toml` 的 `[web.app] title` 或服务端 `<head>` 渲染标题。

## 安全基线

组件库对外部输入默认执行以下安全防护：

- **零 `dangerous_inner_html`**：所有文本与子元素均经由 Dioxus 转义渲染；富文本 HTML 若由业务方自行渲染，由业务方负责白名单清洗。
- **URL 协议白名单**：接收链接地址的组件属性（`Breadcrumb`、`Dock`、`HoverCard`、`Menu`、`NavigationMenu`、`Pagination`、`Sidebar` 等）仅放行相对路径、锚点以及 `http:`、`https:`、`mailto:`、`tel:`，自动拦截 `javascript:` 等危险协议。
- **主题名校验与转义**：主题控制器与 `theme_init_script` 仅接受合法主题标识符（字母、数字、`-`、`_`），并对 `localStorage` 键名做安全转义。
- **CLI 路径边界保护**：`dxui` 仅允许在当前应用根目录下写入文件，并拒绝跨越符号链接（symlink）写入。

## 组件范围与 Tailwind 静态类名准则

完整的 **84 个公开组件目录**见 [docs/components/catalog.md](docs/components/catalog.md)（也可通过 `dxui list` 查看）。

编写或扩展组件样式时，Tailwind 类名必须以**完整的静态字面量**出现，严禁运行时动态拼接字符串：

```rust
// 推荐：静态完整类名，Tailwind 扫描器可精确提取
match variant {
  ButtonVariant::Primary => "bg-primary text-primary-foreground hover:bg-primary/90",
  ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
}

// 禁止：动态拼接会导致样式丢失（npm run verify:tailwind-static 会直接拦截）
format!("bg-{}-500", color)
```

## 在线组件预览站（Component Site）

`site/` 是一个完整的 Dioxus Web 应用（在线地址：[`https://ui.dioxuspress.dev/`](https://ui.dioxuspress.dev/)），按分类展示全部组件的实时交互预览（Preview）、源码标签页（Code）、安装命令、33 套主题预设切换与明暗模式对比（[RFC 0052](docs/rfcs/0052-component-site.md)）。本地启动预览站：

```bash
dx serve --package dioxus-ui-site
```

- 修改站点使用的样式类后运行：`npm run css:site`
- 修改组件注册表、目录或主题预设后运行：`npm run site:catalog`
- 每次推送到 `main` 分支都会通过 `.github/workflows/pages.yml` 自动构建并部署至 <https://ui.dioxuspress.dev/>。

## 本地验证与研发门禁

安装 Node 校验依赖：

```bash
npm install
```

常用验证命令速查（完整门禁清单见英文版 [README.md](README.md#verification-shortcuts) 与 [docs/quality-gates.md](docs/quality-gates.md)）：

```bash
# 默认本地快速门禁（预览烟雾测试 + 文档与目录元数据一致性校验）
npm run verify

# 仅运行文档、目录、RFC 与链接锚点校验
npm run verify:docs

# 发版前完整本地门禁（含全工作区编译测试、特性矩阵、源码拷贝夹具、CSP 与 SemVer 检查）
npm run verify:release
```

## 核心文档索引

- [架构设计总览](docs/design.md)
- [演进路线图](docs/roadmap.md)
- [Cargo 工作区规范](docs/workspace.md)
- [组件 API 设计规范](docs/component-api.md)
- [发版与打包策略](docs/release.md)
- [质量门禁说明](docs/quality-gates.md)
- [组件完整目录](docs/components/README.md)
- [组件预览站设计](docs/site.md)
- [架构与交互 RFC 列表（0001–0082）](README.md#documentation)

## 开源协议

[MIT](LICENSE)
