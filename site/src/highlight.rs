//! Lightweight, zero-dependency syntax highlighter for the `dioxus-ui` site.
//!
//! Used both at build time (`build.rs`) for Markdown prose code blocks and at
//! runtime (`pages.rs`) for component/block source tabs and `CodeBlock` widgets.
//! Every character of the input is preserved verbatim inside `<span class="hl-*">`
//! wrappers so `textContent` and copy-to-clipboard remain lossless.

use std::fmt::Write as _;

/// Highlights `source` according to `lang` into HTML containing `<span class="hl-*">`
/// elements. Unrecognized or empty languages return HTML-escaped plain text.
pub fn highlight_html(source: &str, lang: &str) -> String {
  let normalized = lang.trim().to_ascii_lowercase();
  match normalized.as_str() {
    "rust" | "rs" => highlight_rust(source),
    "bash" | "sh" | "shell" | "zsh" => highlight_bash(source),
    "toml" => highlight_toml(source),
    "css" => highlight_css(source),
    "json" => highlight_json(source),
    _ => escape_html(source),
  }
}

/// Escapes HTML special characters in `text`.
pub fn escape_html(text: &str) -> String {
  let mut out = String::with_capacity(text.len());
  push_escaped(&mut out, text);
  out
}

fn push_escaped(out: &mut String, text: &str) {
  for ch in text.chars() {
    match ch {
      '&' => out.push_str("&amp;"),
      '<' => out.push_str("&lt;"),
      '>' => out.push_str("&gt;"),
      '"' => out.push_str("&quot;"),
      _ => out.push(ch),
    }
  }
}

