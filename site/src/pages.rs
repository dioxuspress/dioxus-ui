use dioxus::prelude::*;
use dioxus_shadcn::{
  Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Tabs, TabsContent, TabsList, TabsTrigger,
};

use crate::Route;
use crate::blocks::BLOCKS;
use crate::catalog::{CATEGORIES, Category, Component};
use crate::examples::EXAMPLES;
use crate::i18n::{self, SiteLang, use_site_lang};

const H1: &str = "text-3xl font-bold tracking-normal";
const H2: &str = "mt-10 border-b border-border pb-2 text-xl font-semibold";
const LEAD: &str = "mt-2 text-lg text-muted-foreground";
const P: &str = "mt-4 leading-7";
const CODE_BLOCK: &str =
  "overflow-x-auto rounded-md border border-border bg-muted p-4 pe-20 font-mono text-sm";
const INLINE_CODE: &str = "rounded bg-muted px-1.5 py-0.5 font-mono text-sm";
const DOCS_URL: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/blob/main/docs/components");

fn find_component(slug: &str) -> Option<(&'static Category, &'static Component)> {
  CATEGORIES.iter().find_map(|category| {
    category
      .components
      .iter()
      .find(|component| component.slug == slug)
      .map(|component| (category, component))
  })
}

// `REFERENCES_EN` / `REFERENCES_ZH` and `BLOCK_REFERENCES_EN` /
// `BLOCK_REFERENCES_ZH`, rendered from Markdown by `build.rs`.
include!(concat!(env!("OUT_DIR"), "/references.rs"));

/// Styles the rendered docs, which arrive as plain HTML elements.
const PROSE: &str = "[&_h2]:mt-10 [&_h2]:border-b [&_h2]:border-border [&_h2]:pb-2 [&_h2]:text-xl [&_h2]:font-semibold [&_h3]:mt-8 [&_h3]:text-lg [&_h3]:font-semibold [&_p]:mt-4 [&_p]:leading-7 [&_ul]:mt-4 [&_ul]:list-disc [&_ul]:ps-6 [&_ol]:mt-4 [&_ol]:list-decimal [&_ol]:ps-6 [&_li]:mt-1.5 [&_li]:leading-7 [&_a]:font-medium [&_a]:underline [&_a]:underline-offset-4 [&_pre]:mt-4 [&_pre]:overflow-x-auto [&_pre]:rounded-md [&_pre]:border [&_pre]:border-border [&_pre]:bg-muted [&_pre]:p-4 [&_pre]:text-sm [&_code]:font-mono [&_code]:text-sm [&_:not(pre)>code]:rounded [&_:not(pre)>code]:bg-muted [&_:not(pre)>code]:px-1.5 [&_:not(pre)>code]:py-0.5 [&_:not(pre)>code]:[overflow-wrap:anywhere] [&_table]:mt-4 [&_table]:w-full [&_table]:text-sm [&_th]:border-b [&_th]:border-border [&_th]:py-2 [&_th]:text-start [&_td]:border-b [&_td]:border-border [&_td]:py-2";

fn with_site_base(html: &str) -> String {
  let base = dioxus::cli_config::base_path().unwrap_or_default();
  let base = base.trim_matches('/');
  let prefix = if base.is_empty() { String::new() } else { format!("/{base}") };
  html.replace("__SITE_BASE__", &prefix)
}

/// A component's rendered reference in `lang`, with links to other component
/// pages rewritten under the site's base path.
pub(crate) fn reference_html(lang: SiteLang, slug: &str) -> Option<String> {
  let table = match lang {
    SiteLang::En => REFERENCES_EN,
    SiteLang::Zh => REFERENCES_ZH,
  };
  let (_, html) = table.iter().find(|(reference, _)| *reference == slug)?;
  Some(with_site_base(html))
}

/// A block's rendered reference in `lang`, with component links rewritten under
/// the site's base path.
pub(crate) fn block_reference_html(lang: SiteLang, slug: &str) -> Option<String> {
  let table = match lang {
    SiteLang::En => BLOCK_REFERENCES_EN,
    SiteLang::Zh => BLOCK_REFERENCES_ZH,
  };
  let (_, html) = table.iter().find(|(reference, _)| *reference == slug)?;
  Some(with_site_base(html))
}

