#!/usr/bin/env -S deno run -A

/**
 * Static site generator for GitHub Pages.
 * Reads markdown posts from posts/ and generates static HTML in docs/.
 */

import { extname, join, dirname } from "std/path/mod.ts";
import { ensureDir, copy } from "std/fs/mod.ts";
import { parse } from "std/yaml/parse.ts";
import { marked } from "https://esm.sh/marked@5.1.1";
import katex from "https://esm.sh/katex@0.16.9";

// ── Types ──────────────────────────────────────────────────────────────────

interface Post {
  slug: string;
  title: string;
  publish_date: string;
  tags: string[];
  description: string;
  content: string;
  url: string; // e.g. /2016/1/28/tdd-kata-2-the-bowling-game
}

// ── Configuration ──────────────────────────────────────────────────────────

// Set BASE_PATH to "/nomkhonwaan" for project site (nomkhonwaan.github.io/nomkhonwaan)
// Set to "" for custom domain (nomkhonwaan.com) or user site (nomkhonwaan.github.io)
const BASE_PATH = "";

// ── Markdown helpers ───────────────────────────────────────────────────────

function renderKaTeX(md: string): string {
  // Render display math $$...$$ first
  let result = md.replace(/\$\$([\s\S]+?)\$\$/g, (_, expr) => {
    try {
      return katex.renderToString(expr.trim(), { displayMode: true, throwOnError: false });
    } catch {
      return `$$${expr}$$`;
    }
  });
  // Render inline math $...$ (but not inside code blocks or already-replaced areas)
  // Split into lines to avoid code blocks
  const lines = result.split('\n');
  let inCodeBlock = false;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (line.startsWith('```')) {
      inCodeBlock = !inCodeBlock;
      continue;
    }
    if (inCodeBlock) continue;
    // Replace inline $...$ math
    lines[i] = line.replace(/(?<!\$)\$([^$\n]+?)\$(?!\$)/g, (_, expr) => {
      try {
        return katex.renderToString(expr.trim(), { displayMode: false, throwOnError: false });
      } catch {
        return `$${expr}$`;
      }
    });
  }
  return lines.join('\n');
}

function renderMarkdown(md: string): string {
  try {
    // First render KaTeX, then pass to marked
    const withMath = renderKaTeX(md);
    let html = marked.parse(withMath, { mangle: false, headerIds: false }) as string;
    // Prefix internal links and images with BASE_PATH
    if (BASE_PATH) {
      html = html.replace(
        /(href|src)="\/(?!\/)/g,
        `$1="${BASE_PATH}/`,
      );
    }
    return html;
  } catch {
    return md;
  }
}