fn push_span(out: &mut String, class: &str, text: &str) {
  if text.is_empty() {
    return;
  }
  let _ = write!(out, r#"<span class="{class}">"#);
  push_escaped(out, text);
  out.push_str("</span>");
}

fn is_ident_start(b: u8) -> bool {
  b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_continue(b: u8) -> bool {
  b.is_ascii_alphanumeric() || b == b'_'
}

fn scan_quoted(bytes: &[u8], start: usize, quote: u8) -> usize {
  let mut i = start + 1;
  while i < bytes.len() {
    if bytes[i] == b'\\' && i + 1 < bytes.len() {
      i += 2;
      continue;
    }
    if bytes[i] == quote {
      return i + 1;
    }
    i += 1;
  }
  bytes.len()
}

fn scan_rust_raw_string(bytes: &[u8], start: usize) -> Option<usize> {
  let mut i = start;
  if i < bytes.len() && bytes[i] == b'b' {
    i += 1;
  }
  if i >= bytes.len() || bytes[i] != b'r' {
    return None;
  }
  i += 1;
  let mut hashes = 0usize;
  while i < bytes.len() && bytes[i] == b'#' {
    hashes += 1;
    i += 1;
  }
  if i >= bytes.len() || bytes[i] != b'"' {
    return None;
  }
  i += 1;
  while i < bytes.len() {
    if bytes[i] == b'"' {
      let mut matched = 0usize;
      while matched < hashes && i + 1 + matched < bytes.len() && bytes[i + 1 + matched] == b'#' {
        matched += 1;
      }
      if matched == hashes {
        return Some(i + 1 + hashes);
      }
    }
    i += 1;
  }
  Some(bytes.len())
}

fn is_rust_keyword(word: &str) -> bool {
  matches!(
    word,
    "as"
      | "async"
      | "await"
      | "break"
      | "const"
      | "continue"
      | "crate"
      | "dyn"
      | "else"
      | "enum"
      | "extern"
      | "fn"
      | "for"
      | "if"
      | "impl"
      | "in"
      | "let"
      | "loop"
      | "match"
      | "mod"
      | "move"
      | "mut"
      | "pub"
      | "ref"
      | "return"
      | "self"
      | "static"
      | "struct"
      | "super"
      | "trait"
      | "type"
      | "unsafe"
      | "use"
      | "where"
      | "while"
  )
}

fn highlight_rust(source: &str) -> String {
  let bytes = source.as_bytes();
  let mut out = String::with_capacity(source.len() * 2);
  let mut i = 0usize;

  while i < bytes.len() {
    // Line comment
    if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
      let start = i;
      while i < bytes.len() && bytes[i] != b'\n' {
        i += 1;
      }
      push_span(&mut out, "hl-comment", &source[start..i]);
      continue;
    }

    // Block comment (supports nesting)
    if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
      let start = i;
      i += 2;
      let mut depth = 1usize;
      while i < bytes.len() && depth > 0 {
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
          depth += 1;
          i += 2;
        } else if i + 1 < bytes.len() && bytes[i] == b'*' && bytes[i + 1] == b'/' {
          depth -= 1;
          i += 2;
        } else {
          i += 1;
        }
      }
      push_span(&mut out, "hl-comment", &source[start..i]);
      continue;
    }

    // Attribute #[...] or #![...]
    if bytes[i] == b'#'
      && ((i + 1 < bytes.len() && bytes[i + 1] == b'[')
        || (i + 2 < bytes.len() && bytes[i + 1] == b'!' && bytes[i + 2] == b'['))
    {
      let start = i;
      while i < bytes.len() && bytes[i] != b'[' {
        i += 1;
      }
      let mut depth = 0usize;
      while i < bytes.len() {
        if bytes[i] == b'"' {
          i = scan_quoted(bytes, i, b'"');
          continue;
        }
        if bytes[i] == b'[' {
          depth += 1;
        } else if bytes[i] == b']' {
          depth = depth.saturating_sub(1);
          i += 1;
          if depth == 0 {
            break;
          }
          continue;
        }
        i += 1;
      }
      push_span(&mut out, "hl-attr", &source[start..i]);
      continue;
    }

    // Raw string literal r"..." / r#"..."# / br#"..."#
    if (bytes[i] == b'r' || bytes[i] == b'b')
      && let Some(end) = scan_rust_raw_string(bytes, i)
    {
      push_span(&mut out, "hl-str", &source[i..end]);
      i = end;
      continue;
    }

    // Byte string b"..." or normal string "..."
    if bytes[i] == b'b' && i + 1 < bytes.len() && bytes[i + 1] == b'"' {
      let end = scan_quoted(bytes, i + 1, b'"');
      push_span(&mut out, "hl-str", &source[i..end]);
      i = end;
      continue;
    }
    if bytes[i] == b'"' {
      let end = scan_quoted(bytes, i, b'"');
      push_span(&mut out, "hl-str", &source[i..end]);
      i = end;
      continue;
    }

    // Char literal vs lifetime ('a / 'static)
    if bytes[i] == b'\'' {
      let start = i;
      if i + 1 < bytes.len() && bytes[i + 1] == b'\\' {
        let end = scan_quoted(bytes, i, b'\'');
        push_span(&mut out, "hl-str", &source[start..end]);
        i = end;
        continue;
      }
      if i + 2 < bytes.len() && bytes[i + 2] == b'\'' {
        i += 3;
        push_span(&mut out, "hl-str", &source[start..i]);
        continue;
      }
      if i + 1 < bytes.len() && is_ident_start(bytes[i + 1]) {
        i += 2;
        while i < bytes.len() && is_ident_continue(bytes[i]) {
          i += 1;
        }
        push_span(&mut out, "hl-num", &source[start..i]);
        continue;
      }
    }

    // Numbers
    if bytes[i].is_ascii_digit() {
      let start = i;
      i += 1;
      while i < bytes.len()
        && (bytes[i].is_ascii_alphanumeric()
          || bytes[i] == b'_'
          || (bytes[i] == b'.' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit()))
      {
        i += 1;
      }
      push_span(&mut out, "hl-num", &source[start..i]);
      continue;
    }

    // Identifiers, keywords, macros, types, functions
    if is_ident_start(bytes[i]) {
      let start = i;
      i += 1;
      while i < bytes.len() && is_ident_continue(bytes[i]) {
        i += 1;
      }
      let word = &source[start..i];
      if i < bytes.len() && bytes[i] == b'!' && !(i + 1 < bytes.len() && bytes[i + 1] == b'=') {
        i += 1;
        push_span(&mut out, "hl-fn", &source[start..i]);
      } else if is_rust_keyword(word) {
        push_span(&mut out, "hl-kw", word);
      } else if word == "true"
        || word == "false"
        || word == "None"
        || word == "Some"
        || word == "Ok"
        || word == "Err"
      {
        push_span(&mut out, "hl-num", word);
      } else if word.as_bytes().first().is_some_and(u8::is_ascii_uppercase) {
        push_span(&mut out, "hl-type", word);
      } else if i < bytes.len() && bytes[i] == b'(' {
        push_span(&mut out, "hl-fn", word);
      } else {
        push_escaped(&mut out, word);
      }
      continue;
    }

    // Multi-char UTF-8 or single ASCII punctuation
    let ch = source[i..].chars().next().unwrap_or(' ');
    let len = ch.len_utf8();
    push_escaped(&mut out, &source[i..i + len]);
    i += len;
  }

  out
}