/// The crate version these pages document, such as `0.3.0`.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The version requirement for `Cargo.toml`, such as `0.3`.
fn version_requirement() -> &'static str {
  VERSION.rsplit_once('.').map_or(VERSION, |(requirement, _)| requirement)
}

// Copies the text it receives, reports whether that worked, and reports
// again two seconds later so the button can show its label again.
const COPY_SCRIPT: &str = r#"
const text = await dioxus.recv();
try {
  await navigator.clipboard.writeText(text);
  dioxus.send(true);
} catch {
  dioxus.send(false);
  return;
}
await new Promise((resolve) => setTimeout(resolve, 2000));
dioxus.send(false);
"#;

/// A button over a code block's corner that copies `text`; it reads
/// "Copied" for two seconds, also announced through a status message.
#[component]
fn CopyButton(text: String, label: String) -> Element {
  let mut copied = use_signal(|| false);
  let zh = use_site_lang()() == SiteLang::Zh;

  rsx! {
    Button {
      class: "absolute end-2 top-2",
      variant: ButtonVariant::Outline,
      size: ButtonSize::Sm,
      "aria-label": "{label}",
      "data-site-copy": "",
      onclick: move |_| {
        let text = text.clone();
        spawn(async move {
          let mut eval = document::eval(COPY_SCRIPT);
          // A send error means the page already finished the script.
          let _ = eval.send(text);
          while let Ok(state) = eval.recv::<bool>().await {
            copied.set(state);
          }
        });
      },
      if copied() {
        if zh { "已复制" } else { "Copied" }
      } else {
        if zh { "复制" } else { "Copy" }
      }
    }
    span { class: "sr-only", role: "status",
      if copied() {
        if zh { "已复制到剪贴板" } else { "Copied to the clipboard" }
      }
    }
  }
}

#[component]
fn CodeBlock(code: String) -> Element {
  let zh = use_site_lang()() == SiteLang::Zh;

  rsx! {
    div { class: "relative mt-4",
      pre { class: CODE_BLOCK, code { "{code}" } }
      CopyButton {
        text: code.clone(),
        label: if zh { "复制代码".to_string() } else { "Copy code".to_string() },
      }
    }
  }
}

