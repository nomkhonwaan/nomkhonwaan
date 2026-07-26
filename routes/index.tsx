import { h } from "preact";
import { Handlers } from "https://deno.land/x/fresh@1.6.0/server.ts";
import { getPosts } from "../utils/posts.ts";
import { renderMarkdown } from "../utils/markdown.ts";

export const handler: Handlers = {
  async GET(_req, ctx) {
    const posts = await getPosts();
    return ctx.render({ posts });
  },
};

export default function Home({ data }: { data: { posts: any[] } }) {
  return (
    <main>
      <h1>Nomkhonwaan</h1>
      <p>Trust me I'm Petdo</p>
      <ul>
        {data.posts.map((p) => (
          <li>
            <a href={p.url}>{p.title}</a> — <small>{p.publish_date}</small>
          </li>
        ))}
      </ul>
    </main>
  );
}