fn highlight_bash(source: &str) -> String {
  let mut out = String::with_capacity(source.len() * 2);
  for (line_idx, line) in source.split_inclusive('\n').enumerate() {
    let _ = line_idx;
    let bytes = line.as_bytes();
    let mut i = 0usize;
    let mut expect_command = true;

    while i < bytes.len() {
      if bytes[i].is_ascii_whitespace() {
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
          i += 1;
        }
        push_escaped(&mut out, &line[start..i]);
        continue;
      }

      if bytes[i] == b'#' {
        push_span(&mut out, "hl-comment", &line[i..]);
        break;
      }

      if bytes[i] == b'"' || bytes[i] == b'\'' {
        let end = scan_quoted(bytes, i, bytes[i]);
        push_span(&mut out, "hl-str", &line[i..end]);
        i = end;
        expect_command = false;
        continue;
      }

      if bytes[i] == b'$' {
        let start = i;
        i += 1;
        while i < bytes.len()
          && (is_ident_continue(bytes[i]) || bytes[i] == b'{' || bytes[i] == b'}')
        {
          i += 1;
        }
        push_span(&mut out, "hl-num", &line[start..i]);
        expect_command = false;
        continue;
      }

      if matches!(bytes[i], b'|' | b'&' | b';') {
        let start = i;
        i += 1;
        if i < bytes.len() && bytes[i] == bytes[start] {
          i += 1;
        }
        push_span(&mut out, "hl-kw", &line[start..i]);
        expect_command = true;
        continue;
      }

      let start = i;
      while i < bytes.len()
        && !bytes[i].is_ascii_whitespace()
        && !matches!(bytes[i], b'"' | b'\'' | b'|' | b'&' | b';')
      {
        i += 1;
      }
      let token = &line[start..i];
      if expect_command {
        push_span(&mut out, "hl-fn", token);
        expect_command = false;
      } else if token.starts_with('-') {
        push_span(&mut out, "hl-attr", token);
      } else {
        push_escaped(&mut out, token);
      }
    }
  }
  out
}

