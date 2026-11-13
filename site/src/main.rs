//! The dioxus-shadcn component site (RFC 0052): the catalog, each component's
//! page, and the setup and theming guides, built on the published components.

mod blocks;
mod catalog;
mod components;
mod examples;
mod i18n;
mod pages;
mod themes;

use dioxus::prelude::*;
use dioxus_shadcn::{
  Button, ButtonSize, ButtonVariant, NativeSelect, NativeSelectGroup, NativeSelectOption, Sheet,
  SheetContent, SheetOverlay, SheetSide, SheetTitle, Theme, ThemeController,
};
use i18n::{SiteLang, use_site_lang};
use pages::{BlockPage, Blocks, ComponentPage, GettingStarted, Home, NotFound, Theming};

const SITE_CSS: Asset = asset!("/assets/site.generated.css");

// `dioxus-router-macro` emits a sibling `const _: () = { ... }` with a trailing
// error return after the infallible `/:..segments` catch-all arm, which Rust
// 1.100+ flags as `unreachable_code`. Scoping the derive in a module keeps the
// lint active everywhere else in `main.rs`.
#[allow(unreachable_code)]
mod route {
  use super::*;

  #[derive(Routable, Clone, PartialEq)]
  #[rustfmt::skip]
  pub enum Route {
    #[layout(Shell)]
      #[route("/")]
      Home {},
      #[route("/docs/getting-started")]
      GettingStarted {},
      #[route("/docs/theming")]
      Theming {},
      #[route("/components/:slug")]
      ComponentPage { slug: String },
      #[route("/blocks")]
      Blocks {},
      #[route("/blocks/:slug")]
      BlockPage { slug: String },
      #[route("/:..segments")]
      NotFound { segments: Vec<String> },
  }
}
pub use route::Route;

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  // Resolve `?lang=` before `Router::<Route>` mounts: Dioxus Router normalizes
  // unknown query strings via `history.replaceState` during `Router` setup.
  use_context_provider(|| Signal::new(i18n::detect_initial_lang()));
  rsx! {
    document::Stylesheet { href: SITE_CSS }
    Router::<Route> {}
  }
}

