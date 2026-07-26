import { marked } from "https://esm.sh/marked@5.1.1";

export function renderMarkdown(md: string) {
  try {
    return marked.parse(md);
  } catch (e) {
    console.error("renderMarkdown error:", e);
    return md;
  }
}