fn highlight_toml(source: &str) -> String {
  let mut out = String::with_capacity(source.len() * 2);
  for line in source.split_inclusive('\n') {
    let bytes = line.as_bytes();
    let mut i = 0usize;
    let mut at_line_start = true;

    while i < bytes.len() {
      if bytes[i].is_ascii_whitespace() {
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
          i += 1;
        }
        push_escaped(&mut out, &line[start..i]);
        continue;
      }

      if bytes[i] == b'#' {
        push_span(&mut out, "hl-comment", &line[i..]);
        break;
      }

      if at_line_start && bytes[i] == b'[' {
        let start = i;
        while i < bytes.len() && bytes[i] != b']' && bytes[i] != b'\n' {
          i += 1;
        }
        if i < bytes.len() && bytes[i] == b']' {
          i += 1;
        }
        push_span(&mut out, "hl-attr", &line[start..i]);
        at_line_start = false;
        continue;
      }

      if bytes[i] == b'"' || bytes[i] == b'\'' {
        let end = scan_quoted(bytes, i, bytes[i]);
        push_span(&mut out, "hl-str", &line[i..end]);
        i = end;
        at_line_start = false;
        continue;
      }

      if is_ident_start(bytes[i]) || bytes[i] == b'-' {
        let start = i;
        i += 1;
        while i < bytes.len() && (is_ident_continue(bytes[i]) || bytes[i] == b'-') {
          i += 1;
        }
        let word = &line[start..i];
        let rest_trimmed = line[i..].trim_start();
        if rest_trimmed.starts_with('=') {
          push_span(&mut out, "hl-type", word);
        } else if word == "true" || word == "false" || word.bytes().all(|b| b.is_ascii_digit()) {
          push_span(&mut out, "hl-num", word);
        } else {
          push_escaped(&mut out, word);
        }
        at_line_start = false;
        continue;
      }

      if bytes[i].is_ascii_digit() {
        let start = i;
        while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.' || bytes[i] == b'_')
        {
          i += 1;
        }
        push_span(&mut out, "hl-num", &line[start..i]);
        at_line_start = false;
        continue;
      }

      let ch = line[i..].chars().next().unwrap_or(' ');
      let len = ch.len_utf8();
      push_escaped(&mut out, &line[i..i + len]);
      i += len;
      at_line_start = false;
    }
  }
  out
}

fn highlight_css(source: &str) -> String {
  let bytes = source.as_bytes();
  let mut out = String::with_capacity(source.len() * 2);
  let mut i = 0usize;

  while i < bytes.len() {
    if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
      let start = i;
      i += 2;
      while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
        i += 1;
      }
      i = (i + 2).min(bytes.len());
      push_span(&mut out, "hl-comment", &source[start..i]);
      continue;
    }

    if bytes[i] == b'"' || bytes[i] == b'\'' {
      let end = scan_quoted(bytes, i, bytes[i]);
      push_span(&mut out, "hl-str", &source[i..end]);
      i = end;
      continue;
    }

    if bytes[i] == b'@' {
      let start = i;
      i += 1;
      while i < bytes.len() && (is_ident_continue(bytes[i]) || bytes[i] == b'-') {
        i += 1;
      }
      push_span(&mut out, "hl-kw", &source[start..i]);
      continue;
    }

    if bytes[i] == b'-' && i + 1 < bytes.len() && bytes[i + 1] == b'-' {
      let start = i;
      i += 2;
      while i < bytes.len() && (is_ident_continue(bytes[i]) || bytes[i] == b'-') {
        i += 1;
      }
      push_span(&mut out, "hl-type", &source[start..i]);
      continue;
    }

    if bytes[i].is_ascii_digit() {
      let start = i;
      while i < bytes.len()
        && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.' || bytes[i] == b'%')
      {
        i += 1;
      }
      push_span(&mut out, "hl-num", &source[start..i]);
      continue;
    }

    if is_ident_start(bytes[i]) {
      let start = i;
      while i < bytes.len() && (is_ident_continue(bytes[i]) || bytes[i] == b'-') {
        i += 1;
      }
      let word = &source[start..i];
      if i < bytes.len() && bytes[i] == b'(' {
        push_span(&mut out, "hl-fn", word);
      } else {
        push_escaped(&mut out, word);
      }
      continue;
    }

    let ch = source[i..].chars().next().unwrap_or(' ');
    let len = ch.len_utf8();
    push_escaped(&mut out, &source[i..i + len]);
    i += len;
  }
  out
}

