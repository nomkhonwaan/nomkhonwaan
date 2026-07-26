import { h } from "preact";
import { Handlers } from "fresh/server.ts";
import { getPostByUrl } from "../../../../utils/posts.ts";
import { renderMarkdown } from "../../../../utils/markdown.ts";

export const handler: Handlers = {
  async GET(req, ctx) {
    const { year, month, day, slug } = ctx.params as Record<string, string>;
    const url = `/posts/${year}/${month}/${day}/${slug}`;
    const post = await getPostByUrl(url);
    if (!post) return ctx.renderNotFound();
    return ctx.render({ post });
  },
};

export default function PostPage({ data }: { data: { post: any } }) {
  const post = data.post;
  return (
    <main>
      <h1>{post.title}</h1>
      <p>{post.publish_date}</p>
      <article dangerouslySetInnerHTML={{ __html: renderMarkdown(post.content) }} />
    </main>
  );
}
