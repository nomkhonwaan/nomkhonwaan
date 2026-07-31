import { Handlers } from "$fresh/server.ts";
import { getPosts } from "../utils/posts.ts";
import type { Post } from "../utils/posts.ts";

export const handler: Handlers = {
  async GET(_req, ctx) {
    const posts = await getPosts();
    return ctx.render({ posts });
  },
};

export default function Home({ data }: { data: { posts: Post[] } }) {
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
        <ul class="post-list">
          {data.posts.map((p) => (
            <li class="post-item">
              <div class="post-title">
                <a href={p.url}>{p.title}</a>
              </div>
              <div class="post-date">{p.publish_date}</div>
              {p.tags && p.tags.length > 0 && (
                <div class="post-tags">
                  {p.tags.map((tag) => (
                    <span class="post-tag">{tag}</span>
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