fn highlight_json(source: &str) -> String {
  let bytes = source.as_bytes();
  let mut out = String::with_capacity(source.len() * 2);
  let mut i = 0usize;

  while i < bytes.len() {
    if bytes[i] == b'"' {
      let end = scan_quoted(bytes, i, b'"');
      let is_key = source[end..].trim_start().starts_with(':');
      push_span(&mut out, if is_key { "hl-type" } else { "hl-str" }, &source[i..end]);
      i = end;
      continue;
    }

    if bytes[i].is_ascii_digit()
      || (bytes[i] == b'-' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit())
    {
      let start = i;
      i += 1;
      while i < bytes.len()
        && (bytes[i].is_ascii_alphanumeric()
          || bytes[i] == b'.'
          || bytes[i] == b'+'
          || bytes[i] == b'-')
      {
        i += 1;
      }
      push_span(&mut out, "hl-num", &source[start..i]);
      continue;
    }

    if is_ident_start(bytes[i]) {
      let start = i;
      while i < bytes.len() && is_ident_continue(bytes[i]) {
        i += 1;
      }
      let word = &source[start..i];
      if matches!(word, "true" | "false" | "null") {
        push_span(&mut out, "hl-num", word);
      } else {
        push_escaped(&mut out, word);
      }
      continue;
    }

    let ch = source[i..].chars().next().unwrap_or(' ');
    let len = ch.len_utf8();
    push_escaped(&mut out, &source[i..i + len]);
    i += len;
  }
  out
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rust_highlights_keywords_types_macros_strings_and_comments() {
    let src = "#[component]\npub fn Demo() -> Element {\n  // hello\n  let s = \"world\";\n  rsx! { Button {} }\n}";
    let html = highlight_html(src, "rust");
    assert!(html.contains(r#"<span class="hl-attr">#[component]</span>"#));
    assert!(html.contains(r#"<span class="hl-kw">pub</span>"#));
    assert!(html.contains(r#"<span class="hl-kw">fn</span>"#));
    assert!(html.contains(r#"<span class="hl-type">Element</span>"#));
    assert!(html.contains(r#"<span class="hl-comment">// hello</span>"#));
    assert!(html.contains(r#"<span class="hl-str">&quot;world&quot;</span>"#));
    assert!(html.contains(r#"<span class="hl-fn">rsx!</span>"#));
  }

  #[test]
  fn rust_handles_lifetimes_and_raw_strings() {
    let src = "struct F { t: &'static str, r: r#\"hi\"# }";
    let html = highlight_html(src, "rust");
    assert!(
      html.contains(r#"<span class="hl-num">&#x27;static</span>"#)
        || html.contains(r#"<span class="hl-num">'static</span>"#)
    );
    assert!(html.contains(r#"<span class="hl-str">r#&quot;hi&quot;#</span>"#));
  }

  #[test]
  fn bash_toml_css_highlight_expected_tokens() {
    let bash = highlight_html("dxui add button --overwrite", "bash");
    assert!(bash.contains(r#"<span class="hl-fn">dxui</span>"#));
    assert!(bash.contains(r#"<span class="hl-attr">--overwrite</span>"#));

    let toml = highlight_html("[dependencies]\ndioxus-shadcn = \"0.6\"", "toml");
    assert!(toml.contains(r#"<span class="hl-attr">[dependencies]</span>"#));
    assert!(toml.contains(r#"<span class="hl-type">dioxus-shadcn</span>"#));
    assert!(toml.contains(r#"<span class="hl-str">&quot;0.6&quot;</span>"#));

    let css =
      highlight_html("@import \"tailwindcss\";\n:root { --primary: oklch(0.5 0.2 260); }", "css");
    assert!(css.contains(r#"<span class="hl-kw">@import</span>"#));
    assert!(css.contains(r#"<span class="hl-type">--primary</span>"#));
    assert!(css.contains(r#"<span class="hl-fn">oklch</span>"#));
  }

  #[test]
  fn unknown_language_escapes_html_without_spans() {
    let html = highlight_html("a < b & c > d", "text");
    assert_eq!(html, "a &lt; b &amp; c &gt; d");
  }
}
