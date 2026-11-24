#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";
import { buildDocsCatalog } from "./docs-catalog-builder.mjs";
import {
  loadBlocks,
  loadThemes,
  readWorkspaceVersion,
  renderGeneratedPluginFiles,
} from "./claude-plugin-sync.mjs";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const failures = [];

const kebabCasePattern = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;

const readJson = (relativePath) => {
  const fullPath = join(repoRoot, relativePath);
  if (!existsSync(fullPath)) {
    failures.push(`missing required JSON file: ${relativePath}`);
    return null;
  }
  try {
    return JSON.parse(readFileSync(fullPath, "utf8"));
  } catch (error) {
    failures.push(`invalid JSON in ${relativePath}: ${error.message}`);
    return null;
  }
};

const workspaceVersion = readWorkspaceVersion(repoRoot);
const { publicComponentNames, featureNames } = buildDocsCatalog({ repoRoot });
const validComponents = new Set(publicComponentNames);
const validFeatures = new Set(featureNames);
const validBlocks = new Set(loadBlocks(repoRoot).map((block) => block.name));
const validThemes = new Set(loadThemes(repoRoot).map((theme) => theme.name));

// 1. Validate root marketplace manifest.
const marketplacePath = ".claude-plugin/marketplace.json";
const marketplace = readJson(marketplacePath);
let pluginEntry = null;

if (marketplace) {
  if (!marketplace.name || !kebabCasePattern.test(marketplace.name)) {
    failures.push(`${marketplacePath}: "name" must be non-empty kebab-case`);
  }
  if (!marketplace.owner?.name) {
    failures.push(`${marketplacePath}: "owner.name" is required`);
  }
  if (!Array.isArray(marketplace.plugins) || marketplace.plugins.length === 0) {
    failures.push(`${marketplacePath}: "plugins" must be a non-empty array`);
  } else {
    pluginEntry = marketplace.plugins.find((entry) => entry.name === "dioxus-ui") ?? null;
    if (!pluginEntry) {
      failures.push(`${marketplacePath}: missing plugin entry "dioxus-ui"`);
    } else {
      if (pluginEntry.version !== workspaceVersion) {
        failures.push(
          `${marketplacePath}: plugin "dioxus-ui" version (${pluginEntry.version}) does not match Cargo.toml (${workspaceVersion})`,
        );
      }
      if (typeof pluginEntry.source !== "string" || !pluginEntry.source.startsWith("./")) {
        failures.push(`${marketplacePath}: plugin "dioxus-ui" source must be a relative "./..." path`);
      } else {
        const resolvedSource = normalize(join(repoRoot, pluginEntry.source));
        if (!resolvedSource.startsWith(repoRoot) || !existsSync(resolvedSource)) {
          failures.push(`${marketplacePath}: plugin source path does not exist: ${pluginEntry.source}`);
        }
      }
    }
  }
}

// 2. Validate plugin manifest.
const pluginManifestPath = "plugins/dioxus-ui/.claude-plugin/plugin.json";
const pluginManifest = readJson(pluginManifestPath);

if (pluginManifest) {
  if (pluginManifest.name !== "dioxus-ui" || !kebabCasePattern.test(pluginManifest.name)) {
    failures.push(`${pluginManifestPath}: "name" must be "dioxus-ui" (kebab-case)`);
  }
  if (pluginManifest.version !== workspaceVersion) {
    failures.push(
      `${pluginManifestPath}: version (${pluginManifest.version}) does not match Cargo.toml (${workspaceVersion})`,
    );
  }
  if (!pluginManifest.description || typeof pluginManifest.description !== "string") {
    failures.push(`${pluginManifestPath}: "description" must be a non-empty string`);
  }
  if (!pluginManifest.author?.name) {
    failures.push(`${pluginManifestPath}: "author.name" is required`);
  }
  if (pluginManifest.license !== "MIT") {
    failures.push(`${pluginManifestPath}: expected license "MIT", found ${JSON.stringify(pluginManifest.license)}`);
  }
}

// 3. Validate SKILL.md frontmatter, reference links, and plugin README version alignment.
const skillPath = "plugins/dioxus-ui/skills/dioxus-ui/SKILL.md";
const skillFullPath = join(repoRoot, skillPath);
const pluginReadmePath = "plugins/dioxus-ui/README.md";
const pluginReadmeFullPath = join(repoRoot, pluginReadmePath);

const requiredFiles = [
  skillPath,
  pluginReadmePath,
  "plugins/dioxus-ui/skills/dioxus-ui/references/api-patterns.md",
  "plugins/dioxus-ui/skills/dioxus-ui/references/catalog.md",
  "plugins/dioxus-ui/skills/dioxus-ui/references/blocks.md",
  "plugins/dioxus-ui/skills/dioxus-ui/references/themes.md",
];

for (const relativePath of requiredFiles) {
  if (!existsSync(join(repoRoot, relativePath))) {
    failures.push(`missing required plugin file: ${relativePath}`);
  }
}

