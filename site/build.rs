//! Renders each component's and block's docs page (English and Chinese) to
//! HTML for the component site (M199.2), so the site shows the localized
//! reference instead of linking to GitHub.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Component, Path, PathBuf};

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};

#[path = "src/highlight.rs"]
#[allow(dead_code)]
mod highlight;

/// Replaced at runtime with the site's base path, such as `/dioxus-ui`.
const SITE_BASE: &str = "__SITE_BASE__";

fn main() {
  let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let root = manifest.parent().expect("the site sits in the workspace root");
  let docs = root.join("docs/components");
  let zh_docs = manifest.join("i18n/zh/components");
  let blocks_docs = root.join("docs/blocks");
  let zh_blocks_docs = manifest.join("i18n/zh/blocks");
  let registry = root.join("crates/dioxus-shadcn-cli/registry");

  for watched in [&docs, &zh_docs, &blocks_docs, &zh_blocks_docs, &registry] {
    println!("cargo:rerun-if-changed={}", watched.display());
  }

  let slugs = fs::read_dir(&registry)
    .expect("read the CLI registry")
    .filter_map(|entry| entry.ok()?.path().file_stem()?.to_str().map(str::to_string))
    .filter(|name| name != "schema")
    .collect::<BTreeSet<_>>();
  let repository = env!("CARGO_PKG_REPOSITORY");

  let mut out = String::from("pub const REFERENCES_EN: &[(&str, &str)] = &[\n");
  for slug in &slugs {
    let path = docs.join(format!("{slug}.md"));
    println!("cargo:rerun-if-changed={}", path.display());
    let markdown = fs::read_to_string(&path).expect("read a component docs page");
    let start = markdown.find("## API Surface").unwrap_or(markdown.len());
    let rendered = render(&markdown[start..], &slugs, repository, "docs/components");
    writeln!(out, "  ({slug:?}, {rendered:?}),").expect("write to a string");
  }
  out.push_str("];\n");

  out.push_str("pub const REFERENCES_ZH: &[(&str, &str)] = &[\n");
  for slug in &slugs {
    let path = zh_docs.join(format!("{slug}.md"));
    println!("cargo:rerun-if-changed={}", path.display());
    let markdown = fs::read_to_string(&path).unwrap_or_else(|err| {
      panic!("missing Chinese component reference {}: {err}", path.display())
    });
    let start = markdown.find("## API 概览").unwrap_or_else(|| {
      panic!("Chinese component reference {} must start with `## API 概览`", path.display())
    });
    let rendered = render(&markdown[start..], &slugs, repository, "site/i18n/zh/components");
    writeln!(out, "  ({slug:?}, {rendered:?}),").expect("write to a string");
  }
  out.push_str("];\n");

  // Blocks (RFC 0073): each docs page from its Behavior section on.
  let mut block_pages = fs::read_dir(&blocks_docs)
    .expect("read the blocks docs")
    .filter_map(|entry| entry.ok().map(|entry| entry.path()))
    .filter(|path| path.file_name().is_some_and(|name| name != "README.md"))
    .collect::<Vec<_>>();
  block_pages.sort();

  out.push_str("pub const BLOCK_REFERENCES_EN: &[(&str, &str)] = &[\n");
  for path in &block_pages {
    println!("cargo:rerun-if-changed={}", path.display());
    let slug = path.file_stem().and_then(|stem| stem.to_str()).expect("block docs file name");
    let markdown = fs::read_to_string(path).expect("read a block docs page");
    let start = markdown.find("## Behavior").unwrap_or(markdown.len());
    let rendered = render(&markdown[start..], &slugs, repository, "docs/blocks");
    writeln!(out, "  ({slug:?}, {rendered:?}),").expect("write to a string");
  }
  out.push_str("];\n");

  out.push_str("pub const BLOCK_REFERENCES_ZH: &[(&str, &str)] = &[\n");
  for path in &block_pages {
    let slug = path.file_stem().and_then(|stem| stem.to_str()).expect("block docs file name");
    let zh_path = zh_blocks_docs.join(format!("{slug}.md"));
    println!("cargo:rerun-if-changed={}", zh_path.display());
    let markdown = fs::read_to_string(&zh_path)
      .unwrap_or_else(|err| panic!("missing Chinese block reference {}: {err}", zh_path.display()));
    let start = markdown.find("## 行为与交互").unwrap_or_else(|| {
      panic!("Chinese block reference {} must start with `## 行为与交互`", zh_path.display())
    });
    let rendered = render(&markdown[start..], &slugs, repository, "site/i18n/zh/blocks");
    writeln!(out, "  ({slug:?}, {rendered:?}),").expect("write to a string");
  }
  out.push_str("];\n");

  let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("cargo sets OUT_DIR"));
  fs::write(out_dir.join("references.rs"), out).expect("write the rendered references");
}

