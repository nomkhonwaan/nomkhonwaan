import { extname, join } from "std/path/mod.ts";
import { parse } from "std/yaml/parse.ts";

export interface Post {
  slug: string;
  title: string;
  publish_date: string;
  tags?: string[];
  cover_image?: string;
  description: string;
  content: string;
  url: string;
}

export interface Pagination {
  page: number;
  totalPages: number;
  total: number;
}

export function extractFirstParagraph(body: string): string {
  // Get the first paragraph: text before the first heading, blank line, or horizontal rule
  const text = body.trim();
  // Split into paragraphs separated by blank lines
  const paragraphs = text.split(/\n\n+/);
  for (const para of paragraphs) {
    let cleaned = para.trim();
    // Strip markdown images: ![alt](url)
    cleaned = cleaned.replace(/!\[([^\]]*)\]\([^)]*\)/g, "");
    // Strip markdown links: [text](url) -> text
    cleaned = cleaned.replace(/\[([^\]]*)\]\([^)]*\)/g, "$1");
    // Strip inline code backticks
    cleaned = cleaned.replace(/`([^`]+)`/g, "$1");
    // Strip bold/italic markers
    cleaned = cleaned.replace(/(\*{1,3}|_{1,3})(.+?)\1/g, "$2");
    cleaned = cleaned.trim();
    if (cleaned) return cleaned;
  }
  return "";
}

export function extractFirstImage(body: string): string | undefined {
  const match = body.match(/!\[([^\]]*)\]\(([^)]+)\)/);
  return match ? match[2] : undefined;
}

function toDateString(value: unknown): string {
  if (value instanceof Date) return value.toISOString().split("T")[0];
  return (value as string) ?? "1970-01-01";
}

async function readFileText(path: string) {
  return await Deno.readTextFile(path);
}

export function extractFrontMatter(text: string) {
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

export async function getPosts(postsDir = "posts") {
  const posts: Post[] = [];
  try {
    for await (const path of walk(postsDir)) {
      const raw = await readFileText(path);
      const { attrs, body } = extractFrontMatter(raw);
      const title = (attrs?.title as string) ?? "Untitled";
      const publish_date = toDateString(attrs?.publish_date);
      const tags = (attrs?.tags as string[]) ?? [];
      // Build URL from path: posts/2022/8/18/slug.md -> /2022/8/18/slug
      const rel = path.replace(/^posts[\/]/, "").replace(/\\/g, "/");
      const url = "/" + rel.replace(/\.md$/, "");
      const slug = url.split("/").pop() || "";
      posts.push({ slug, title, publish_date, tags, cover_image: extractFirstImage(body), description: extractFirstParagraph(body), content: body, url });
    }
  } catch (e) {
    console.error("getPosts error:", e);
  }
  posts.sort((a, b) => (a.publish_date < b.publish_date ? 1 : -1));
  return posts;
}

const POSTS_PER_PAGE = 10;

export async function getPostsPaginated(
  page: number,
  postsDir = "posts",
): Promise<{ posts: Post[]; pagination: Pagination }> {
  const all = await getPosts(postsDir);
  const total = all.length;
  const totalPages = Math.max(1, Math.ceil(total / POSTS_PER_PAGE));
  const p = Math.max(1, Math.min(page, totalPages));
  const start = (p - 1) * POSTS_PER_PAGE;
  const posts = all.slice(start, start + POSTS_PER_PAGE);
  return { posts, pagination: { page: p, totalPages, total } };
}

export async function getPostByUrl(url: string) {
  // url expected like /2022/8/18/go-generics
  const path = url.replace(/^\//, "");
  const mdPath = `posts/${path}.md`;
  try {
    const raw = await readFileText(mdPath);
    const { attrs, body } = extractFrontMatter(raw);
    const title = (attrs?.title as string) ?? "Untitled";
    const publish_date = toDateString(attrs?.publish_date);
    const tags = (attrs?.tags as string[]) ?? [];
    const slug = mdPath.split("/").pop()!.replace(/\.md$/, "");
    return { slug, title, publish_date, tags, description: extractFirstParagraph(body), content: body, url } as Post;
  } catch (e) {
    return null;
  }
}
