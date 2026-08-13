import { Handlers } from "$fresh/server.ts";
import { getPostsPaginated } from "../utils/posts.ts";
import type { Post, Pagination } from "../utils/posts.ts";

export const handler: Handlers = {
  async GET(_req, ctx) {
    const { posts, pagination } = await getPostsPaginated(1);
    return ctx.render({ posts, pagination });
  },
};

export default function Home(
  { data }: { data: { posts: Post[]; pagination: Pagination } },
) {
  const { posts, pagination } = data;
  const { page, totalPages } = pagination;
  const prevUrl = page > 2 ? `/page/${page - 1}` : page > 1 ? "/" : null;
  const nextUrl = page < totalPages ? `/page/${page + 1}` : null;
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
                  {p.tags.map((tag) => (
                    <span class="post-tag">{tag}</span>
                  ))}
                </div>
              )}
            </li>
          ))}
        </ul>

        {totalPages > 1 && (
          <nav class="pagination">
            {prevUrl ? (
              <a href={prevUrl} class="pagination-link">&larr; หน้าก่อน</a>
            ) : (
              <span class="pagination-link disabled">&larr; หน้าก่อน</span>
            )}
            <span class="pagination-info">
              หน้า {page} จาก {totalPages}
            </span>
            {nextUrl ? (
              <a href={nextUrl} class="pagination-link">หน้าถัดไป &rarr;</a>
            ) : (
              <span class="pagination-link disabled">หน้าถัดไป &rarr;</span>
            )}
          </nav>
        )}
      </section>

      <footer class="footer">
        &copy; {new Date().getFullYear()} Natcha Luangaroonchai
      </footer>
    </main>
  );
}