/// The header, the catalog sidebar, and the routed page. The theme menu picks
/// the system scheme, light, dark, or a preset (RFC 0057), which the
/// `ThemeController` applies to the document root and remembers (RFC 0071);
/// pages read and set the same theme and language (`en` / `zh`) through context.
#[component]
fn Shell() -> Element {
  let mut theme = use_context_provider(|| Signal::new(Theme::System));
  let mut lang = use_site_lang();
  let mut menu_open = use_signal(|| false);
  let current_lang = lang();
  let zh = current_lang == SiteLang::Zh;

  rsx! {
    ThemeController { theme: theme(), on_theme_change: move |stored| theme.set(stored) }
    div {
      class: "min-h-screen bg-background text-foreground",
      "data-site-root": "",
      "data-site-lang": current_lang.code(),
      header { class: "sticky top-0 z-40 border-b border-border bg-background",
        div { class: "mx-auto flex h-14 max-w-6xl items-center gap-1.5 px-4 sm:gap-3",
          Button {
            class: "md:hidden",
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Sm,
            "aria-label": if zh { "打开组件导航菜单" } else { "Open the component menu" },
            onclick: move |_| menu_open.set(true),
            if zh { "菜单" } else { "Menu" }
          }
          a {
            class: "inline-flex h-7 shrink-0 items-center gap-1 rounded-md border border-border bg-muted px-2 text-xs font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground",
            href: current_lang.main_site_href(),
            "aria-label": if zh { "返回 DioxusPress 主站" } else { "Back to DioxusPress" },
            title: if zh { "返回 DioxusPress 主站" } else { "Back to DioxusPress main site" },
            "data-site-back-link": "",
            svg {
              class: "h-3.5 w-3.5 shrink-0",
              fill: "none",
              stroke: "currentColor",
              view_box: "0 0 24 24",
              "aria-hidden": "true",
              path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M10 19l-7-7m0 0l7-7m-7 7h18",
              }
            }
            span { class: "hidden sm:inline", "DioxusPress" }
          }
          Link {
            class: "whitespace-nowrap text-sm font-semibold tracking-tight sm:text-base",
            to: Route::Home {},
            "dioxus-shadcn"
          }
          nav { class: "hidden items-center gap-4 text-sm text-muted-foreground md:flex",
            Link {
              class: "hover:text-foreground",
              to: Route::GettingStarted {},
              if zh { "文档" } else { "Docs" }
            }
            Link {
              class: "hover:text-foreground",
              to: Route::ComponentPage { slug: "button".to_string() },
              if zh { "组件" } else { "Components" }
            }
            Link {
              class: "hover:text-foreground",
              to: Route::Blocks {},
              if zh { "区块" } else { "Blocks" }
            }
            Link {
              class: "hover:text-foreground",
              to: Route::Theming {},
              if zh { "主题" } else { "Theming" }
            }
          }
          div { class: "ml-auto flex items-center gap-1.5 sm:gap-2",
            label {
              class: "hidden items-center gap-1.5 text-xs font-medium text-muted-foreground lg:inline-flex",
              r#for: "site-theme",
              svg {
                class: "h-3.5 w-3.5 shrink-0",
                fill: "none",
                stroke: "currentColor",
                view_box: "0 0 24 24",
                "aria-hidden": "true",
                path {
                  stroke_linecap: "round",
                  stroke_linejoin: "round",
                  stroke_width: "2",
                  d: "M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01",
                }
              }
              if zh { "主题" } else { "Theme" }
            }
            NativeSelect {
              id: "site-theme",
              class: "h-8 w-24 sm:w-36",
              "aria-label": if zh { "主题" } else { "Theme" },
              on_value_change: move |value: String| theme.set(Theme::parse(&value)),
              NativeSelectGroup { label: if zh { "默认模式" } else { "Default" },
                for option in [Theme::System, Theme::Light, Theme::Dark] {
                  NativeSelectOption {
                    key: "{option.as_str()}",
                    value: option.as_str().to_string(),
                    selected: theme() == option,
                    match option {
                      Theme::Light => if zh { "浅色" } else { "Light" },
                      Theme::Dark => if zh { "深色" } else { "Dark" },
                      _ => if zh { "跟随系统" } else { "System" },
                    }
                  }
                }
              }
              for (group_key, label, dark) in [
                ("light", if zh { "浅色预设" } else { "Light presets" }, false),
                ("dark", if zh { "深色预设" } else { "Dark presets" }, true),
              ] {
                NativeSelectGroup { key: "{group_key}", label,
                  for preset in themes::THEMES.iter().filter(|preset| preset.dark == dark) {
                    NativeSelectOption {
                      key: "{preset.name}",
                      value: preset.name,
                      selected: theme().as_str() == preset.name,
                      "{preset.title}"
                    }
                  }
                }
              }
            }
            button {
              r#type: "button",
              class: "inline-flex h-8 shrink-0 items-center justify-center rounded-md border border-border bg-muted px-2 text-xs font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground",
              "aria-label": current_lang.toggle_aria_label(),
              title: current_lang.toggle_aria_label(),
              "data-site-lang-toggle": "",
              onclick: move |_| {
                let next = lang().toggle();
                i18n::persist_lang(next);
                lang.set(next);
              },
              "{current_lang.toggle_button_text()}"
            }
            a {
              class: "hidden h-8 w-8 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground sm:inline-flex",
              href: "https://github.com/dioxuspress/dioxus-ui",
              "aria-label": if zh { "GitHub 源码仓库" } else { "GitHub repository" },
              title: if zh { "GitHub 源码仓库" } else { "GitHub repository" },
              svg {
                class: "h-4 w-4",
                fill: "currentColor",
                view_box: "0 0 24 24",
                "aria-hidden": "true",
                path {
                  fill_rule: "evenodd",
                  clip_rule: "evenodd",
                  d: "M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.53 1.032 1.53 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z",
                }
              }
            }
          }
        }
      }
      div { class: "mx-auto flex max-w-6xl gap-8 px-4",
        aside { class: "sticky top-14 hidden h-[calc(100vh-3.5rem)] w-56 shrink-0 overflow-y-auto py-6 md:block",
          SiteNav {}
        }
        main { class: "min-w-0 flex-1 py-8", Outlet::<Route> {} }
      }
      Sheet { open: menu_open(), on_open_change: move |open| menu_open.set(open),
        SheetOverlay {}
        SheetContent { class: "overflow-y-auto", side: SheetSide::Left,
          SheetTitle { class: "mb-4", if zh { "组件目录" } else { "Components" } }
          // A followed link bubbles here, so the menu closes on navigation.
          div { onclick: move |_| menu_open.set(false), SiteNav {} }
        }
      }
    }
  }
}