/// Markdown from a docs page under `dir` to HTML, with its links rewritten and
/// fenced code blocks syntax-highlighted.
fn render(markdown: &str, slugs: &BTreeSet<String>, repository: &str, dir: &str) -> String {
  let mut events: Vec<Event<'_>> = Vec::new();
  let mut code_lang: Option<String> = None;
  let mut code_buf = String::new();

  for event in Parser::new_ext(markdown, Options::ENABLE_TABLES) {
    match event {
      Event::Start(Tag::CodeBlock(kind)) => {
        let lang = match kind {
          CodeBlockKind::Fenced(info) => info.split_whitespace().next().unwrap_or("").to_string(),
          CodeBlockKind::Indented => String::new(),
        };
        code_lang = Some(lang);
        code_buf.clear();
      }
      Event::End(TagEnd::CodeBlock) => {
        if let Some(lang) = code_lang.take() {
          let highlighted = highlight::highlight_html(&code_buf, &lang);
          let html_block = if lang.is_empty() {
            format!(r#"<pre tabindex="0"><code>{highlighted}</code></pre>"#)
          } else {
            let escaped_lang = highlight::escape_html(&lang);
            format!(
              r#"<pre tabindex="0"><code class="language-{escaped_lang}">{highlighted}</code></pre>"#
            )
          };
          events.push(Event::Html(CowStr::from(html_block)));
          code_buf.clear();
        }
      }
      Event::Text(text) if code_lang.is_some() => {
        code_buf.push_str(&text);
      }
      Event::Start(Tag::Link { link_type, dest_url, title, id }) => {
        events.push(Event::Start(Tag::Link {
          link_type,
          dest_url: rewrite_link(&dest_url, slugs, repository, dir),
          title,
          id,
        }));
      }
      other => events.push(other),
    }
  }

  let mut rendered = String::new();
  html::push_html(&mut rendered, events.into_iter());
  rendered
}

/// Links to a component's docs page go to its site route; other relative
/// links go to the file on GitHub; absolute links and fragments stay.
fn rewrite_link(
  dest: &str,
  slugs: &BTreeSet<String>,
  repository: &str,
  dir: &str,
) -> CowStr<'static> {
  if dest.starts_with('#') || dest.contains("://") || dest.starts_with("mailto:") {
    return CowStr::from(dest.to_string());
  }
  let (path, fragment) =
    dest.split_once('#').map_or((dest, ""), |(path, fragment)| (path, fragment));
  let fragment = if fragment.is_empty() { String::new() } else { format!("#{fragment}") };
  let in_components =
    (dir == "docs/components" || dir == "site/i18n/zh/components") && !path.contains('/');
  if let Some(stem) = path.strip_suffix(".md").filter(|stem| in_components && slugs.contains(*stem))
  {
    return CowStr::from(format!("{SITE_BASE}/components/{stem}{fragment}"));
  }
  let mut resolved = PathBuf::from(dir);
  for part in Path::new(path).components() {
    match part {
      Component::ParentDir => {
        resolved.pop();
      }
      Component::Normal(name) => resolved.push(name),
      _ => {}
    }
  }
  if resolved.parent() == Some(Path::new("site/i18n/zh/components"))
    && let Some(stem) = resolved
      .file_name()
      .and_then(|name| name.to_str())
      .and_then(|name| name.strip_suffix(".md"))
      .filter(|stem| slugs.contains(*stem))
  {
    return CowStr::from(format!("{SITE_BASE}/components/{stem}{fragment}"));
  }
  CowStr::from(format!("{repository}/blob/main/{}{fragment}", resolved.display()))
}