#[component]
pub fn Home() -> Element {
  let lang = use_site_lang()();
  let zh = lang == SiteLang::Zh;
  let count: usize = CATEGORIES.iter().map(|category| category.components.len()).sum();

  rsx! {
    article { "data-site-page": "home",
      h1 { class: H1, "dioxus-shadcn" }
      p { class: LEAD,
        if zh {
          "面向 Dioxus 0.7 的无障碍、Tailwind CSS v4 语义化组件库，设计理念源自 shadcn/ui。"
          "既可通过 dxui CLI 将组件源码拷贝到你的项目中自由定制，也可作为标准 Cargo crate 按需开启特性依赖。"
        } else {
          "Accessible, Tailwind-styled components for Dioxus, in the manner of shadcn/ui. "
          "Copy a component's source into your app with the CLI, or depend on the crate."
        }
      }
      div { class: "mt-6 flex flex-wrap gap-3",
        Link {
          class: "inline-flex h-10 items-center rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground hover:bg-primary/90",
          to: Route::GettingStarted {},
          if zh { "快速开始" } else { "Get started" }
        }
        Link {
          class: "inline-flex h-10 items-center rounded-md border border-input bg-background px-4 text-sm font-medium hover:bg-accent",
          to: Route::ComponentPage { slug: "button".to_string() },
          if zh { "浏览组件" } else { "Browse components" }
        }
      }
      h2 { class: H2, if zh { "{count} 个组件" } else { "{count} components" } }
      div { class: "mt-6 grid gap-4 sm:grid-cols-2",
        for category in CATEGORIES {
          section {
            key: "{category.id}",
            class: "rounded-md border border-border bg-card p-4 text-card-foreground",
            h3 { class: "font-semibold", "{i18n::category_label(lang, category.id, category.label)}" }
            ul { class: "mt-2 flex flex-wrap gap-x-3 gap-y-1 text-sm",
              for component in category.components {
                li { key: "{component.slug}",
                  Link {
                    class: "text-muted-foreground hover:text-foreground hover:underline",
                    to: Route::ComponentPage { slug: component.slug.to_string() },
                    "{i18n::component_title(lang, component.slug, component.title)}"
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}

#[component]
pub fn GettingStarted() -> Element {
  let zh = use_site_lang()() == SiteLang::Zh;

  rsx! {
    article { "data-site-page": "getting-started",
      h1 { class: H1, if zh { "安装指南" } else { "Installation" } }
      p { class: LEAD,
        if zh {
          "支持「源码拷贝模式」与「Crate 依赖模式」两种接入方式。"
        } else {
          "Use the components as copied source or as a crate dependency."
        }
      }
      h2 { class: H2, if zh { "源码拷贝模式（Source-copy）" } else { "Source-copy mode" } }
      if zh {
        p { class: P,
          "使用 CLI 初始化 Tailwind CSS 输入样式表，并将组件源码拷贝到项目的 "
          code { class: INLINE_CODE, "src/components/ui" }
          " 目录，由你完全拥有并自由修改。"
        }
      } else {
        p { class: P,
          "The CLI writes the Tailwind input stylesheet and copies component source into "
          code { class: INLINE_CODE, "src/components/ui" }
          ", where you own and edit it."
        }
      }
      CodeBlock { code: "cargo install dioxus-shadcn-cli\n\ndxui init\ndxui add button\ndxui add dialog" }
      if zh {
        p { class: P,
          code { class: INLINE_CODE, "dxui add" }
          " 默认保留已存在的本地修改；如需覆盖已生成的组件文件，请传入 "
          code { class: INLINE_CODE, "--overwrite" }
          "。"
        }
      } else {
        p { class: P,
          code { class: INLINE_CODE, "dxui add" }
          " keeps existing files; pass "
          code { class: INLINE_CODE, "--overwrite" }
          " to replace a previously generated component."
        }
      }
      h2 { class: H2, if zh { "Crate 依赖模式" } else { "Crate mode" } }
      p { class: P,
        if zh {
          "按需开启对应组件的 Cargo feature，未开启的组件不会进入最终编译产物："
        } else {
          "Enable one feature per component:"
        }
      }
      CodeBlock {
        code: format!(
          "[dependencies]\ndioxus-shadcn = {{ version = \"{}\", default-features = false, features = [\"button\", \"dialog\"] }}",
          version_requirement()
        )
      }
      CodeBlock { code: "use dioxus_shadcn::{Button, ButtonVariant};".to_string() }
      h2 { class: H2, if zh { "样式表与令牌扫描" } else { "Stylesheet" } }
      if zh {
        p { class: P,
          code { class: INLINE_CODE, "dxui init" }
          " 会生成包含全部语义化颜色令牌的 Tailwind CSS v4 输入文件 "
          code { class: INLINE_CODE, "assets/dioxus-shadcn.css" }
          "。Crate 依赖模式同样需要该令牌表：可直接运行 "
          code { class: INLINE_CODE, "dxui init" }
          " 或将令牌块复制到你的输入样式表中交由构建管线编译。"
        }
        p { class: P,
          "在 Crate 模式下，由于 Tailwind 仅生成扫描源码中出现过的工具类，需在导入语句后添加指向 crate 源码目录的 "
          code { class: INLINE_CODE, "@source" }
          " 指令。已依赖该 crate 的项目中运行 "
          code { class: INLINE_CODE, "dxui init" }
          " 会自动通过 "
          code { class: INLINE_CODE, "cargo metadata" }
          " 写入对应版本路径；升级 crate 版本后重新运行一次 "
          code { class: INLINE_CODE, "dxui init" }
          " 即可更新："
        }
      } else {
        p { class: P,
          code { class: INLINE_CODE, "dxui init" }
          " writes "
          code { class: INLINE_CODE, "assets/dioxus-shadcn.css" }
          ", a Tailwind CSS v4 input with the semantic color tokens. Crate-mode apps need it too: run "
          code { class: INLINE_CODE, "dxui init" }
          " or copy the token blocks into your own input, and let your build compile it."
        }
        p { class: P,
          "Crate mode also needs an "
          code { class: INLINE_CODE, "@source" }
          " line for the crate's source after the import, since Tailwind generates only the classes it finds in scanned files. In an app that depends on the crate, "
          code { class: INLINE_CODE, "dxui init" }
          " writes it from "
          code { class: INLINE_CODE, "cargo metadata" }
          ". The path names one version, so run "
          code { class: INLINE_CODE, "dxui init" }
          " again after upgrading the crate:"
        }
      }
      CodeBlock { code: format!("@import \"tailwindcss\";\n@source \"/path/to/dioxus-shadcn-{VERSION}/src\";") }
      if zh {
        p { class: P,
          "接下来可继续阅读 "
          Link { class: "font-medium underline underline-offset-4", to: Route::Theming {}, "主题定制" }
          "。"
        }
      } else {
        p { class: P,
          "Continue with "
          Link { class: "font-medium underline underline-offset-4", to: Route::Theming {}, "Theming" }
          "."
        }
      }
    }
  }
}

#[component]
pub fn Theming() -> Element {
  let zh = use_site_lang()() == SiteLang::Zh;

  rsx! {
    article { "data-site-page": "theming",
      h1 { class: H1, if zh { "主题定制" } else { "Theming" } }
      p { class: LEAD,
        if zh {
          "所有组件均通过 shadcn/ui 语义化颜色令牌着色，重定义令牌即可一键统一切换全站视觉风格。"
        } else {
          "Components color through the shadcn/ui semantic tokens, so redefining a token restyles every component."
        }
      }
      h2 { class: H2, if zh { "语义化颜色令牌" } else { "Tokens" } }
      if zh {
        p { class: P,
          "生成的样式表在 "
          code { class: INLINE_CODE, ":root" }
          " 与 "
          code { class: INLINE_CODE, ".dark" }
          " 中定义全部语义化变量，并通过 "
          code { class: INLINE_CODE, "@theme inline" }
          " 映射为 Tailwind 颜色工具类，因此 "
          code { class: INLINE_CODE, "bg-primary" }
          " 与 "
          code { class: INLINE_CODE, "text-muted-foreground" }
          " 在你的业务标记中也同样开箱即用。"
        }
      } else {
        p { class: P,
          "The generated stylesheet defines the tokens in "
          code { class: INLINE_CODE, ":root" }
          " and "
          code { class: INLINE_CODE, ".dark" }
          ", and "
          code { class: INLINE_CODE, "@theme inline" }
          " maps each one to a Tailwind color, so "
          code { class: INLINE_CODE, "bg-primary" }
          " and "
          code { class: INLINE_CODE, "text-muted-foreground" }
          " work in your own markup too."
        }
      }
      ul { class: "mt-4 flex flex-wrap gap-2",
        for token in [
          "background", "foreground", "card", "popover", "primary", "secondary", "muted",
          "accent", "destructive", "success", "warning", "info", "border", "input", "ring",
          "chart-1", "sidebar", "radius",
        ] {
          li { key: "{token}", Badge { variant: BadgeVariant::Outline, "--{token}" } }
        }
      }
      h2 { class: H2, if zh { "品牌色定制" } else { "Rebrand" } }
      p { class: P,
        if zh {
          "在浅色与深色主题中覆盖你需要定制的品牌令牌，例如切换为蓝色品牌主色："
        } else {
          "Set the tokens you want to change in both themes, for example a blue brand:"
        }
      }
      CodeBlock {
        code: ":root {\n  --primary: oklch(0.546 0.245 262.881);\n  --primary-foreground: oklch(0.985 0 0);\n  --ring: oklch(0.546 0.245 262.881);\n}\n\n.dark {\n  --primary: oklch(0.623 0.214 259.815);\n  --ring: oklch(0.623 0.214 259.815);\n}"
          .to_string()
      }
      h2 { class: H2, if zh { "33 套预设主题" } else { "Theme presets" } }
      if zh {
        p { class: P,
          "CLI 内置移植自 daisyUI 的 33 套预设主题，且每一组文本对比度均通过 WCAG AA 无障碍校验。运行 "
          code { class: INLINE_CODE, "dxui theme add" }
          " 即可将所选预设追加到样式表；每套预设对应一条 "
          code { class: INLINE_CODE, "[data-theme]" }
          " 规则，可在任意子树或根节点上生效。"
          code { class: INLINE_CODE, "ThemeController" }
          " 默认跟随系统浅色/深色偏好并自动持久化记忆用户选择，本站顶栏的主题选择器即基于它实现。"
        }
      } else {
        p { class: P,
          "The CLI ships 33 presets ported from daisyUI, with every text color checked against WCAG AA. "
          code { class: INLINE_CODE, "dxui theme add" }
          " appends them to the stylesheet; each is one "
          code { class: INLINE_CODE, "[data-theme]" }
          " rule, so setting the attribute themes that element's subtree. "
          code { class: INLINE_CODE, "ThemeController" }
          " sets it on the page root, follows the system's light or dark scheme by default, and remembers the choice; this site's header menu uses it."
        }
      }
      CodeBlock {
        code: "dxui theme add nord dracula\n\n// main.rs\nlet mut theme = use_signal(|| Theme::System);\nrsx! {\n  ThemeController { theme: theme(), on_theme_change: move |stored| theme.set(stored) }\n  // theme.set(Theme::Preset(\"nord\".into())) from any control\n  App {}\n}"
          .to_string()
      }
      ul { class: "mt-4 grid gap-3 sm:grid-cols-2 lg:grid-cols-3", "data-site-presets": "",
        for theme in crate::themes::THEMES {
          li {
            key: "{theme.name}",
            "data-theme": theme.name,
            class: "rounded-lg border border-border bg-background p-4 text-foreground",
            div { class: "flex items-baseline justify-between gap-2",
              h3 { class: "font-semibold", "{theme.title}" }
              span { class: "text-xs text-muted-foreground",
                if theme.dark {
                  if zh { "深色" } else { "dark" }
                } else {
                  if zh { "浅色" } else { "light" }
                }
              }
            }
            div { class: "mt-3 flex gap-1", "aria-hidden": "true",
              for swatch in ["bg-primary", "bg-secondary", "bg-chart-3", "bg-muted", "bg-destructive", "bg-success", "bg-warning", "bg-info"] {
                span { key: "{swatch}", class: "h-6 flex-1 rounded-sm border border-border {swatch}" }
              }
            }
            div { class: "mt-3 flex items-center gap-2",
              span { class: "rounded-md bg-primary px-2 py-1 text-xs font-medium text-primary-foreground",
                if zh { "主色" } else { "Primary" }
              }
              span { class: "rounded-md bg-secondary px-2 py-1 text-xs font-medium text-secondary-foreground",
                if zh { "次色" } else { "Secondary" }
              }
            }
            code { class: "mt-3 block text-xs text-muted-foreground", "dxui theme add {theme.name}" }
          }
        }
      }
      h2 { class: H2, if zh { "深色模式" } else { "Dark theme" } }
      if zh {
        p { class: P,
          "深色模式采用显式开启设计：在根节点或祖先容器上添加 "
          code { class: INLINE_CODE, "dark" }
          " 类名即可切换全部语义令牌，同时不会篡改你显式编写的调色板工具类："
        }
      } else {
        p { class: P,
          "The dark theme is opt-in. Add the "
          code { class: INLINE_CODE, "dark" }
          " class to an ancestor, usually the app root. It redefines only the tokens, so your own palette classes keep their colors. The toggle in this site's header does the same."
        }
      }
      CodeBlock {
        code: "div { class: \"dark min-h-screen bg-background text-foreground\", App {} }".to_string()
      }
    }
  }
}

#[component]
pub fn ComponentPage(slug: String) -> Element {
  let lang = use_site_lang()();
  let zh = lang == SiteLang::Zh;
  let Some((category, component)) = find_component(&slug) else {
    return rsx! { NotFoundContent { path: format!("/components/{slug}") } };
  };

  let has_examples = EXAMPLES.iter().any(|example| example.slug == component.slug);

  rsx! {
    article { "data-site-page": "component", "data-component": component.slug,
      p { class: "text-sm text-muted-foreground",
        "{i18n::category_label(lang, category.id, category.label)}"
      }
      h1 { class: "{H1} mt-1", "{i18n::component_title(lang, component.slug, component.title)}" }
      p { class: LEAD, "{i18n::component_description(lang, component.slug, component.description)}" }
      h2 { class: H2, if zh { "安装方式" } else { "Installation" } }
      p { class: P, if zh { "拷贝源码到你的应用：" } else { "Copy the source into your app:" } }
      CodeBlock { code: "dxui add {component.slug}" }
      p { class: P, if zh { "或开启对应的 Crate 特性：" } else { "Or enable the crate feature:" } }
      CodeBlock {
        code: format!(
          "dioxus-shadcn = {{ version = \"{}\", features = [\"{}\"] }}",
          version_requirement(),
          component.feature
        )
      }
      if has_examples {
        h2 { class: H2, if zh { "交互示例" } else { "Examples" } }
        for (index, example) in EXAMPLES.iter().enumerate().filter(|(_, example)| example.slug == component.slug) {
          ExampleCard { key: "{component.slug}-{example.title}", index }
        }
      }
      if let Some(reference) = reference_html(lang, component.slug) {
        div { class: PROSE, "data-site-reference": "", dangerous_inner_html: reference }
      }
      p { class: "mt-10 text-sm text-muted-foreground",
        a {
          class: "font-medium underline underline-offset-4",
          href: "{DOCS_URL}/{component.slug}.md",
          if zh { "在 GitHub 上查看此组件源码文档" } else { "View this page on GitHub" }
        }
      }
    }
  }
}

/// The blocks (RFC 0073), each with its add command.
#[component]
pub fn Blocks() -> Element {
  let lang = use_site_lang()();
  let zh = lang == SiteLang::Zh;

  rsx! {
    article { "data-site-page": "blocks",
      h1 { class: H1, if zh { "页面区块（Blocks）" } else { "Blocks" } }
      p { class: LEAD,
        if zh {
          "由基础组件搭建的完整业务界面，执行命令即可连同依赖组件一并拷贝到你的应用中。"
        } else {
          "Whole screens built from the components, copied into your app with their components."
        }
      }
      CodeBlock { code: "dxui list blocks\ndxui add dashboard" }
      ul { class: "mt-8 grid gap-4 sm:grid-cols-2",
        for block in BLOCKS {
          li { key: "{block.slug}", class: "rounded-lg border border-border p-4",
            Link {
              class: "font-semibold underline-offset-4 hover:underline",
              to: Route::BlockPage { slug: block.slug.to_string() },
              "{i18n::block_title(lang, block.slug, block.title)}"
            }
            p { class: "mt-1 text-sm text-muted-foreground",
              "{i18n::block_description(lang, block.slug, block.description)}"
            }
            code { class: "{INLINE_CODE} mt-3 inline-block", "dxui add {block.slug}" }
          }
        }
      }
    }
  }
}

/// One block: its add command, a full-width preview or its source, and its
/// docs page.
#[component]
pub fn BlockPage(slug: String) -> Element {
  let lang = use_site_lang()();
  let zh = lang == SiteLang::Zh;
  let Some(block) = BLOCKS.iter().find(|block| block.slug == slug) else {
    return rsx! { NotFoundContent { path: format!("/blocks/{slug}") } };
  };
  let reference = block_reference_html(lang, block.slug);
  let display_title = i18n::block_title(lang, block.slug, block.title);

  rsx! {
    article { "data-site-page": "block", "data-block": block.slug,
      p { class: "text-sm text-muted-foreground", if zh { "页面区块" } else { "Blocks" } }
      h1 { class: "{H1} mt-1", "{display_title}" }
      p { class: LEAD, "{i18n::block_description(lang, block.slug, block.description)}" }
      CodeBlock { code: format!("dxui add {}", block.slug) }
      Tabs { class: "mt-8", default_value: "preview",
        TabsList {
          "aria-label": if zh {
            format!("{display_title} 区块切换")
          } else {
            format!("{} block", block.title)
          },
          TabsTrigger { value: "preview", if zh { "实时预览" } else { "Preview" } }
          TabsTrigger { value: "code", if zh { "源码" } else { "Code" } }
        }
        TabsContent { value: "preview",
          // The transform makes the frame the containing block of the
          // block's fixed parts, such as the off-canvas sidebar.
          div {
            class: "h-[44rem] overflow-auto rounded-lg border border-border [transform:translateZ(0)]",
            tabindex: "0",
            "aria-label": if zh {
              format!("{display_title} 预览")
            } else {
              format!("{} preview", block.title)
            },
            "data-site-block-preview": "",
            BlockRender { slug: block.slug }
          }
        }
        TabsContent { value: "code", class: "relative",
          pre { class: "{CODE_BLOCK} max-h-[44rem]", tabindex: "0", "data-site-block-source": "",
            code { "{block.source}" }
          }
          CopyButton {
            text: block.source.to_string(),
            label: if zh {
              format!("复制 {} 源码", block.title)
            } else {
              format!("Copy {} source", block.title)
            },
          }
        }
      }
      if let Some(reference) = reference {
        div { class: PROSE, "data-site-reference": "", dangerous_inner_html: reference }
      }
    }
  }
}

/// Renders a block in its own scope, so its hooks belong to it.
#[component]
fn BlockRender(slug: &'static str) -> Element {
  match BLOCKS.iter().find(|block| block.slug == slug) {
    Some(block) => (block.render)(),
    None => rsx! {},
  }
}

/// One example: its live preview, or its source on the Code tab.
#[component]
fn ExampleCard(index: usize) -> Element {
  let lang = use_site_lang()();
  let zh = lang == SiteLang::Zh;
  let Some(example) = EXAMPLES.get(index) else {
    return rsx! {};
  };
  let shown_title = i18n::example_title(lang, example.title);

  rsx! {
    section { class: "mt-6", "data-site-example": example.title,
      h3 { class: "font-semibold", "{shown_title}" }
      Tabs { class: "mt-3", default_value: "preview",
        TabsList {
          "aria-label": if zh {
            format!("{shown_title} 示例")
          } else {
            format!("{} example", example.title)
          },
          TabsTrigger { value: "preview", if zh { "预览" } else { "Preview" } }
          TabsTrigger { value: "code", if zh { "代码" } else { "Code" } }
        }
        TabsContent {
          value: "preview",
          class: "rounded-md border border-border p-6",
          div { "data-site-example-preview": "", ExampleRender { index } }
        }
        TabsContent { value: "code", class: "relative",
          pre { class: "{CODE_BLOCK} max-h-[32rem]", "data-site-example-source": "",
            code { "{example.source}" }
          }
          CopyButton {
            text: example.source.to_string(),
            label: if zh {
              format!("复制 {} 源码", shown_title)
            } else {
              format!("Copy {} source", example.title)
            },
          }
        }
      }
    }
  }
}

/// Renders an example in its own scope, so its hooks belong to it.
#[component]
fn ExampleRender(index: usize) -> Element {
  match EXAMPLES.get(index) {
    Some(example) => (example.render)(),
    None => rsx! {},
  }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
  rsx! { NotFoundContent { path: format!("/{}", segments.join("/")) } }
}

#[component]
fn NotFoundContent(path: String) -> Element {
  let zh = use_site_lang()() == SiteLang::Zh;

  rsx! {
    article { "data-site-page": "not-found",
      h1 { class: H1, if zh { "页面未找到" } else { "Page not found" } }
      p { class: LEAD,
        if zh {
          "路径 {path} 不存在。"
        } else {
          "Nothing lives at {path}."
        }
      }
      p { class: P,
        Link {
          class: "font-medium underline underline-offset-4",
          to: Route::Home {},
          if zh { "返回首页简介" } else { "Back to the introduction" }
        }
      }
    }
  }
}