/// The guides and the catalog grouped by category, from `catalog.rs`.
#[component]
fn SiteNav() -> Element {
  let lang = use_site_lang()();
  let zh = lang == SiteLang::Zh;
  let link = "block rounded-md px-2 py-1 text-sm text-muted-foreground hover:bg-accent hover:text-foreground";
  let active = "bg-accent text-foreground";

  rsx! {
    nav { class: "grid gap-6", "aria-label": if zh { "站点导航" } else { "Site" },
      section { class: "grid gap-1",
        h2 { class: "px-2 text-sm font-semibold", if zh { "快速开始" } else { "Getting Started" } }
        Link {
          class: link,
          active_class: active,
          to: Route::Home {},
          if zh { "简介" } else { "Introduction" }
        }
        Link {
          class: link,
          active_class: active,
          to: Route::GettingStarted {},
          if zh { "安装指南" } else { "Installation" }
        }
        Link {
          class: link,
          active_class: active,
          to: Route::Theming {},
          if zh { "主题定制" } else { "Theming" }
        }
        a {
          class: link,
          href: lang.main_site_href(),
          if zh { "← 返回 DioxusPress 主站" } else { "← DioxusPress Home" }
        }
      }
      section { class: "grid gap-1",
        h2 { class: "px-2 text-sm font-semibold", if zh { "页面区块" } else { "Blocks" } }
        for block in blocks::BLOCKS {
          Link {
            key: "{block.slug}",
            class: link,
            active_class: active,
            to: Route::BlockPage { slug: block.slug.to_string() },
            "{i18n::block_title(lang, block.slug, block.title)}"
          }
        }
      }
      for category in catalog::CATEGORIES {
        section { key: "{category.id}", id: "category-{category.id}", class: "grid gap-1",
          h2 {
            class: "px-2 text-sm font-semibold",
            "{i18n::category_label(lang, category.id, category.label)}"
          }
          for component in category.components {
            Link {
              key: "{component.slug}",
              class: link,
              active_class: active,
              to: Route::ComponentPage { slug: component.slug.to_string() },
              "{component.title}"
            }
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Route;
  use dioxus_shadcn::{THEME_STORAGE_KEY, theme_init_script};
  use std::str::FromStr;

  #[test]
  fn page_template_applies_the_stored_theme_before_the_app_loads() {
    let template = include_str!("../index.html");

    assert!(
      template.contains(&format!("<script>{}</script>", theme_init_script(THEME_STORAGE_KEY)))
    );
    assert!(template.contains(r#"<html lang="en">"#));
  }

  #[test]
  fn routes_parse_with_lang_query_param() {
    assert!(Route::from_str("/?lang=en").ok() == Some(Route::Home {}));
    assert!(
      Route::from_str("/components/button?lang=zh").ok()
        == Some(Route::ComponentPage { slug: "button".to_string() })
    );
  }
}