if (existsSync(skillFullPath)) {
  const skillText = readFileSync(skillFullPath, "utf8");
  const frontmatterMatch = skillText.match(/^---\n([\s\S]*?)\n---\n/);
  if (!frontmatterMatch) {
    failures.push(`${skillPath}: missing YAML frontmatter block`);
  } else {
    const fm = frontmatterMatch[1];
    if (!/^name:\s*dioxus-ui\s*$/m.test(fm)) {
      failures.push(`${skillPath}: frontmatter must declare "name: dioxus-ui"`);
    }
    if (!/^description:\s*.+/m.test(fm)) {
      failures.push(`${skillPath}: frontmatter must declare a non-empty "description"`);
    }
  }

  if (!skillText.includes(`\`${workspaceVersion}\``)) {
    failures.push(`${skillPath}: must state synced workspace version \`${workspaceVersion}\``);
  }
  if (!skillText.includes(`${validComponents.size} public components`)) {
    failures.push(`${skillPath}: component count does not match registry (${validComponents.size})`);
  }
  if (!skillText.includes(`${validBlocks.size} full-screen blocks`)) {
    failures.push(`${skillPath}: block count does not match registry (${validBlocks.size})`);
  }
  if (!skillText.includes(`${validThemes.size} WCAG-AA theme presets`)) {
    failures.push(`${skillPath}: theme count does not match registry (${validThemes.size})`);
  }

  // Ensure every markdown relative reference in SKILL.md resolves inside the plugin.
  for (const match of skillText.matchAll(/\[[^\]]+\]\((references\/[^)#\s]+)\)/g)) {
    const target = join(dirname(skillFullPath), match[1]);
    if (!existsSync(target)) {
      failures.push(`${skillPath}: broken reference link ${match[1]}`);
    }
  }
}

if (existsSync(pluginReadmeFullPath)) {
  const readmeText = readFileSync(pluginReadmeFullPath, "utf8");
  const requiredReadmeSnippets = [
    `\`${workspaceVersion}\``,
    "claude plugin marketplace add dioxuspress/dioxus-ui",
    "claude plugin install dioxus-ui@dioxuspress --scope user",
    "claude plugin install dioxus-ui@dioxuspress --scope project",
    "claude plugin install dioxus-ui@dioxuspress --scope local",
    "claude plugin update dioxus-ui@dioxuspress",
    "claude plugin uninstall dioxus-ui@dioxuspress",
    "Windows",
  ];
  for (const snippet of requiredReadmeSnippets) {
    if (!readmeText.includes(snippet)) {
      failures.push(`${pluginReadmePath}: missing expected documentation snippet: ${snippet}`);
    }
  }
}

// 4. Verify generated reference files have zero drift against source registries.
const expectedGenerated = renderGeneratedPluginFiles(repoRoot);
for (const [relativePath, expectedContent] of Object.entries(expectedGenerated)) {
  const fullPath = join(repoRoot, relativePath);
  if (!existsSync(fullPath)) {
    failures.push(`missing generated reference file: ${relativePath}`);
    continue;
  }
  const actualContent = readFileSync(fullPath, "utf8");
  if (actualContent !== expectedContent) {
    failures.push(
      `generated reference file ${relativePath} is stale; run \`npm run claude-plugin:sync\``,
    );
  }
}

// 5. Verify example CLI commands and Cargo feature lists inside plugin Markdown files
//    reference only real components, blocks, themes, and features in this release.
const markdownToScan = [
  skillPath,
  pluginReadmePath,
  "plugins/dioxus-ui/skills/dioxus-ui/references/api-patterns.md",
  "plugins/dioxus-ui/skills/dioxus-ui/references/themes.md",
];

for (const relativePath of markdownToScan) {
  const fullPath = join(repoRoot, relativePath);
  if (!existsSync(fullPath)) {
    continue;
  }
  const content = readFileSync(fullPath, "utf8");

  for (const match of content.matchAll(/^\s*dxui add ([a-z0-9 -]+)$/gm)) {
    const tokens = match[1].trim().split(/\s+/).filter((token) => !token.startsWith("--"));
    for (const token of tokens) {
      if (!validComponents.has(token) && !validBlocks.has(token)) {
        failures.push(`${relativePath}: \`dxui add ${token}\` is not a valid component or block`);
      }
    }
  }

  for (const match of content.matchAll(/^\s*dxui theme add ([a-z0-9 -]+)$/gm)) {
    const tokens = match[1].trim().split(/\s+/);
    for (const token of tokens) {
      if (!validThemes.has(token)) {
        failures.push(`${relativePath}: \`dxui theme add ${token}\` is not a valid theme preset`);
      }
    }
  }

  for (const match of content.matchAll(/features\s*=\s*\[([^\]]+)\]/g)) {
    for (const featMatch of match[1].matchAll(/"([a-z0-9_-]+)"/g)) {
      const feature = featMatch[1];
      if (!validFeatures.has(feature)) {
        failures.push(`${relativePath}: Cargo feature "${feature}" does not exist in dioxus-shadcn`);
      }
    }
  }
}

if (failures.length > 0) {
  console.error("Claude Code plugin verification failed:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(
    `Claude Code plugin verification passed (v${workspaceVersion}: ${validComponents.size} components, ${validBlocks.size} blocks, ${validThemes.size} themes)`,
  );
}