function extractFirstParagraph(body: string): string {
  const text = body.trim();
  const match = text.match(/^(.+?)(?:\n\n|\n#{1,6}\s|\n---|\n*$)/s);
  if (!match) return "";
  let para = match[1].trim();
  para = para.replace(/\[([^\]]*)\]\([^)]*\)/g, "$1");
  para = para.replace(/`([^`]+)`/g, "$1");
  para = para.replace(/(\*{1,3}|_{1,3})(.+?)\1/g, "$2");
  return para;
}

function toDateString(value: unknown): string {
  if (value instanceof Date) return value.toISOString().split("T")[0];
  return (value as string) ?? "1970-01-01";
}

function extractFrontMatter(text: string) {
  if (!text.startsWith("---")) return { attrs: {}, body: text };
  const end = text.indexOf("---", 3);
  if (end === -1) return { attrs: {}, body: text };
  const fm = text.slice(3, end).trim();
  const body = text.slice(end + 3).trim();
  const attrs = parse(fm) as Record<string, unknown>;
  return { attrs, body };
}

async function* walk(dir: string): AsyncGenerator<string> {
  for await (const entry of Deno.readDir(dir)) {
    const p = join(dir, entry.name);
    if (entry.isDirectory) {
      yield* walk(p);
    } else if (entry.isFile && extname(entry.name) === ".md") {
      yield p;
    }
  }
}

async function getPosts(postsDir = "posts"): Promise<Post[]> {
  const posts: Post[] = [];
  try {
    for await (const path of walk(postsDir)) {
      const raw = await Deno.readTextFile(path);
      const { attrs, body } = extractFrontMatter(raw);
      const title = (attrs?.title as string) ?? "Untitled";
      const publish_date = toDateString(attrs?.publish_date);
      const tags = (attrs?.tags as string[]) ?? [];
      const rel = path.replace(/^posts[\/]/, "").replace(/\\/g, "/");
      const url = "/" + rel.replace(/\.md$/, "");
      const slug = url.split("/").pop() || "";
      posts.push({
        slug,
        title,
        publish_date,
        tags,
        description: extractFirstParagraph(body),
        content: body,
        url,
      });
    }
  } catch (e) {
    console.error("getPosts error:", e);
  }
  posts.sort((a, b) => (a.publish_date < b.publish_date ? 1 : -1));
  return posts;
}

// ── HTML templates ─────────────────────────────────────────────────────────

const MARKDOWN_CSS = `
  .markdown-body {
    font-family: 'Maitree', sans-serif;
    line-height: 1.8;
    color: var(--text, #333);
  }
  .markdown-body h1, .markdown-body h2, .markdown-body h3,
  .markdown-body h4, .markdown-body h5, .markdown-body h6 {
    font-family: 'Prompt', sans-serif;
    line-height: 1.3;
    color: var(--heading, #111);
    margin-top: 1.5em;
    margin-bottom: 0.5em;
  }
  .markdown-body h1 { font-size: 2rem; }
  .markdown-body h2 { font-size: 1.5rem; }
  .markdown-body h3 { font-size: 1.25rem; }
  .markdown-body p { margin-bottom: 1rem; }
  .markdown-body a { color: var(--link, #2563eb); }
  .markdown-body a:hover { text-decoration: underline; }
  .markdown-body code {
    font-family: 'Source Code Pro', monospace;
    font-size: 0.9em;
    background: var(--code-bg, #f1f5f9);
    padding: 0.15em 0.4em;
    border-radius: 4px;
  }
  .markdown-body pre {
    background: #1e293b;
    color: #e2e8f0;
    padding: 1rem;
    border-radius: 8px;
    overflow-x: auto;
    margin-bottom: 1rem;
  }
  .markdown-body pre code {
    background: none;
    padding: 0;
    color: inherit;
  }
  .markdown-body ul, .markdown-body ol {
    padding-left: 1.5rem;
    margin-bottom: 1rem;
  }
  .markdown-body li { margin-bottom: 0.25rem; }
  .markdown-body blockquote {
    border-left: 4px solid var(--blockquote-border, #2563eb);
    padding-left: 1rem;
    margin: 1rem 0;
    color: var(--blockquote-text, #475569);
    font-style: italic;
  }
  .markdown-body table {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 1rem;
  }
  .markdown-body th, .markdown-body td {
    border: 1px solid var(--table-border, #e2e8f0);
    padding: 0.5rem 0.75rem;
    text-align: left;
  }
  .markdown-body th {
    background: var(--table-header-bg, #f8fafc);
    font-weight: 700;
  }
  .markdown-body img { max-width: 100%; }
  .markdown-body hr {
    border: none;
    border-top: 1px solid var(--border, #e2e8f0);
    margin: 2rem 0;
  }
`;

function layout(title: string, body: string, extraHead = ""): string {
  return `<!DOCTYPE html>
<html lang="th">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <meta name="color-scheme" content="light dark" />
  <title>${title} — Nomkhonwaan</title>
  <meta name="description" content="Trust me I'm Petdo" />
  <link rel="icon" href="${BASE_PATH}/favicon.ico" />
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
  <link href="https://fonts.googleapis.com/css2?family=Maitree:wght@400;700&family=Prompt:wght@700&family=Source+Code+Pro&display=swap" rel="stylesheet" />
  <link rel="stylesheet" href="${BASE_PATH}/styles.css" />
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css" integrity="sha384-n8MVd4RsNIU0tAv4ct0nTaAbDJwPJzDEaqSD1odI+WdtXRGWt2kTvGFasHpSy3SV" crossorigin="anonymous" />
  ${extraHead}
  <!-- Google Analytics -->
  <script async src="https://www.googletagmanager.com/gtag/js?id=G-ER347CPNY4"></script>
  <script>
    window.dataLayer = window.dataLayer || [];
    function gtag(){dataLayer.push(arguments);}
    gtag('js', new Date());
    gtag('config', 'G-ER347CPNY4');
  </script>
</head>
<body>
  ${body}
</body>
</html>`;
}

function renderIndex(posts: Post[]): string {
  const items = posts.map((p) => `
      <li class="post-item">
        <div class="post-title">
          <a href="${BASE_PATH}${p.url}">${p.title}</a>
        </div>
        <div class="post-date">${p.publish_date}</div>
        ${p.description ? `<div class="post-desc">${p.description}</div>` : ""}
        ${p.tags.length > 0
          ? `<div class="post-tags">${
              p.tags.map((t) => `<span class="post-tag">${t}</span>`).join("")
            }</div>`
          : ""}
      </li>`).join("\n");

  return layout("Nomkhonwaan", `
  <main>
    <section class="profile">
      <img src="${BASE_PATH}/avatar.png" alt="Natcha Luangaroonchai" class="profile-avatar" />
      <h1 class="profile-name">Nomkhonwaan</h1>
      <p class="profile-desc">Trust me I'm Petdo</p>
      <div class="profile-links">
        <a href="mailto:me@nomkhonwaan.com">Email</a>
        <a href="https://github.com/nomkhonwaan">GitHub</a>
        <a href="https://linkedin.com/in/nomkhonwaan">LinkedIn</a>
      </div>
    </section>

    <section>
      <ul class="post-list">${items}</ul>
    </section>

    <footer class="footer">
      &copy; ${new Date().getFullYear()} Natcha Luangaroonchai
    </footer>
  </main>`);
}

function renderPost(post: Post): string {
  const body = renderMarkdown(post.content);
  const tags = post.tags.length > 0
    ? `<div class="post-tags">${
        post.tags.map((t) => `<span class="post-tag">${t}</span>`).join("")
      }</div>`
    : "";

  return layout(post.title, `
  <main>
    <a href="${BASE_PATH}/" class="back-link">&larr; Back to home</a>

    <article>
      <header class="post-header">
        <h1>${post.title}</h1>
        <div class="post-meta">
          <time>${post.publish_date}</time>
          ${tags}
        </div>
      </header>

      <div class="post-content markdown-body">${body}</div>
    </article>

    <footer class="footer">
      &copy; ${new Date().getFullYear()} Natcha Luangaroonchai
    </footer>
  </main>`, `<style>${MARKDOWN_CSS}</style>`);
}

function render404(): string {
  return layout("404 — Page not found", `
  <main class="not-found">
    <h1>404</h1>
    <p>The page you were looking for doesn't exist.</p>
    <a href="${BASE_PATH}/">Go back home</a>
  </main>`);
}

// ── Main ───────────────────────────────────────────────────────────────────

const OUT_DIR = "docs";

async function main() {
  console.log("📦 Building static site...\n");

  // Build index page
  const posts = await getPosts();
  const indexHtml = renderIndex(posts);
  await ensureDir(OUT_DIR);
  await Deno.writeTextFile(join(OUT_DIR, "index.html"), indexHtml);
  console.log(`  ✓ index.html (${posts.length} posts)`);

  // Build post pages
  for (const post of posts) {
    const html = renderPost(post);
    // post.url is like /2016/1/28/tdd-kata-2-the-bowling-game
    const outPath = join(OUT_DIR, post.url, "index.html");
    await ensureDir(dirname(outPath));
    await Deno.writeTextFile(outPath, html);
    console.log(`  ✓ ${post.url}/index.html`);
  }

  // Build 404 page
  await Deno.writeTextFile(join(OUT_DIR, "404.html"), render404());
  console.log("  ✓ 404.html");

  // Copy static assets
  await copy("static", OUT_DIR, { overwrite: true });
  // Copy root favicon.ico (the static/ one is a placeholder)
  try {
    await Deno.copyFile("favicon.ico", join(OUT_DIR, "favicon.ico"));
  } catch {
    // ignore if favicon doesn't exist at root
  }
  console.log("  ✓ static/ → docs/");

  console.log("\n✅ Build complete! Output in docs/");
  console.log("   Deploy docs/ to GitHub Pages.");
}

if (import.meta.main) {
  await main();
}