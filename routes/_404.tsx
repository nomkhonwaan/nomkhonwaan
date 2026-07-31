import { Head } from "$fresh/runtime.ts";

export default function Error404() {
  return (
    <main class="not-found">
      <Head>
        <title>404 — Page not found</title>
      </Head>
      <h1>404</h1>
      <p>The page you were looking for doesn't exist.</p>
      <a href="/">Go back home</a>
    </main>
  );
}