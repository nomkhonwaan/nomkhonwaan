import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { getPostByUrl } from "../../../../utils/posts.ts";
import { renderMarkdown, CSS } from "../../../../utils/markdown.ts";
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
  const body = renderMarkdown(post.content);

  return (
    <main>
      <Head>
        <title>{post.title} — Nomkhonwaan</title>
        <style dangerouslySetInnerHTML={{ __html: CSS }} />
      </Head>

      <a href="/" class="back-link">&larr; Back to home</a>

      <article>
        <header class="post-header">
          <h1>{post.title}</h1>
          <div class="post-meta">
            <time>{post.publish_date}</time>
            {post.tags && post.tags.length > 0 && (
              <span>
                {" "}&middot;{" "}
                {post.tags.map((tag) => (
                  <span class="post-tag">{tag}</span>
                ))}
              </span>
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
