use dioxus::prelude::*;
use dioxus_shadcn::{CodeTabs, CodeTabsContent, CodeTabsList, CodeTabsTrigger};

#[component]
pub fn CodeTabsCompareDemo() -> Element {
  rsx! {
    CodeTabs { class: "max-w-xl", default_value: "typst",
      CodeTabsList { "aria-label": "Document markup comparison",
        CodeTabsTrigger { value: "markdown", "Markdown" }
        CodeTabsTrigger { value: "latex", "LaTeX" }
        CodeTabsTrigger { value: "typst", "Typst" }
      }
      CodeTabsContent { value: "markdown",
        pre { class: "font-mono text-sm leading-relaxed",
          code { "# Section Heading\n\nWrite **bold**, *italic*, and `inline code`.\n\n- First item\n- Second item" }
        }
      }
      CodeTabsContent { value: "latex",
        pre { class: "font-mono text-sm leading-relaxed",
          code { "\\section{{Section Heading}}\n\nWrite \\textbf{{bold}}, \\textit{{italic}}, and \\texttt{{inline code}}.\n\n\\begin{{itemize}}\n  \\item First item\n  \\item Second item\n\\end{{itemize}}" }
        }
      }
      CodeTabsContent { value: "typst",
        pre { class: "font-mono text-sm leading-relaxed",
          code { "= Section Heading\n\nWrite *bold*, _italic_, and `inline code`.\n\n- First item\n- Second item" }
        }
      }
    }
  }
}
