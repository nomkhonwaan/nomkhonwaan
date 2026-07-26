import { extname, join } from "https://deno.land/std@0.200.0/path/mod.ts";
import { parse } from "https://deno.land/std@0.200.0/encoding/yaml.ts";

export interface Post {
  slug: string;
  title: string;
  publish_date: string;
  tags?: string[];
  content: string;
  url: string;
}

async function readFileText(path: string) {
  return await Deno.readTextFile(path);
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

export async function getPosts(postsDir = "posts") {
  const posts: Post[] = [];
  try {
    for await (const path of walk(postsDir)) {
      const raw = await readFileText(path);
      const { attrs, body } = extractFrontMatter(raw);
      const title = (attrs?.title as string) ?? "Untitled";
      const publish_date = (attrs?.publish_date as string) ?? "1970-01-01";
      const tags = (attrs?.tags as string[]) ?? [];
      // Build URL from path: posts/2022/8/18/slug.md -> /posts/2022/8/18/slug
      const rel = path.replace(/^posts[\/]/, "").replace(/\\/g, "/");
      const url = "/posts/" + rel.replace(/\.md$/, "");
      const slug = url.split("/").pop() || "";
      posts.push({ slug, title, publish_date, tags, content: body, url });
    }
  } catch (e) {
    console.error("getPosts error:", e);
  }
  posts.sort((a, b) => (a.publish_date < b.publish_date ? 1 : -1));
  return posts;
}

export async function getPostByUrl(url: string) {
  // url expected like /posts/2022/8/18/go-generics
  const path = url.replace(/^\//, "");
  const mdPath = `${path}.md`;
  try {
    const raw = await readFileText(mdPath);
    const { attrs, body } = extractFrontMatter(raw);
    const title = (attrs?.title as string) ?? "Untitled";
    const publish_date = (attrs?.publish_date as string) ?? "1970-01-01";
    const tags = (attrs?.tags as string[]) ?? [];
    const slug = mdPath.split("/").pop()!.replace(/\.md$/, "");
    return { slug, title, publish_date, tags, content: body, url } as Post;
  } catch (e) {
    return null;
  }
}
