import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { getPostByUrl } from "../../../../utils/posts.ts";
import { renderMarkdown, resolveMarkdownImages, CSS } from "../../../../utils/markdown.ts";
import type { Post } from "../../../../utils/posts.ts";

export const handler: Handlers = {
  async GET(_req, ctx) {
    const { year, month, day, slug } = ctx.params;
    const url = `/${year}/${month}/${day}/${slug}`;
    const post = await getPostByUrl(url);
    if (!post) return ctx.renderNotFound();
    return ctx.render({ post });
  },
};

export default function PostPage({ data }: PageProps<{ post: Post }>) {
  const post = data.post;
  const body = renderMarkdown(resolveMarkdownImages(post.content, post.url));

  return (
    <main>
      <Head>
        <title>{post.title} — Nomkhonwaan</title>
        <style dangerouslySetInnerHTML={{ __html: CSS }} />
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css" integrity="sha384-n8MVd4RsNIU0tAv4ct0nTaAbDJwPJzDEaqSD1odI+WdtXRGWt2kTvGFasHpSy3SV" crossorigin="anonymous" />
      </Head>

      <a href="/" class="back-link">&larr; Back to home</a>

      <article>
        <header class="post-header">
          <h1>{post.title}</h1>
          <div class="post-meta">
            <time>{post.publish_date}</time>
            {post.tags && post.tags.length > 0 && (
              <div class="post-tags">
                {post.tags.map((tag) => (
                  <span class="post-tag">{tag}</span>
                ))}
              </div>
            )}
          </div>
        </header>

        <div
          class="post-content markdown-body"
          dangerouslySetInnerHTML={{ __html: body }}
        />
      </article>

      <footer class="footer">
        &copy; {new Date().getFullYear()} Natcha Luangaroonchai
      </footer>
    </main>
  );
}
