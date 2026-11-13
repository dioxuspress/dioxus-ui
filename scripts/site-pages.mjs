#!/usr/bin/env node
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildDocsCatalog } from "./docs-catalog-builder.mjs";

// Prepares a `dx build` of the component site for GitHub Pages:
// 1. Preloads the WASM binary and links the compiled stylesheet directly in
//    `<head>` so CSS and WASM download in parallel with the JS entry module
//    instead of waiting for WASM boot to inject `document::Stylesheet`.
// 2. Renders a zero-JS instant app-shell skeleton that respects the stored
//    theme from `<head>` and hides via CSS `:has([data-site-root])` the exact
//    frame Dioxus mounts `Shell`.
// 3. Writes `index.html` into every static route directory (including `/blocks`
//    and `/blocks/:slug`) plus `404.html` so deep links load with HTTP 200.
const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const publicDir = process.argv[2];
if (!publicDir || !existsSync(join(publicDir, "index.html"))) {
  console.error("usage: node scripts/site-pages.mjs <dx web public dir containing index.html>");
  process.exit(1);
}

const assetsDir = join(publicDir, "assets");
const assetFiles = existsSync(assetsDir) ? readdirSync(assetsDir).sort() : [];
const cssFile = assetFiles.find((file) => /^site\.generated.*\.css$/.test(file));
const wasmFile = assetFiles.find((file) => /\.wasm$/.test(file));

const headInjections = [
  cssFile ? `<link rel="stylesheet" href="/assets/${cssFile}">` : "",
  wasmFile ? `<link rel="preload" href="/assets/${wasmFile}" as="fetch" type="application/wasm" crossorigin>` : "",
]
  .filter(Boolean)
  .join("");

const bootShell = `<div id="site-boot-shell" class="min-h-screen bg-background text-foreground [:has([data-site-root])_&]:hidden" aria-hidden="true"><div class="border-b border-border bg-background"><div class="mx-auto flex h-14 max-w-6xl items-center gap-3 px-4"><span class="inline-flex h-7 items-center gap-1 rounded-md border border-border bg-muted px-2 text-xs font-medium text-foreground">← <span class="hidden sm:inline">DioxusPress</span></span><span class="font-semibold tracking-tight">dioxus-shadcn</span><div class="hidden items-center gap-4 md:flex"><span class="h-4 w-10 animate-pulse rounded bg-muted"></span><span class="h-4 w-20 animate-pulse rounded bg-muted"></span><span class="h-4 w-12 animate-pulse rounded bg-muted"></span><span class="h-4 w-14 animate-pulse rounded bg-muted"></span></div><div class="ml-auto flex items-center gap-2"><span class="h-8 w-28 animate-pulse rounded-md border border-border bg-muted sm:w-36"></span></div></div></div><div class="mx-auto flex max-w-6xl gap-8 px-4"><div class="hidden w-56 shrink-0 space-y-3 py-6 md:block"><div class="h-4 w-28 animate-pulse rounded bg-muted"></div><div class="h-6 w-full animate-pulse rounded bg-muted"></div><div class="h-6 w-4/5 animate-pulse rounded bg-muted"></div><div class="h-6 w-3/4 animate-pulse rounded bg-muted"></div></div><div class="min-w-0 flex-1 space-y-4 py-8"><div class="h-9 w-56 animate-pulse rounded-md bg-muted"></div><div class="h-5 w-full max-w-xl animate-pulse rounded bg-muted"></div><div class="mt-6 grid gap-4 sm:grid-cols-2"><div class="h-32 animate-pulse rounded-md border border-border bg-muted"></div><div class="h-32 animate-pulse rounded-md border border-border bg-muted"></div><div class="h-32 animate-pulse rounded-md border border-border bg-muted"></div><div class="h-32 animate-pulse rounded-md border border-border bg-muted"></div></div></div></div></div>`;

const index = join(publicDir, "index.html");
let html = readFileSync(index, "utf8");
if (headInjections && !html.includes("/assets/site.generated")) {
  html = html.replace("</head>", `  ${headInjections}\n  </head>`);
}
if (!html.includes('id="site-boot-shell"')) {
  html = html.replace('<div id="main"></div>', `${bootShell}\n    <div id="main"></div>`);
}
writeFileSync(index, html);

const blocksDir = join(repoRoot, "crates/dioxus-shadcn-cli/blocks");
const blockNames = existsSync(blocksDir)
  ? readdirSync(blocksDir)
      .filter((file) => file.endsWith(".json"))
      .map((file) => file.replace(/\.json$/, ""))
      .sort()
  : [];

const routes = [
  "docs/getting-started",
  "docs/theming",
  "blocks",
  ...blockNames.map((name) => `blocks/${name}`),
  ...buildDocsCatalog().catalog.map((item) => `components/${item.slug}`),
];
for (const route of routes) {
  mkdirSync(join(publicDir, route), { recursive: true });
  copyFileSync(index, join(publicDir, route, "index.html"));
}
copyFileSync(index, join(publicDir, "404.html"));
console.log(
  `site pages: ${routes.length} route pages and 404.html in ${publicDir} (css=${cssFile ?? "none"}, wasm=${wasmFile ?? "none"})`,
);
