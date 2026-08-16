import { Handlers } from "$fresh/server.ts";
import { getPostsByTag } from "../../../utils/posts.ts";
import type { Post } from "../../../utils/posts.ts";

export const handler: Handlers = {
  async GET(_req, ctx) {
    const tag = ctx.params.tag ?? "";
    const posts = await getPostsByTag(tag);
    if (posts.length === 0) {
      return ctx.renderNotFound();
    }
    return ctx.render({ tag, posts });
   },
};

export default function TagPage({
  data,
}: {
  data: { tag: string; posts: Post[] };
}) {
  const { tag, posts } = data;
  return (
    <main>
      <section class="profile">
        <img src="/avatar.png" alt="Natcha Luangaroonchai" class="profile-avatar" />
        <h1 class="profile-name">Nomkhonwaan</h1>
        <p class="profile-desc">Trust me I'm Petdo</p>
        <div class="profile-links">
          <a href="mailto:me@nomkhonwaan.com">Email</a>
          <a href="https://github.com/nomkhonwaan">GitHub</a>
          <a href="https://linkedin.com/in/nomkhonwaan">LinkedIn</a>
        </div>
      </section>

      <section>
        <div class="tag-heading">
          <a href="/" class="back-link">&larr; Back to home</a>
           <h1 class="tag-title">{tag}</h1>
           <p class="tag-count">{posts.length} post{posts.length === 1 ? "" : "s"}</p>
        </div>

        <ul class="post-list">
          {posts.map((p) => (
            <li class="post-item">
              {p.cover_image && (
                <a href={p.url} class="post-cover-link">
                  <img src={p.cover_image} alt="" class="post-cover" />
                </a>
              )}
              <div class="post-title">
                <a href={p.url}>{p.title}</a>
              </div>
              <div class="post-date">{p.publish_date}</div>
              {p.description && (
                <div class="post-desc">{p.description}</div>
              )}
              {p.tags && p.tags.length > 0 && (
                <div class="post-tags">
                  {p.tags.map((t) => (
                    <a href={`/tag/${encodeURIComponent(t)}`} class="post-tag">
                      #{t}
                    </a>
                  ))}
                </div>
              )}
            </li>
          ))}
        </ul>
      </section>

      <footer class="footer">
        &copy; {new Date().getFullYear()} Natcha Luangaroonchai
      </footer>
    </main>
  );
}
